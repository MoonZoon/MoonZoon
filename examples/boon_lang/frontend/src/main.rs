use zoon::*;

mod runtime;

mod example_interval;
mod example_simple_counter;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .s(Background::new().color(color!("oklch(0.4 0 0)")))
        .child_signal(boon_document_root().into_signal_option())    
}

async fn boon_document_root() -> impl Element {
    // example_interval::run().await
    example_simple_counter::run().await
}
