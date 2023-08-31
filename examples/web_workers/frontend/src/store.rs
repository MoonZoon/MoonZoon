use crate::workers::*;
use educe::Educe;
use std::{cell::RefCell, rc::Rc};
use zoon::*;

static DEFAULT_MARKDOWN: &str = r#"This content is *rendered* by a **web worker**"#;

#[static_ref]
pub fn store() -> &'static Store {
    create_web_workers_and_triggers();
    Store::new()
}

#[derive(Educe)]
#[educe(Default(new))]
pub struct Store {
    #[educe(Default(expression = "Mutable::new(DEFAULT_MARKDOWN.to_owned())"))]
    pub markdown: Mutable<String>,
    pub html: Mutable<String>,
    pub prime: Mutable<u64>,
    pub is_generating_primes: Mutable<bool>,
}

fn create_web_workers_and_triggers() {
    // The command to build a worker after the `frontend` crate has been built at least once:
    //
    // cargo build --target wasm32-unknown-unknown && ../../wasm-bindgen --target no-modules --out-dir ./pkg ../../../target/wasm32-unknown-unknown/debug/[worker_crate_name].wasm
    //
    // (run it inside the `web_workers/[worker_name]` folder)

    let markdown_bridge =
        MarkdownWebWorker::spawner().spawn("/_api/web_workers/markdown/pkg/markdown.js");

    let (prime_bridge_sink, mut prime_bridge_stream) = PrimeWebWorker::spawner()
        .spawn("/_api/web_workers/prime/pkg/prime.js")
        .split();
    let prime_bridge_sink = Rc::new(RefCell::new(prime_bridge_sink));

    // set `html` on `markdown` change
    Task::start(async move {
        store()
            .markdown
            .signal_cloned()
            .for_each(move |markdown| {
                let mut markdown_web_worker_bridge = markdown_bridge.fork();
                async move {
                    let html = markdown_web_worker_bridge.run(markdown).await;
                    store().html.set(html);
                }
            })
            .await
    });

    // send `ControlSignal` on `is_generating_primes` change
    Task::start(async move {
        store()
            .is_generating_primes
            .signal()
            .for_each(move |is_generating_primes| clone!((prime_bridge_sink) async move {
                prime_bridge_sink.borrow_mut().send(
                    if is_generating_primes { ControlSignal::Start } else { ControlSignal::Stop }
                ).await.unwrap_throw();
            }))
            .await
    });

    // set `prime` on prime receive
    Task::start(async move {
        while let Some(prime) = prime_bridge_stream.next().await {
            store().prime.set(prime);
        }
    })
}
