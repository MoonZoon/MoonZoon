use crate::web_workers::*;
use std::{cell::RefCell, rc::Rc};
use zoon::*;

static DEFAULT_MARKDOWN: &str = r#"This content is *rendered* by a **web worker**"#;

pub static STORE: Lazy<Store> = Lazy::new(|| {
    create_web_workers_and_triggers();
    Store::new()
});

#[derive(Educe)]
#[educe(Default(new))]
pub struct Store {
    #[educe(Default(expression = Mutable::new(DEFAULT_MARKDOWN.to_owned())))]
    pub markdown: Mutable<String>,
    pub html: Mutable<String>,
    pub prime: Mutable<u64>,
    pub is_generating_primes: Mutable<bool>,
}

fn create_web_workers_and_triggers() {
    let markdown_bridge = MarkdownWebWorker::start();

    let (prime_bridge_sink, mut prime_bridge_stream) = PrimeWebWorker::start().split();
    let prime_bridge_sink = Rc::new(RefCell::new(prime_bridge_sink));

    // set `html` on `markdown` change
    Task::start(async move {
        STORE
            .markdown
            .signal_cloned()
            .for_each(move |markdown| {
                let mut markdown_web_worker_bridge = markdown_bridge.fork();
                async move {
                    let html = markdown_web_worker_bridge.run(markdown).await;
                    STORE.html.set(html);
                }
            })
            .await
    });

    // send `Command` on `is_generating_primes` change
    Task::start(async move {
        STORE
            .is_generating_primes
            .signal()
            .for_each(move |is_generating_primes| {
                clone!((prime_bridge_sink) async move {
                    prime_bridge_sink.borrow_mut().send(
                        if is_generating_primes { Command::Start } else { Command::Stop }
                    ).await.unwrap_throw();
                })
            })
            .await
    });

    // set `prime` on prime receive
    Task::start(async move {
        while let Some(prime) = prime_bridge_stream.next().await {
            STORE.prime.set(prime);
        }
    });
}
