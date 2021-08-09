use crate::*;
use futures_util::future::{abortable, AbortHandle};

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
}

// ------ TaskHandle ------

#[must_use]
pub struct TaskHandle(AbortHandle);

impl Drop for TaskHandle {
    fn drop(&mut self) {
        self.0.abort();
    }
}
