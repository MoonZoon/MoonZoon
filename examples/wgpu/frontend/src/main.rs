use zoon::*;

#[allow(dead_code)]
mod hello_triangle;
mod hello_world;

const CANVAS_WIDTH: u32 = 350;
const CANVAS_HEIGHT: u32 = 350;

pub fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        .item(panel_with_canvas(hello_triangle::run))
        .item(panel_with_canvas(hello_world::run))
}

fn panel_with_canvas(
    example_runner: impl FnOnce(web_sys::HtmlCanvasElement) + 'static,
) -> impl Element {
    El::new()
        .s(Align::center())
        .s(Clip::both())
        .s(Borders::all(Border::new().color(color!("Gray"))))
        .child(
            Canvas::new()
                .width(CANVAS_WIDTH)
                .height(CANVAS_HEIGHT)
                .after_insert(example_runner),
        )
}
