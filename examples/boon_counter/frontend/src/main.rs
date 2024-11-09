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
        .child(boon_document_root())    
}

fn boon_document_root() -> impl Element {
    let program = include_str!("counter.bn");
    interpreter::run(program)
}
