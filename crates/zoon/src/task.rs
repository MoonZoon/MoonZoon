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

    /// https://javascript.info/event-loop
    pub async fn next_micro_tick() {
        let (sender, receiver) = oneshot::channel();
        Self::start(async {
            sender
                .send(())
                .expect_throw("`sender` failed in `Task::next_tick`")
        });
        receiver
            .await
            .expect_throw("`receiver` failed in `Task::next_tick`")
    }

    /// https://javascript.info/event-loop
    pub async fn next_macro_tick() {
        // @TODO make it more efficient, see
        // https://developer.mozilla.org/en-US/docs/Web/API/Window/setImmediate
        Timer::sleep(0).await
    }

    /// NOTE: Add `frontend_multithreading = true` to your MoonZoon.toml file to enable it
    #[cfg(feature = "frontend_multithreading")]
    pub fn start_blocking<Fut: Future<Output = ()>>(
        f: impl FnOnce(DedicatedWorkerGlobalScope) -> Fut + Send + 'static,
    ) {
        let f = |scope| Box::pin(f(scope)) as Pin<Box<dyn Future<Output = ()>>>;
        let f = Box::new(f) as BlockingCallback;
        // Double-boxing because `dyn FnOnce` is unsized and so `Box<dyn FnOnce()>` is a fat pointer.
        // But `Box<Box<dyn FnOnce()>>` is just a plain pointer, and since wasm has 32-bit pointers,
        // we can cast it to a `u32` and back.
        let pointer = Box::into_raw(Box::new(f));

        let message = JsValue::from(pointer as u32);
        // @TODO worker pool
        // Inspiration: https://github.com/rustwasm/wasm-bindgen/blob/main/examples/raytrace-parallel/src/pool.rs
        WORKER.with(|worker| worker.post_message(&message).unwrap_throw());
    }

    // @TODO add `start_blocking_droppable` (properly drop Workers and ObjectUrls)

    /// NOTE: Add `frontend_multithreading = true` to your MoonZoon.toml file to enable it
    #[cfg(feature = "frontend_multithreading")]
    pub fn start_blocking_with_tasks<FutI, FutB, FutO, IBMsg, BOMsg>(
        input_task: impl FnOnce(Box<dyn FnBoxClone<IBMsg>>) -> FutI + 'static,
        blocking_task: impl FnOnce(
                UnboundedReceiver<IBMsg>,
                DedicatedWorkerGlobalScope,
                Box<dyn FnBoxClone<BOMsg>>,
            ) -> FutB
            + Send
            + 'static,
        output_task: impl FnOnce(UnboundedReceiver<BOMsg>) -> FutO + 'static,
    ) where
        FutI: Future<Output = ()> + 'static,
        FutB: Future<Output = ()>,
        FutO: Future<Output = ()> + 'static,
        IBMsg: Send + 'static,
        BOMsg: Send + 'static,
    {
        let (input_to_blocking_sender, input_to_blocking_receiver) = mpsc::unbounded();
        let (blocking_to_output_sender, blocking_to_output_receiver) = mpsc::unbounded();
        Task::start(input_task(Box::new(move |to_blocking| {
            input_to_blocking_sender
                .unbounded_send(to_blocking)
                .unwrap_throw()
        })));
        Task::start_blocking(move |scope| {
            blocking_task(
                input_to_blocking_receiver,
                scope,
                Box::new(move |to_output| {
                    blocking_to_output_sender
                        .unbounded_send(to_output)
                        .unwrap_throw()
                }),
            )
        });
        Task::start(output_task(blocking_to_output_receiver));
    }

    // @TODO add `start_blocking_with_tasks_droppable`
    // (properly drop everything, graceful channel shutdown?)

    /// NOTE: Add `frontend_multithreading = true` to your MoonZoon.toml file to enable it
    #[cfg(feature = "frontend_multithreading")]
    pub fn start_blocking_with_output_task<FutB, FutO, BOMsg>(
        blocking_task: impl FnOnce(DedicatedWorkerGlobalScope, Box<dyn FnBoxClone<BOMsg>>) -> FutB
            + Send
            + 'static,
        output_task: impl FnOnce(UnboundedReceiver<BOMsg>) -> FutO + 'static,
    ) where
        FutB: Future<Output = ()>,
        FutO: Future<Output = ()> + 'static,
        BOMsg: Send + 'static,
    {
        let (blocking_to_output_sender, blocking_to_output_receiver) = mpsc::unbounded();
        Task::start_blocking(move |scope| {
            blocking_task(
                scope,
                Box::new(move |to_output| {
                    blocking_to_output_sender
                        .unbounded_send(to_output)
                        .unwrap_throw()
                }),
            )
        });
        Task::start(output_task(blocking_to_output_receiver));
    }

    // @TODO add `start_blocking_with_output_task_droppable`
    // (properly drop everything, graceful channel shutdown?)
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
    Box<dyn FnOnce(DedicatedWorkerGlobalScope) -> Pin<Box<dyn Future<Output = ()> + 'fut>> + 'f>;

#[cfg(feature = "frontend_multithreading")]
thread_local! {
    static WORKER: web_sys::Worker = {
        let worker = web_sys::Worker::new(&worker_loader_url()).unwrap_throw();

        let message = js_sys::Array::new();
        message.push(&wasm_bindgen::module());
        message.push(&wasm_bindgen::memory());

        worker.post_message(&message).unwrap_or_else(|error| {
            crate::eprintln!("Failed to post the initialization message to the worker. \
            Make sure you set the env var 'FRONTEND_MULTITHREADING=true' \
            before the Moon app start or you set the cross-origin isolation headers \
            'Cross-Origin-Embedder-Policy: require-corp' and 'Cross-Origin-Opener-Policy: same-origin' \
            in your custom backend app. Also the headers are taken into account by the browser \
            only when served over HTTPS or from the 'localhost` domain. \
            Error details: {error:?}");
            panic!("Failed to post the initialization message to the worker");
        });
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
    let callback = unsafe { Box::from_raw(pointer as *mut BlockingCallback) };
    let future = (*callback)(js_sys::global().unchecked_into());
    spawn_local(future);
}
