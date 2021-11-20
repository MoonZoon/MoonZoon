use crate::*;

// ------ Timer ------

pub struct Timer {
    handle: Option<JsHandle>,
    #[allow(dead_code)]
    on_tick: SendWrapper<Closure<dyn FnMut()>>,
}

impl Timer {
    pub fn new(ms: u32, on_tick: impl FnOnce() + Clone + 'static) -> Self {
        let on_tick = move || (on_tick.clone())();
        let on_tick = Closure::wrap(Box::new(on_tick) as Box<dyn FnMut()>);
        Self {
            handle: Some(JsHandle::Interval(set_interval(&on_tick, ms))),
            on_tick: SendWrapper::new(on_tick),
        }
    }

    pub fn new_immediate(ms: u32, on_tick: impl FnOnce() + Clone + 'static) -> Self {
        on_tick.clone()();
        Self::new(ms, on_tick)
    }

    pub fn once(ms: u32, on_tick: impl FnOnce() + 'static) -> Self {
        let on_tick = Closure::once(on_tick);
        Self {
            handle: Some(JsHandle::Timeout(set_timeout(&on_tick, ms))),
            on_tick: SendWrapper::new(on_tick),
        }
    }

    pub async fn sleep(ms: u32) {
        let (sender, receiver) = oneshot::channel();
        let _timer = Self::once(ms, move || {
            sender
                .send(())
                .expect_throw("`sender` failed in `Timer::sleep`")
        });
        receiver
            .await
            .expect_throw("`receiver` failed in `Timer::sleep`")
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            match handle {
                JsHandle::Interval(handle) => clear_interval(handle),
                JsHandle::Timeout(handle) => clear_timeout(handle),
            }
        }
    }
}

// ------ JsHandle ------

enum JsHandle {
    Interval(f64),
    Timeout(f64),
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setInterval)]
    fn set_interval(callback: &Closure<dyn FnMut()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(token: f64);

    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(callback: &Closure<dyn FnMut()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearTimeout)]
    fn clear_timeout(token: f64);
}
