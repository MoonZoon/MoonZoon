use zoon::{
    named_color::*,
    strum::{AsRefStr, EnumIter, IntoEnumIterator},
    *,
};

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, EnumIter, AsRefStr)]
#[strum(crate = "strum")]
enum RectangleAlignment {
    TopRight,
    Center,
    BottomLeft,
}

impl RectangleAlignment {
    fn to_align(&self) -> Align {
        match self {
            Self::TopRight => Align::new().top().right(),
            Self::Center => Align::center(),
            Self::BottomLeft => Align::new().bottom().left(),
        }
    }
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn rectangle_alignment() -> &'static Mutable<Option<RectangleAlignment>> {
    Mutable::default()
}

// ------ ------
//   Commands
// ------ ------

fn set_rectangle_alignment(alignment: RectangleAlignment) {
    rectangle_alignment().set(Some(alignment))
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::exact(200))
        .s(Height::exact(200))
        .s(Borders::all(Border::new().color(GRAY_5).width(3)))
        .s(RoundedCorners::all(15))
        .layer(rectangle())
        .layers(RectangleAlignment::iter().map(button))
}

fn button(rectangle_alignment: RectangleAlignment) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(rectangle_alignment.to_align())
        .s(Background::new().color_signal(hovered_signal.map_bool(|| BLUE_7, || BLUE_9)))
        .s(Padding::all(5))
        .s(RoundedCorners::all(10))
        .label(rectangle_alignment.as_ref())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || set_rectangle_alignment(rectangle_alignment))
}

fn rectangle() -> impl Element {
    El::new()
        .s(Width::exact(70))
        .s(Height::exact(70))
        .s(Background::new().color(GREEN_7))
        .s(RoundedCorners::all(10))
        .s(Align::with_signal(rectangle_alignment().signal_ref(
            |alignment| alignment.map(|alignment| alignment.to_align()),
        )))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
