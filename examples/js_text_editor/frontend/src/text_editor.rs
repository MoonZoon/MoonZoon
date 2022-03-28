use std::{cell::Cell, iter, rc::Rc, sync::Arc};
use zoon::{futures_util::join, *};

#[static_ref]
fn load_assets_once() -> &'static Mutable<bool> {
    Task::start(async {
        join!(
            load_stylesheet("https://cdn.quilljs.com/1.3.6/quill.snow.css"),
            load_script("https://cdn.quilljs.com/1.3.6/quill.min.js"),
        );
        load_assets_once().set(true);
    });
    Mutable::default()
}

// ------ TextEditor ------

pub struct TextEditor {
    raw_el: RawHtmlEl,
    controller: Mutable<Option<Arc<externs::QuillController>>>,
}

impl TextEditor {
    pub fn new() -> Self {
        load_assets_once();
        let controller = Mutable::default();
        let controller_creator = Rc::new(Cell::new(None));

        let raw_el = RawHtmlEl::new("div")
            .after_insert(clone!((controller, controller_creator) move |html_element| {
                controller_creator.set(Some(Task::start_droppable(async move {
                    load_assets_once().signal().wait_for(true).await;
                    controller.set(Some(Arc::new(externs::QuillController::new(&html_element))));
                })));
            }))
            .after_remove(clone!((controller) move |_| {
                drop(controller);
                drop(controller_creator);
            }));

        Self { raw_el, controller }
    }

    pub fn on_change(mut self, on_change: impl Fn(serde_json::Value) + 'static) -> Self {
        let callback = move |json: JsString| {
            let json = json
                .as_string()
                .and_then(|json| serde_json::from_str(&json).ok())
                .expect_throw("failed to parse Quill contents");
            on_change(json)
        };

        let closure = Closure::wrap(Box::new(callback) as Box<dyn Fn(JsString)>);

        let on_change_setter =
            Task::start_droppable(self.controller.signal_cloned().for_each_sync(
                move |controller| {
                    if let Some(controller) = controller {
                        controller.on_change(&closure);
                    }
                },
            ));
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
    #[wasm_bindgen(module = "/js/text_editor/quill_controller.js")]
    extern "C" {
        pub type QuillController;

        #[wasm_bindgen(constructor)]
        pub fn new(element: &JsValue) -> QuillController;

        #[wasm_bindgen(method)]
        pub fn on_change(this: &QuillController, on_change: &Closure<dyn Fn(JsString)>);
    }
}
