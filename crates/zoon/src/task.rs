use crate::*;
use futures_util::future::{abortable, AbortHandle};
#[cfg(feature = "frontend_multithreading")]
use std::pin::Pin;
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

    #[cfg(feature = "frontend_multithreading")]
    pub fn start_blocking<FUT: Future<Output = ()>>(
        mut f: impl FnMut(DedicatedWorkerGlobalScope) -> FUT + Send + 'static,
    ) {
        let f = |scope| Box::pin(f(scope)) as Pin<Box<dyn Future<Output = ()>>>;
        let f = Box::new(f) as BlockingCallback;
        // Double-boxing because `dyn FnMut` is unsized and so `Box<dyn FnMut()>` is a fat pointer.
        // But `Box<Box<dyn FnMut()>>` is just a plain pointer, and since wasm has 32-bit pointers,
        // we can cast it to a `u32` and back.
        let pointer = Box::into_raw(Box::new(f));

        let message = JsValue::from(pointer as u32);
        // @TODO worker pool?
        WORKER.with(|worker| worker.post_message(&message).unwrap_throw());
    }

    // @TODO add `start_blocking_droppable` ; note: properly drop Workers and ObjectUrls
}

// ------ TaskHandle ------

#[must_use]
pub struct TaskHandle(AbortHandle);

impl Drop for TaskHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}

// ------ WORKER ------

#[cfg(feature = "frontend_multithreading")]
type BlockingCallback<'fut, 'f> =
    Box<dyn FnMut(DedicatedWorkerGlobalScope) -> Pin<Box<dyn Future<Output = ()> + 'fut>> + 'f>;

#[cfg(feature = "frontend_multithreading")]
thread_local! {
    static WORKER: web_sys::Worker = {
        let worker = web_sys::Worker::new(&worker_loader_url()).unwrap_throw();

        let message = js_sys::Array::new();
        message.push(&wasm_bindgen::module());
        message.push(&wasm_bindgen::memory());

        worker.post_message(&message).unwrap_throw();
        worker
    };
}

#[cfg(feature = "frontend_multithreading")]
fn worker_loader_url() -> String {
    const FRONTEND_BUILD_ID: &str = env!("FRONTEND_BUILD_ID");
    const CACHE_BUSTING: &str = env!("CACHE_BUSTING");

    let current_href = window().location().href().unwrap_throw();

    let js_url = if CACHE_BUSTING == "true" {
        format!("/_api/pkg/frontend_{FRONTEND_BUILD_ID}.js")
    } else {
        format!("/_api/pkg/frontend.js")
    };
    let js_url = web_sys::Url::new_with_base(&js_url, &current_href)
        .expect_throw("Failed to create URL for Web Worker Javascript")
        .to_string();

    let array: js_sys::Array = js_sys::Array::new();
    array.push(
        &format!(
            r#"
        importScripts("{js_url}");
        self.onmessage = async event => {{
            const [wasm_module, wasm_memory] = event.data;
            const instance_creator = await wasm_bindgen(wasm_module, wasm_memory);
            
            self.onmessage = async event => {{
                const {{ worker_entry_point }} = await instance_creator;
                const callback_pointer_u32 = Number(event.data);
                worker_entry_point(callback_pointer_u32);
            }};
          }}
    "#
        )
        .into(),
    );

    let blob = web_sys::Blob::new_with_str_sequence_and_options(
        &array,
        web_sys::BlobPropertyBag::new().type_("application/javascript"),
    )
    .unwrap_throw();

    web_sys::Url::create_object_url_with_blob(&blob).unwrap_throw()
}

#[cfg(feature = "frontend_multithreading")]
#[wasm_bindgen]
pub fn worker_entry_point(pointer: u32) {
    let mut callback = unsafe { Box::from_raw(pointer as *mut BlockingCallback) };
    let future = (*callback)(js_sys::global().unchecked_into());
    spawn_local(future);
}
