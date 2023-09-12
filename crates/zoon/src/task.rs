use crate::*;
use futures_util::future::{abortable, AbortHandle};
use wasm_bindgen_futures::spawn_local;

// ------ Task ------

pub struct Task;

impl Task {
    pub fn start(future: impl Future<Output = ()> + 'static) {
        spawn_local(future)
    }

    pub fn start_droppable(future: impl Future<Output = ()> + 'static) -> TaskHandle {
        let (future_handler, future_handle) = abortable(future);
        spawn_local(async {
            let _ = future_handler.await;
        });
        TaskHandle(future_handle)
    }

    // @TODO add `start_blocking_droppable`

    pub fn start_blocking(f: impl FnOnce() + Send + 'static) {
        let worker =
            web_sys::Worker::new(WebWorkerLoader::new_from_frontend().path()).unwrap_throw();
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
}

#[wasm_bindgen]
pub fn worker_entry_point(ptr: u32) {
    let closure = unsafe { Box::from_raw(ptr as *mut Box<dyn FnOnce()>) };
    (*closure)();
}

// ------ TaskHandle ------

#[must_use]
pub struct TaskHandle(AbortHandle);

impl Drop for TaskHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
