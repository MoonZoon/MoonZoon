use zoon::*;

mod hello_triangle;

const CANVAS_WIDTH: u32 = 350;
const CANVAS_HEIGHT: u32 = 350;

pub fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        // https://github.com/gfx-rs/wgpu/tree/trunk/examples/src/hello_triangle
        .item(panel_with_canvas(|canvas| {
            Task::start(hello_triangle::run(canvas))
        }))
}

fn panel_with_canvas(
    example_runner: impl FnOnce(web_sys::HtmlCanvasElement) + 'static,
) -> impl Element {
    El::new()
        .s(Align::center())
        .s(RoundedCorners::all(30))
        .s(Clip::both())
        .child(
            Canvas::new()
                .width(CANVAS_WIDTH)
                .height(CANVAS_HEIGHT)
                .after_insert(example_runner),
        )
}
