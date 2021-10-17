use zoon::{
    web_sys::{CanvasRenderingContext2d, HtmlCanvasElement},
    *,
    named_color::*,
};

// ------ ------
//    States
// ------ ------

#[derive(Copy, Clone)]
enum Color {
    Red,
    Blue,
}

#[static_ref]
fn color() -> &'static Mutable<Color> {
    Mutable::new(Color::Red)
}

#[static_ref]
fn canvas_context() -> &'static Mutable<Option<SendWrapper<CanvasRenderingContext2d>>> {
    Mutable::new(None)
}

// ------ ------
//   Commands
// ------ ------

fn toggle_color() {
    use self::Color::{Blue, Red};
    color().update(|color| if let Red = color { Blue } else { Red });
    paint_canvas();
}

fn set_canvas_context(canvas: HtmlCanvasElement) {
    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .unchecked_into::<CanvasRenderingContext2d>();
    canvas_context().set(Some(SendWrapper::new(ctx)));
    paint_canvas();
}

fn remove_canvas_context() {
    canvas_context().take();
}

fn paint_canvas() {
    canvas_context().use_ref(|ctx| {
        if let Some(ctx) = ctx.as_ref() {
            let style = match color().get() {
                Color::Red => "darkred",
                Color::Blue => "darkblue",
            }
            .apply(JsValue::from);

            ctx.set_line_width(10.);
            ctx.set_fill_style(&style);
            ctx.set_stroke_style(&style);
            // Wall
            ctx.stroke_rect(75., 140., 150., 110.);
            // Door
            ctx.fill_rect(130., 190., 40., 60.);
            // Roof
            ctx.begin_path();
            ctx.move_to(50., 140.);
            ctx.line_to(150., 60.);
            ctx.line_to(250., 140.);
            ctx.close_path();
            ctx.stroke();
        }
    });
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Borders::all(Border::new().color(GRAY_7)))
        .s(RoundedCorners::all(30))
        .s(Clip::both())
        .item(canvas())
        .item(change_color_button())
}

fn canvas() -> impl Element {
    Canvas::new()
        .width(300)
        .height(300)
        .after_insert(set_canvas_context)
        .after_remove(|_| remove_canvas_context())
}

fn change_color_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(10))
        .s(RoundedCorners::new().bottom(30))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GREEN_8, || GREEN_9)))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Change color")
        .on_press(toggle_color)
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
