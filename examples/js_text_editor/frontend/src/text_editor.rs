use zoon::*;
use std::{iter, sync::Arc};

// ------ TextEditor ------

pub struct TextEditor {
    raw_el: RawHtmlEl,
    controller: Mutable<Option<Arc<externs::QuillController>>>,
}

impl TextEditor {
    pub fn new() -> Self {
        let controller = Mutable::default();

        let raw_el = RawHtmlEl::new("div")
            .after_insert(clone!((controller) move |html_element| {
                controller.set(Some(Arc::new(externs::QuillController::new(&html_element))))
            }))
            .after_remove(clone!((controller) move |_| drop(controller)));

        Self {
            raw_el,
            controller,
        }
    }

    pub fn on_change(mut self, on_change: impl Fn(serde_json::Value) + 'static) -> Self {
        let callback = move |json: JsString| {
            let json = json.as_string().and_then(|json| serde_json::from_str(&json).ok());
            on_change(json.expect_throw("failed to parse Quill contents"))
        };

        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn(JsString)>);

        let on_change_setter = Task::start_droppable(self.controller.signal_cloned().for_each_sync(move |controller| {
            if let Some(controller) = controller {
                controller.on_change(&closure);
            }
        }));
        self.raw_el = self.raw_el.after_remove(move |_| drop(on_change_setter));

        self
    }
}

impl Element for TextEditor {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into_raw_element()
    }
}

impl IntoIterator for TextEditor {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

// ------ ------
//    Externs
// ------ ------

mod externs {
    use super::*;
    #[wasm_bindgen(module = "/js/quill_controller.js")]
    extern "C" {
        pub type QuillController;

        #[wasm_bindgen(constructor)]
        pub fn new(element: &JsValue) -> QuillController;

        #[wasm_bindgen(method)]
        pub fn on_change(this: &QuillController, on_change: &Closure<dyn Fn(JsString)>);
    }
}
