use zoon::*;
use std::rc::Rc;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn contents() -> &'static Mutable<Option<String>> {
    Mutable::default()
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .item(text_editor())
        .item(contents_display())
}

fn text_editor() -> impl Element {
    fn format_contents(json: JsString) -> Option<String> {
        let json = json.as_string()?;
        let json: serde_json::Value = serde_json::from_str(&json).ok()?;
        Some(format!("{json:#}"))
    }

    let on_change = Rc::new(Closure::wrap(Box::new(|json: JsString| {
        contents().set(format_contents(json)) 
    }) as Box<dyn Fn(JsString)>));

    El::new()
        .after_insert(clone!((on_change) move |html_element| {
            externs::QuillController::new(html_element.into()).on_change(&on_change);
        }))
        .after_remove(|_| drop(on_change))
}

fn contents_display() -> impl Element {
    El::new()
        .s(Padding::all(10))
        .s(Font::new().family([FontFamily::Monospace]))
        .child_signal(contents().signal_cloned())
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
        pub fn new(element: JsValue) -> QuillController;

        #[wasm_bindgen(method)]
        pub fn on_change(this: &QuillController, on_change: &Closure<dyn Fn(JsString)>);
    }
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
