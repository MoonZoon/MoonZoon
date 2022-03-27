use zoon::*;

// ------ TextEditor ------

pub struct TextEditor {
    #[allow(dead_code)]
    controller: externs::QuillController,
    on_change: Option<Closure<dyn Fn(JsString)>>,
}

impl TextEditor {
    pub fn new(element: impl AsRef<web_sys::Element>) -> Self {
        Self {
            controller: externs::QuillController::new(element.as_ref()),
            on_change: None,
        }
    }

    pub fn on_change(mut self, on_change: impl Fn(Option<serde_json::Value>) + 'static) -> Self {
        let callback = move |json: JsString| {
            let json = json.as_string().and_then(|json| serde_json::from_str(&json).ok());
            on_change(json)
        };
        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn(JsString)>);
        self.controller.on_change(&closure);
        self.on_change = Some(closure);
        self
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
