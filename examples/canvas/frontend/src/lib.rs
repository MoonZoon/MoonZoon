use zoon::*;

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

// ------ ------
//    Signals
// ------ ------

fn fill_style() -> impl Signal<Item = JsValue> {
    color().signal().map(|color| {
        match color {
            Color::Red => "red",
            Color::Blue => "blue"
        }.into()
    })
}

// ------ ------
//   Commands
// ------ ------

fn toggle_color() {
    use self::Color::{Red, Blue};
    color().update(|color| if let Red = color { Blue } else { Red });
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item(canvas())
        .item(change_color_button())
}

fn canvas() -> impl Element {
    Text::new("canvas")

    // let fill_style = fill_style(); 
    // canvas![
    //         canvas::width(300),
    //         canvas::height(300),
    //         canvas::on_ready(|canvas| {
    //             let ctx = canvas.context_2d();
    //             ctx.lineWidth = 10;
    //             fill_style.use_ref(|style| ctx.set_fill_style(style));
    //             // Wall
    //             ctx.stroke_rect(75., 140., 150., 110.);
    //             // Door
    //             ctx.fill_rect(130., 190., 40., 60.);
    //             // Roof
    //             ctx.begin_path();
    //             ctx.move_to(50., 140.);
    //             ctx.line_to(150., 60.);
    //             ctx.line_to(250., 140.);
    //             ctx.close_path();
    //             ctx.stroke();
    //         });
    //     ],
}

fn change_color_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().all(6))
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

