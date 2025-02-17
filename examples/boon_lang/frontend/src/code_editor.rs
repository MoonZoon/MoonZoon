pub use js_bridge::CodeEditorController;
use zoon::*;

pub struct CodeEditor {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
    controller: Mutable<Option<SendWrapper<js_bridge::CodeEditorController>>>,
    task_with_controller: Mutable<Option<TaskHandle>>,
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
        let controller: Mutable<Option<SendWrapper<js_bridge::CodeEditorController>>> =
            Mutable::new(None);
        let task_with_controller = Mutable::new(None);
        Self {
            controller: controller.clone(),
            task_with_controller: task_with_controller.clone(),
            raw_el: El::new()
                .s(RoundedCorners::new().bottom(10))
                .s(Clip::both())
                .after_insert(clone!((controller) move |element| {
                    Task::start(async move {
                        let code_editor_controller = SendWrapper::new(js_bridge::CodeEditorController::new());
                        code_editor_controller.init(&element).await;
                        controller.set(Some(code_editor_controller));
                    });
                }))
                .after_remove(move |_| {
                    drop(task_with_controller);
                })
                .into_raw_el(),
        }
    }

    pub fn task_with_controller<FUT: Future<Output = ()> + 'static>(
        self,
        f: impl FnOnce(Mutable<Option<SendWrapper<js_bridge::CodeEditorController>>>) -> FUT,
    ) -> Self {
        self.task_with_controller
            .set(Some(Task::start_droppable(f(self.controller.clone()))));
        self
    }
}

mod js_bridge {
    use zoon::*;

    // Note: Add all corresponding methods to `frontend/typescript/code_editor/code_editor.ts`
    #[wasm_bindgen(module = "/typescript/bundles/code_editor.js")]
    extern "C" {
        #[derive(Clone)]
        pub type CodeEditorController;

        #[wasm_bindgen(constructor)]
        pub fn new() -> CodeEditorController;

        #[wasm_bindgen(method)]
        pub async fn init(this: &CodeEditorController, parent_element: &JsValue);

        #[wasm_bindgen(method)]
        pub fn set_content(
            this: &CodeEditorController,
            content: String,
        );
    }
}
