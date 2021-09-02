use crate::*;

// ------ Timer ------

pub struct Timer {
    handle: Option<JsHandle>,
    on_tick: SendWrapper<Closure<dyn Fn()>>,
}

impl Timer {
    pub fn new(ms: u32, on_tick: impl FnOnce() + Clone + 'static) -> Self {
        let mut this = Self::without_handle(on_tick);
        this.handle = Some(JsHandle::Interval(set_interval(&this.on_tick, ms)));
        this
    }

    pub fn once(ms: u32, on_tick: impl FnOnce() + Clone + 'static) -> Self {
        let mut this = Self::without_handle(on_tick);
        this.handle = Some(JsHandle::Timeout(set_timeout(&this.on_tick, ms)));
        this
    }

    fn without_handle(on_tick: impl FnOnce() + Clone + 'static) -> Self {
        let on_tick = move || (on_tick.clone())();
        let on_tick = Closure::wrap(Box::new(on_tick) as Box<dyn Fn()>);
        Self {
            handle: None,
            on_tick: SendWrapper::new(on_tick),
        }
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
    fn set_interval(callback: &Closure<dyn Fn()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(token: f64);

    #[wasm_bindgen(js_name = setTimeout)]
    fn set_timeout(callback: &Closure<dyn Fn()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearTimeout)]
    fn clear_timeout(token: f64);
}
