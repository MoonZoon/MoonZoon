use zoon::*;

mod boon;
use boon::platform::browser::bridge::root_object_to_element_signal;

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
        .child_signal(boon_document_root().into_signal_option())
}

async fn boon_document_root() -> impl Element {
    let root_object = examples::call_document_new::run().await;
    // let root_object = examples::interval::run().await;
    // let root_object = examples::counter::run().await;

    El::new()
        .child_signal(root_object_to_element_signal(root_object.clone()))
        .after_remove(move |_| drop(root_object))
}
