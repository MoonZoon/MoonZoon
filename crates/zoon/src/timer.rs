use crate::*;

pub struct Timer {
    interval_handle: f64,
    _on_tick: SendWrapper<Closure<dyn Fn()>>,
}

impl Timer {
    pub fn new(ms: u32, on_tick: impl FnOnce() + Clone + 'static) -> Timer {
        let on_tick = move || (on_tick.clone())();
        let on_tick = Closure::wrap(Box::new(on_tick) as Box<dyn Fn()>);
        Self {
            interval_handle: set_interval(&on_tick, ms),
            _on_tick: SendWrapper::new(on_tick),
        }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        clear_interval(self.interval_handle);
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = setInterval)]
    fn set_interval(callback: &Closure<dyn Fn()>, ms: u32) -> f64;

    #[wasm_bindgen(js_name = clearInterval)]
    fn clear_interval(token: f64);
}
