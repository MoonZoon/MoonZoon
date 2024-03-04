use zoon::*;

#[derive(Copy, Clone, Default)]
enum Color {
    #[default]
    Red,
    Blue,
}

static COLOR: Lazy<Mutable<Color>> = lazy::default();

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Borders::all(Border::new().color(color!("Gray"))))
        .s(RoundedCorners::all(30))
        .s(Clip::both())
        .item(canvas())
        .item(change_color_button())
}

fn canvas() -> impl Element {
    let paint_task = Mutable::new(None);
    Canvas::new()
        .width(300)
        .height(300)
        .after_insert(clone!((paint_task) move |canvas| {
            let ctx = canvas
                .get_context("2d")
                .unwrap_throw()
                .unwrap_throw()
                .unchecked_into::<web_sys::CanvasRenderingContext2d>();
            paint_task.set(Some(Task::start_droppable(COLOR.signal().for_each_sync(move |color| {
                let style = match color {
                    Color::Red => color!("DarkRed"),
                    Color::Blue => color!("DarkBlue"),
                };
                let style = JsValue::from(style.to_css_string());

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
            }))));
        }))
        .after_remove(move |_| drop(paint_task))
}

fn change_color_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(10))
        .s(RoundedCorners::new().bottom(30))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("Green"), || color!("DarkGreen"))))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Change color")
        .on_press(|| {
            COLOR.update(|color| {
                if let Color::Red = color {
                    Color::Blue
                } else {
                    Color::Red
                }
            })
        })
}
