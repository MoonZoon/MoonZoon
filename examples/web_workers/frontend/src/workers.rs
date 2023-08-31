pub use gloo_worker::{
    oneshot::{oneshot, OneshotBridge},
    reactor::{reactor, ReactorScope},
    Spawnable,
};
pub use zoon::*;

// ------ markdown web worker ------

#[oneshot]
pub async fn MarkdownWebWorker(markdown: String) -> String {
    let options = pulldown_cmark::Options::all();
    let parser = pulldown_cmark::Parser::new_ext(&markdown, options);
    let mut html_text = String::new();
    pulldown_cmark::html::push_html(&mut html_text, parser);
    html_text
}

// ------ prime web worker ------

#[derive(PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum ControlSignal {
    Start,
    Stop,
}

#[reactor]
pub async fn PrimeWebWorker(mut scope: ReactorScope<ControlSignal, u64>) {
    while let Some(control_signal) = scope.next().await {
        if control_signal != ControlSignal::Start {
            continue;
        }
        for prime in prime_iter::primes::<u64>() {
            scope.send(prime).await.unwrap();
            if control_signal == ControlSignal::Stop {
                break;
            }
        }
    }
}
