use boon::zoon::*;
use std::rc::Rc;

pub struct CodeEditor {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
    controller: Mutable<Option<js_bridge::CodeEditorController>>,
}

impl Element for CodeEditor {}

impl RawElWrapper for CodeEditor {
    type RawEl = RawHtmlEl<web_sys::HtmlElement>;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Styleable<'_> for CodeEditor {}
impl KeyboardEventAware for CodeEditor {}
impl MouseEventAware for CodeEditor {}
impl PointerEventAware for CodeEditor {}
impl TouchEventAware for CodeEditor {}
impl AddNearbyElement<'_> for CodeEditor {}
impl HasIds for CodeEditor {}

impl CodeEditor {
    pub fn new() -> Self {
        let controller: Mutable<Option<js_bridge::CodeEditorController>> = Mutable::new(None);
        Self {
            controller: controller.clone(),
            raw_el: El::new()
                .after_insert(clone!((controller) move |element| {
                    let code_editor_controller = js_bridge::CodeEditorController::new();
                    code_editor_controller.init(&element);
                    controller.set(Some(code_editor_controller));
                }))
                .after_remove(move |_| {
                    drop(controller);
                })
                .into_raw_el(),
        }
    }

    pub fn content_signal(
        self,
        content: impl Signal<Item = impl IntoCowStr<'static>> + 'static,
    ) -> Self {
        let controller = self.controller.clone();
        let task = Task::start_droppable(async move {
            let controller = controller.wait_for_some_cloned().await;
            content
                .for_each_sync(|content| controller.set_content(&content.into_cow_str()))
                .await;
        });
        self.after_remove(move |_| drop(task))
    }

    pub fn on_change(self, mut on_change: impl FnMut(String) + 'static) -> Self {
        let callback = move |content: JsString| {
            let content = content
                .as_string()
                .expect_throw("Failed to get CodeEditor content");
            on_change(content)
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
    use boon::zoon::*;

    // Note: Add all corresponding methods to `frontend/typescript/code_editor/code_editor.ts`
    #[wasm_bindgen(module = "/typescript/bundles/code_editor.js")]
    extern "C" {
        #[derive(Clone)]
        pub type CodeEditorController;

        #[wasm_bindgen(constructor)]
        pub fn new() -> CodeEditorController;

        #[wasm_bindgen(method)]
        pub fn init(this: &CodeEditorController, parent_element: &JsValue);

        #[wasm_bindgen(method)]
        pub fn set_content(this: &CodeEditorController, content: &str);

        #[wasm_bindgen(method)]
        pub fn on_change(this: &CodeEditorController, on_change: &Closure<dyn FnMut(JsString)>);
    }
}
