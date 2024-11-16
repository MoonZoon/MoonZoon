use zoon::*;

mod interpreter;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .s(Background::new().color(color!("oklch(0.4 0 0)")))
        .child_signal(boon_document_root())    
}

fn boon_document_root() -> impl Signal<Item = Option<impl Element>> {
    let program = include_str!("counter.bn");
    let element_future = interpreter::run(program);
    signal::from_future(Box::pin(element_future))
}
