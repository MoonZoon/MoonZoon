use zoon::*;
use zoon::web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

// ------ ------
//    Statics
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
fn canvas_html_el() -> &'static Mutable<Option<SendWrapper<HtmlCanvasElement>>> {
    Mutable::new(None)
}

// ------ ------
//   Commands
// ------ ------

fn toggle_color() {
    use self::Color::{Red, Blue};
    color().update(|color| if let Red = color { Blue } else { Red });
    paint_canvas();
}

fn paint_canvas() {
    let canvas_lock = canvas_html_el().lock_ref();
    let canvas = match canvas_lock.as_ref() {
        Some(canvas) => canvas,
        None => return
    };

    let style: JsValue = match color().get() {
        Color::Red => "darkred",
        Color::Blue => "darkblue"
    }.into();

    let ctx = canvas
        .get_context("2d")
        .unwrap_throw()
        .unwrap_throw()
        .unchecked_into::<CanvasRenderingContext2d>();

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

fn set_canvas_html_el(canvas: HtmlCanvasElement) {
    canvas_html_el().set(Some(SendWrapper::new(canvas)));
    paint_canvas();
}

fn unset_canvas_html_el() {
    canvas_html_el().take();
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Width::new(300))
        .item(canvas())
        .item(change_color_button())
}

fn canvas() -> impl Element {
    Canvas::new()
        .width(300)
        .height(300)
        .after_insert(set_canvas_html_el)
        .after_remove(|_| unset_canvas_html_el())
}

fn change_color_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().all(10))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2))
        )
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

