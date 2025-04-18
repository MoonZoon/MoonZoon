use zoon::*;

#[allow(dead_code)]
mod hello_triangle;

#[allow(dead_code)]
mod hello_world;

#[allow(dead_code)]
mod rust_logo;

const CANVAS_WIDTH: u32 = 350;
const CANVAS_HEIGHT: u32 = 350;

pub fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        // NOTE 1: Only one example can be run at a time becuse there can be only one Winit EventLoop
        // NOTE 2: Some examples disappear when the window is moved to another monitor because the canvas is not redrawn
        // .item(panel_with_canvas(hello_triangle::run))
        // .item(panel_with_canvas(hello_world::run))
        .item(panel_with_canvas(rust_logo::run))
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
