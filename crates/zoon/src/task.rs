use std::future::Future;
use wasm_bindgen_futures::spawn_local;

pub struct Task<F> {
    future: F
}

impl<F: Future<Output = ()> + 'static> Task<F> {
    pub fn new(future: F) -> Self {
        Self {
            future
        }
    }

    pub fn perform(self) {
        spawn_local(self.future)
    }
}
