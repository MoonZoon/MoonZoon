use crate::*;
use std::sync::{Arc, Mutex};

// ------ AnimationLoop Data ------

struct Data {
    id: Option<i32>,
    timestamp: f64,
    callback: Option<SendWrapper<Closure<dyn FnMut(f64)>>>,
}

// ------ AnimationLoop ------

#[derive(Clone)]
pub struct AnimationLoop {
    data: Arc<Mutex<Data>>,
}

impl AnimationLoop {
    pub fn new(mut on_frame: impl FnMut(Duration) + 'static) -> Self {
        let timestamp = window().performance().unwrap_throw().now();
        let data: Arc<Mutex<Data>> = Arc::new(Mutex::new(Data {
            id: None,
            timestamp,
            callback: None,
        }));

        fn schedule(callback: &Closure<dyn FnMut(f64)>) -> i32 {
            window()
                .request_animation_frame(callback.as_ref().unchecked_ref())
                .unwrap_throw()
        }

        let callback = {
            let data = data.clone();

            Closure::new(move |timestamp| {
                let previous_timestamp = {
                    let mut data = data.lock().unwrap_throw();
                    data.id = Some(schedule(data.callback.as_ref().unwrap_throw()));
                    let previous_timestamp = data.timestamp;
                    data.timestamp = timestamp;
                    previous_timestamp
                };
                // @TODO make sure `timestamp` is in ms
                on_frame(Duration::milliseconds(
                    (timestamp - previous_timestamp) as i64,
                ));
            })
        };

        *data.lock().unwrap_throw() = Data {
            id: Some(schedule(&callback)),
            timestamp,
            callback: Some(SendWrapper::new(callback)),
        };

        Self { data }
    }
}

impl Drop for AnimationLoop {
    fn drop(&mut self) {
        let mut data = self.data.lock().unwrap_throw();
        let _callback = data.callback.take();
        window()
            .cancel_animation_frame(data.id.unwrap_throw())
            .unwrap_throw();
    }
}
