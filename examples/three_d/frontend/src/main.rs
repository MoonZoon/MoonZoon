use zoon::*;

mod picking;
mod shapes2d;
mod triangle;

const CANVAS_WIDTH: u32 = 500;
const CANVAS_HEIGHT: u32 = 500;

pub fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        // `three-d` examples: https://github.com/asny/three-d/blob/master/examples/README.md
        .item(panel_with_canvas({
            let suzanne_model_url = {
                let url = public_url("suzanne.obj");
                // `three_d_asset::io::load_async(..)` fails when the url starts with `/`
                url.strip_prefix("/").unwrap_throw().to_owned()
            };
            |canvas| Task::start(picking::run(canvas, suzanne_model_url))
        }))
    // Only one event loop is supported by `three-d` so comment out the previous example/`item`
    // before you enable an `item` below.
    // Note: There is a workaround (https://github.com/asny/three-d/blob/master/examples/multiwindow/src/main.rs)
    // but it hasn't been implemented for the sake of simplicity.
    // @TODO Is possible to make `rustfmt` to skip this block of comments to stop moving it to the left?
    // .item(panel_with_canvas(shapes2d::run))
    // .item(panel_with_canvas(triangle::run))
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
