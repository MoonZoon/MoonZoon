use zoon::*;

mod boon;
use boon::platform::browser::{bridge::object_with_document_to_element_signal, interpreter};

mod code_editor;
use code_editor::{CodeEditorController, CodeEditor};

// @TODO remove
// mod examples;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    let code_editor_controller = Mutable::default();
    Column::new()
        .s(Width::fill())
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        .s(Font::new().size(17).color(color!("oklch(0.8 0 0)")))
        .s(Scrollbars::both())
        .item(
            Row::new()
                .s(Width::fill())
                .s(Height::fill())
                .s(Scrollbars::both())
                .item(code_editor_panel(code_editor_controller))
                .item(example_panel())
        )
}

fn code_editor_panel(code_editor_controller: Mutable<Mutable<Option<SendWrapper<CodeEditorController>>>>) -> impl Element {
    El::new()
        .s(Align::new().top())
        .s(Width::fill())
        .s(Height::fill())
        .s(Padding::all(5))
        .s(Scrollbars::both())
        .child(
            CodeEditor::new()
                .s(RoundedCorners::all(10))
                .s(Scrollbars::both())
                .task_with_controller(move |controller| {
                    code_editor_controller.set(controller.clone());
                    async {}
                })
        )
}

fn example_panel() -> impl Element {
    El::new()
        .s(Align::new().top())
        .s(Width::fill())
        .s(Height::fill())
        .s(Padding::all(5))
        .child(
            El::new()
                .s(RoundedCorners::all(10))
                .s(Clip::both())
                .s(Borders::all(
                    Border::new()
                    .color(color!("#282c34"))
                    .width(4)
                ))
                .child(boon_object_with_document())
        )
}

macro_rules! run_example {
    ($name:literal) => {{
        interpreter::run(
            concat!($name, ".bn"),
            include_str!(concat!("examples/", $name, "/", $name, ".bn")),
        )
    }};
}

fn boon_object_with_document() -> impl Element {
    // -- Choose example! --
    // let object = run_example!("call_document_new");
    // let object = run_example!("interval");
    let object = run_example!("counter");

    // NOT RUNNABLE YET
    // let object = run_example!("complex_counter");
    // let object = run_example!("todo_mvc");

    if let Some(object) = object {
        El::new()
            .child_signal(object_with_document_to_element_signal(object.clone()))
            .after_remove(move |_| drop(object))
            .unify()
    } else {
        El::new().child("Failed to get Boon root Object").unify()
    }
}
