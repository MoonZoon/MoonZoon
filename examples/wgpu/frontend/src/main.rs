use zoon::*;

mod lyon_svg;

const WINDOW_SIZE: u32 = 800;

pub fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Height::fill())
        .s(Background::new().color(color!("Black")))
        .item(panel_with_canvas(lyon_svg::run))
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
                .width(WINDOW_SIZE)
                .height(WINDOW_SIZE)
                .after_insert(example_runner),
        )
}
