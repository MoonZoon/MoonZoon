use zoon::*;

mod boon;
use boon::platform::browser::{
    bridge::root_object_to_element_signal,
    interpreter
};

mod examples;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .s(Background::new().color(color!("oklch(0.4 0 0)")))
        .s(Font::new().color(color!("oklch(0.8 0 0)")))
        .child(boon_document_root())
}

fn boon_document_root() -> impl Element {
    let root_object = interpreter::run(include_str!(
        "examples/call_document_new/call_document_new.bn"
    ));

    // let root_object = interpreter::run(include_str!(
    //     "examples/interval/interval.bn"
    // ));

    // let root_object = interpreter::run(include_str!(
    //     "examples/counter/counter.bn"
    // ));

    // let root_object = interpreter::run(include_str!(
    //     "examples/complex_counter/complex_counter.bn"
    // ));

    El::new()
        .child_signal(root_object_to_element_signal(root_object.clone()))
        .after_remove(move |_| drop(root_object))
}
