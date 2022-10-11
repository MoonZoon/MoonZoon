use zoon::{
    named_color::*,
    println,
    strum::{EnumIter, IntoEnumIterator},
    *,
};

// ------ ------
//     Types
// ------ ------

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[strum(crate = "strum")]
enum Rectangle {
    A,
    B,
    C,
}

impl Rectangle {
    fn color_and_align(&self) -> (HSLuv, Align) {
        match self {
            Self::A => (RED_6, Align::new()),
            Self::B => (GREEN_6, Align::center()),
            Self::C => (BLUE_6, Align::new().bottom().right()),
        }
    }
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn rectangles() -> &'static MutableVec<Rectangle> {
    MutableVec::new_with_values(Rectangle::iter().collect())
}

#[static_ref]
fn spread_oscillator() -> &'static Oscillator {
    let oscillator = Oscillator::with_speed(Duration::seconds(2));
    oscillator.cycle();
    oscillator
}

// ------ ------
//   Commands
// ------ ------

fn bring_to_front(rectangle: Rectangle) {
    let mut rectangles = rectangles().lock_mut();
    let position = rectangles
        .iter()
        .position(|r| r == &rectangle)
        .unwrap_throw();
    rectangles.move_from_to(position, rectangles.len() - 1);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::exact(180))
        .s(Height::exact(180))
        .layers_signal_vec(rectangles().signal_vec().map(rectangle))
}

fn rectangle(rectangle: Rectangle) -> impl Element {
    println!("render Rectangle '{rectangle:?}'");
    let (color, align) = rectangle.color_and_align();

    let lightness_oscillator = Oscillator::fast();
    let color = lightness_oscillator
        .signal()
        .map(ease::linear_unit(color.l(), color.l() + 5.))
        .map(move |l| color.set_l(l));

    El::new()
        .s(Transform::with_signal(
            spread_oscillator().signal().map(|spread| Transform::new().scale(100. + spread * 20.)),
        ))
        .s(Width::exact(100))
        .s(Height::exact(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(GRAY_8)]))
        .s(Background::new().color_signal(color))
        .s(align)
        .on_hovered_change(move |is_hovered| lightness_oscillator.go_to(is_hovered))
        .on_click(move || bring_to_front(rectangle))
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
}
