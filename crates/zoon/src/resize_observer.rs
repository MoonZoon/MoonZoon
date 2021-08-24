use crate::*;

// ------ ResizeObserver ------

pub struct ResizeObserver {
    observer: native::ResizeObserver,
    _callback: Closure<dyn Fn(Vec<native::ResizeObserverEntry>)>,
}

impl ResizeObserver {
    #[must_use]
    pub fn new(ws_element: &web_sys::Element, on_resize: impl FnOnce(u32, u32) + Clone + 'static) -> Self {
        let on_resize = move |width, height| on_resize.clone()(width, height); 

        let callback = move |entries: Vec<native::ResizeObserverEntry>| {
            let entry = &entries[0];
            let size = entry.border_box_size();
            let width = size.inline_size();
            let height = size.block_size();
            on_resize(width, height);
        };

        let callback = Closure::wrap(
            Box::new(callback) as Box<dyn Fn(Vec<native::ResizeObserverEntry>)>
        );

        let observer = native::ResizeObserver::new(callback.as_ref().unchecked_ref());
        observer.observe(ws_element);

        Self {
            observer,
            _callback: callback,
        }
    }
}

impl Drop for ResizeObserver {
    fn drop(&mut self) {
        self.observer.disconnect();
    }
}

// ----- Native ------

mod native {
    use crate::*;
    use js_sys::Function;
    use web_sys::Element;

    #[wasm_bindgen]
    extern "C" {
        // ------ ResizeObserver ------

        pub type ResizeObserver;

        #[wasm_bindgen(constructor)]
        pub fn new(callback: &Function) -> ResizeObserver;

        #[wasm_bindgen(method)]
        pub fn disconnect(this: &ResizeObserver);

        #[wasm_bindgen(method)]
        pub fn observe(this: &ResizeObserver, target: &Element);

        // ------ ResizeObserverEntry ------

        pub type ResizeObserverEntry;

        #[wasm_bindgen(method, getter, js_name = "borderBoxSize")]
        pub fn border_box_size(this: &ResizeObserverEntry) -> ResizeObserverSize;

        // ------ ResizeObserverSize ------

        pub type ResizeObserverSize;

        #[wasm_bindgen(method, getter, js_name = "blockSize")]
        pub fn block_size(this: &ResizeObserverSize) -> u32;

        #[wasm_bindgen(method, getter, js_name = "inlineSize")]
        pub fn inline_size(this: &ResizeObserverSize) -> u32;
    }

}
