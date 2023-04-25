use anyhow::{Context, Error};
use fehler::throws;
use notify_debouncer_mini::{notify::{RecommendedWatcher, RecursiveMode}, new_debouncer as new_notify_debouncer, Debouncer, DebounceEventResult};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::time::{sleep, Duration};
use tokio::{spawn, task::JoinHandle};

pub struct ProjectWatcher {
    watcher: Debouncer<RecommendedWatcher>,
    debouncer: JoinHandle<()>,
}

impl ProjectWatcher {
    #[throws]
    pub fn start(paths: &[String], debounce_time: Duration) -> (Self, UnboundedReceiver<()>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let watcher = start_recommended_watcher(sender, paths, debounce_time)?;
        let (debounced_sender, debounced_receiver) = mpsc::unbounded_channel();

        let this = ProjectWatcher {
            watcher,
            debouncer: spawn(debounced_on_change(
                debounced_sender,
                receiver,
                debounce_time,
            )),
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
fn start_recommended_watcher(sender: UnboundedSender<()>, paths: &[String], debounce_time: Duration) -> Debouncer<RecommendedWatcher> {
    let mut debounced_watcher = new_notify_debouncer(debounce_time, None, move |event| on_change(event, &sender))
        .context("Failed to create the watcher")?;

    for path in paths {
        debounced_watcher
            .watcher()
            .watch(Path::new(path), RecursiveMode::Recursive)
            .with_context(|| format!("Failed to set the watched path: '{}'", path))?;
    }
    debounced_watcher
}

fn on_change(event: DebounceEventResult, sender: &UnboundedSender<()>) {
    if let Err(errors) = event {
        return eprintln!("Watcher failed: {:?}", errors);
    }
    if let Err(error) = sender.send(()) {
        return eprintln!("Failed to send with the sender: {:?}", error);
    }
}

async fn debounced_on_change(
    debounced_sender: UnboundedSender<()>,
    mut receiver: UnboundedReceiver<()>,
    debounce_time: Duration,
) {
    let mut debounce_task = None::<JoinHandle<()>>;
    let debounced_sender = Arc::new(debounced_sender);

    while receiver.recv().await.is_some() {
        if let Some(debounce_task) = debounce_task {
            debounce_task.abort();
        }
        debounce_task = Some(spawn(debounce(
            Arc::clone(&debounced_sender),
            debounce_time,
        )));
    }

    if let Some(debounce_task) = debounce_task {
        debounce_task.abort();
    }
}

async fn debounce(debounced_sender: Arc<UnboundedSender<()>>, debounce_time: Duration) {
    sleep(debounce_time).await;
    if let Err(error) = debounced_sender.send(()) {
        return eprintln!("Failed to send with the debounced sender: {:?}", error);
    }
}
