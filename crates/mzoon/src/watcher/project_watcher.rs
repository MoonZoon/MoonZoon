use notify::{RecursiveMode, immediate_watcher, Watcher, RecommendedWatcher, Event};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::time::{Duration, sleep};
use anyhow::{Context, Error};
use tokio::{spawn, task::JoinHandle};
use std::sync::Arc;
use fehler::throws;

pub struct ProjectWatcher {
    watcher: RecommendedWatcher,
    debouncer: JoinHandle<()>,
}

impl ProjectWatcher {
    #[throws]
    pub async fn start(paths: &[String], debounce_time: Duration) -> (Self, UnboundedReceiver<()>)  {
        let (sender, receiver) = mpsc::unbounded_channel();
        let watcher = start_immediate_watcher(sender, paths)?;
        let (debounced_sender, debounced_receiver) = mpsc::unbounded_channel();
    
        let this = ProjectWatcher {
            watcher,
            debouncer: spawn(debounced_on_change(debounced_sender, receiver, debounce_time)),
        };
        (this, debounced_receiver)
    }

    #[throws]
    pub async fn stop(self) {
        let watcher = self.watcher;
        drop(watcher);
        self.debouncer.await?;
    }
}

#[throws]
fn start_immediate_watcher(sender: UnboundedSender<()>, paths: &[String]) -> RecommendedWatcher {
    let mut watcher = immediate_watcher(move |event| {
        on_change(event, &sender)
    }).context("Failed to create the watcher")?;

    configure_watcher(&mut watcher).context("Failed to configure the watcher")?;

    for path in paths {
        watcher.watch(path, RecursiveMode::Recursive)
            .with_context(|| format!("Failed to set the watched path: '{}'", path))?;
    }
    watcher
}

fn on_change(event: notify::Result<Event>, sender: &UnboundedSender<()>) {
    if let Err(error) = event {
        return eprintln!("Watcher failed: {:#?}", error);
    }
    if let Err(error) = sender.send(()) {
        return eprintln!("Failed to send with the sender: {:#?}", error);
    }
}

#[throws]
fn configure_watcher(watcher: &mut RecommendedWatcher) {
    watcher.configure(notify::Config::PreciseEvents(false))?;
    watcher.configure(notify::Config::NoticeEvents(false))?;
    watcher.configure(notify::Config::OngoingEvents(None))?;
}

async fn debounced_on_change(
    debounced_sender: UnboundedSender<()>, 
    mut receiver: UnboundedReceiver<()>, 
    debounce_time: Duration
) {
    let mut debounce_task = None::<JoinHandle<()>>;
    let debounced_sender = Arc::new(debounced_sender);

    while receiver.recv().await.is_some() {
        if let Some(debounce_task) = debounce_task {
            debounce_task.abort();
        }
        debounce_task = Some(spawn(
            debounce(Arc::clone(&debounced_sender), debounce_time)
        ));
    }

    if let Some(debounce_task) = debounce_task {
        debounce_task.abort();
    }
}

async fn debounce(debounced_sender: Arc<UnboundedSender<()>>, debounce_time: Duration) {
    sleep(debounce_time).await; 
    if let Err(error) = debounced_sender.send(()) {
        return eprintln!("Failed to send with the debounced sender: {:#?}", error);
    }
}
