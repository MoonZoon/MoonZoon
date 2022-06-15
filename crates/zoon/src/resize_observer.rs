use crate::*;

// ------ ResizeObserver ------

pub struct ResizeObserver {
    observer: native::ResizeObserver,
    _callback: Closure<dyn FnMut(Vec<native::ResizeObserverEntry>)>,
}

impl ResizeObserver {
    #[must_use]
    pub fn new(
        ws_element: &web_sys::Element,
        mut on_resize: impl FnMut(u32, u32) + 'static,
    ) -> Self {
        let callback = move |entries: Vec<native::ResizeObserverEntry>| {
            let entry = &entries[0];
            let (width, height) = entry_size(&entry);
            on_resize(width, height);
        };
        let callback = Closure::new(callback);

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

fn entry_size(entry: &native::ResizeObserverEntry) -> (u32, u32) {
    if not(js_sys::Reflect::has(entry, &"borderBoxSize".into()).unwrap_throw()) {
        // Safari, browsers on iOS and maybe others
        let dom_rect = entry.content_rect();
        return (dom_rect.width() as u32, dom_rect.height() as u32);
    }

    let size = entry.border_box_size();
    let size: native::ResizeObserverSize = if size.is_instance_of::<native::ResizeObserverSize>() {
        // Firefox and maybe others
        size.unchecked_into()
    } else if size.is_instance_of::<js_sys::Array>() {
        // Chrome and maybe others
        size.unchecked_into::<js_sys::Array>()
            .get(0)
            .unchecked_into()
    } else {
        panic!("cannot get size from ResizeObserverEntry")
    };

    let width = size.inline_size();
    let height = size.block_size();
    (width, height)
}

// ----- Native ------

mod native {
    use crate::*;
    use js_sys::Function;
    use web_sys::{DomRectReadOnly, Element};

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
        pub fn border_box_size(this: &ResizeObserverEntry) -> JsValue;

        #[wasm_bindgen(method, getter, js_name = "contentRect")]
        pub fn content_rect(this: &ResizeObserverEntry) -> DomRectReadOnly;

        // ------ ResizeObserverSize ------

        pub type ResizeObserverSize;

        #[wasm_bindgen(method, getter, js_name = "blockSize")]
        pub fn block_size(this: &ResizeObserverSize) -> u32;

        #[wasm_bindgen(method, getter, js_name = "inlineSize")]
        pub fn inline_size(this: &ResizeObserverSize) -> u32;
    }
}
