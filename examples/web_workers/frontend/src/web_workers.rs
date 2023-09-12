pub use gloo_worker::{
    oneshot::{oneshot, OneshotBridge},
    reactor::{reactor, ReactorBridge, ReactorScope},
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

impl MarkdownWebWorker {
    pub fn start() -> OneshotBridge<Self> {
        Self::spawner().spawn_with_loader(WebWorkerLoader::new("markdown_web_worker").path())
    }
}

// ------ prime web worker ------

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub enum Command {
    Start,
    Stop,
}

#[reactor]
pub async fn PrimeWebWorker(mut scope: ReactorScope<Command, u64>) {
    while let Some(command) = scope.next().await {
        if command != Command::Start {
            continue;
        }
        for prime in prime_iter::primes::<u64>() {
            select_future! {
                command = scope.next() => {
                    if matches!(command, Some(Command::Stop) | None) {
                        break;
                    }
                }
                _ = Timer::sleep(0).fuse() => {
                    scope.send(prime).await.unwrap()
                },
            }
        }
    }
}

impl PrimeWebWorker {
    pub fn start() -> ReactorBridge<Self> {
        Self::spawner().spawn_with_loader(WebWorkerLoader::new("prime_web_worker").path())
    }
}

// ------ shared memory web worker ------

pub fn spawn_on_web_worker(f: impl FnOnce() + Send + 'static) {
    let worker = web_sys::Worker::new(WebWorkerLoader::new_from_frontend().path()).unwrap_throw();
    // Double-boxing because `dyn FnOnce` is unsized and so `Box<dyn FnOnce()>` is a fat pointer.
    // But `Box<Box<dyn FnOnce()>>` is just a plain pointer, and since wasm has 32-bit pointers,
    // we can cast it to a `u32` and back.
    let ptr = Box::into_raw(Box::new(Box::new(f) as Box<dyn FnOnce()>));
    let msg = js_sys::Array::new();
    msg.push(&wasm_bindgen::module());
    msg.push(&wasm_bindgen::memory());
    msg.push(&JsValue::from(ptr as u32));
    worker.post_message(&msg).unwrap_throw();
}

#[wasm_bindgen]
pub fn worker_entry_point(ptr: u32) {
    let closure = unsafe { Box::from_raw(ptr as *mut Box<dyn FnOnce()>) };
    (*closure)();
}
