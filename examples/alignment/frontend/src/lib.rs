use zoon::{*, named_color::*};

#[static_ref]
fn alignment() -> &'static Mutable<Option<Align<'static>>> {
    Mutable::default()
}

fn set_alignment(align: Align<'static>) {
    alignment().set(Some(align))
}

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::new(200))
        .s(Height::new(200))
        .s(Borders::all(Border::new().color(GRAY_5).width(3)))
        .s(RoundedCorners::all(10))
        .layer(button("Top Right", || Align::new().top().right()))
        .layer(button("Center",|| Align::center()))
        .layer(button("Bottom Left", || Align::new().bottom().left()))
}

fn button(label: &str, align: fn() -> Align<'static>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(align())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || BLUE_7, || BLUE_9
        )))
        .s(Padding::all(5))
        .s(RoundedCorners::all(10))
        .label(label)
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || set_alignment(align()))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
