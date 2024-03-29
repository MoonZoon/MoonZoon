use std::rc::Rc;
use zoon::{futures_util::join, *};

pub struct TextEditor {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
    controller: ReadOnlyMutable<Option<js_bridge::QuillController>>,
}

impl Element for TextEditor {}

impl RawElWrapper for TextEditor {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl TextEditor {
    pub fn new() -> Self {
        let controller = Mutable::new(None);
        Self {
            controller: controller.read_only(),
            raw_el: El::new()
                .update_raw_el(|raw_el| {
                    raw_el.style_group(StyleGroup::new(" *").style("white-space", "unset"))
                })
                .child(RawHtmlEl::new("div").after_insert(move |html_element| {
                    Task::start(async move {
                        run_once_async!(async {
                            join!(
                                load_stylesheet("https://cdn.quilljs.com/1.3.6/quill.snow.css"),
                                load_script("https://cdn.quilljs.com/1.3.6/quill.min.js"),
                            );
                        })
                        .await;
                        controller.set(Some(js_bridge::QuillController::new(&html_element)));
                    });
                }))
                .into_raw_el(),
        }
    }

    pub fn on_change(self, on_change: impl Fn(serde_json::Value) + 'static) -> Self {
        let callback = move |json: JsString| {
            let json = json
                .as_string()
                .and_then(|json| serde_json::from_str(&json).ok())
                .expect_throw("failed to parse Quill contents");
            on_change(json)
        };
        let closure = Rc::new(Closure::new(callback));
        let task =
            Task::start_droppable(self.controller.wait_for_some_ref(
                clone!((closure) move |controller| controller.on_change(&closure)),
            ));
        self.after_remove(move |_| {
            drop(task);
            drop(closure);
        })
    }
}

mod js_bridge {
    use super::*;
    // https://rustwasm.github.io/wasm-bindgen/reference/js-snippets.html
    #[wasm_bindgen(module = "/js/text_editor/quill_controller.js")]
    extern "C" {
        pub type QuillController;

        #[wasm_bindgen(constructor)]
        pub fn new(element: &JsValue) -> QuillController;

        #[wasm_bindgen(method)]
        pub fn on_change(this: &QuillController, on_change: &Closure<dyn Fn(JsString)>);
    }
}
