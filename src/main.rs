use wasm_bindgen::prelude::*;
use moxie::runtime::Runtime;
use std::sync::Mutex;
use std::cell::RefCell;

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(input: &str);
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
}

fn runtime_run_once() {
    RUNTIME.with(|runtime| {
        runtime.borrow_mut().run_once(root);
    });
}

fn main() {
    log!("main");
    console_error_panic_hook::set_once();
    runtime_run_once();
}

#[topo::nested]
fn root() {
    log!("root");
}

