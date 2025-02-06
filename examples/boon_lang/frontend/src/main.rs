

use zoon::*;

mod boon;
use boon::platform::browser::{
    bridge::object_with_document_to_element_signal,
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
        .child(boon_object_with_document())
}

macro_rules! run_example {
    ($name:literal) => {{
        interpreter::run(
            concat!($name, ".bn"), 
            include_str!(concat!("examples/", $name, "/", $name, ".bn"))
        )
    }}
}

fn boon_object_with_document() -> impl Element {
    // -- Choose example! --
    let object = run_example!("call_document_new");
    // let object = run_example!("interval");
    // let object = run_example!("counter");
    // let object = run_example!("complex_counter");

    El::new()
        .child_signal(object_with_document_to_element_signal(object.clone()))
        .after_remove(move |_| drop(object))
}
