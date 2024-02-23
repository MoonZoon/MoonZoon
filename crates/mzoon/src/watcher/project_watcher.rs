use anyhow::{Context, Error};
use fehler::throws;
use notify_debouncer_mini::{
    new_debouncer as new_notify_debouncer,
    notify::{RecommendedWatcher, RecursiveMode},
    DebounceEventResult, Debouncer,
};
use std::path::Path;
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::time::Duration;

pub struct ProjectWatcher {
    #[allow(dead_code)]
    debounced_watcher: Debouncer<RecommendedWatcher>,
}

impl ProjectWatcher {
    #[throws]
    pub fn start(paths: &[String], debounce_time: Duration) -> (Self, UnboundedReceiver<()>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let debounced_watcher = start_debounced_recommended_watcher(sender, paths, debounce_time)?;
        let this = ProjectWatcher { debounced_watcher };
        (this, receiver)
    }
}

#[throws]
fn start_debounced_recommended_watcher(
    sender: UnboundedSender<()>,
    paths: &[String],
    debounce_time: Duration,
) -> Debouncer<RecommendedWatcher> {
    let mut debounced_watcher =
        new_notify_debouncer(debounce_time, move |event| on_change(event, &sender))
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
