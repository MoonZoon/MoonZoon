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
        .child_signal(boon_document_root().into_signal_option())    
}

async fn boon_document_root() -> impl Element {
    // let program = include_str!("examples/counter.bn");
    // let program = include_str!("examples/simple_counter.bn");
    let program = include_str!("examples/interval.bn");
    // let program = include_str!("examples/todo_mvc.bn");

    interpreter::run(program).await
}
