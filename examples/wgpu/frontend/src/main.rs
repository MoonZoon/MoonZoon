use zoon::*;

mod pong;

const CANVAS_WIDTH: u32 = 500;
const CANVAS_HEIGHT: u32 = 500;

pub fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        .item(panel_with_canvas(pong::run))
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
