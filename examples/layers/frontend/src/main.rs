use zoon::{
    println,
    strum::{EnumIter, IntoEnumIterator},
    *,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[strum(crate = "strum")]
enum Rectangle {
    A,
    B,
    C,
}

impl Rectangle {
    fn color_and_align(&self) -> (Oklch, Align) {
        match self {
            // https://oklch.com/
            Self::A => (color!("oklch(62% 0.25 29)"), Align::new()),
            Self::B => (color!("oklch(51% 0.17 142)"), Align::center()),
            Self::C => (color!("oklch(45% 0.31 264)"), Align::new().bottom().right()),
        }
    }
}

static RECTANGLES: Lazy<MutableVec<Rectangle>> =
    Lazy::new(|| MutableVec::new_with_values(Rectangle::iter().collect()));

static SPREAD_OSCILLATOR: Lazy<Oscillator> = Lazy::new(|| {
    let oscillator = Oscillator::new(Duration::seconds(2));
    oscillator.cycle();
    oscillator
});

fn bring_to_front(rectangle: Rectangle) {
    let mut rectangles = RECTANGLES.lock_mut();
    let position = rectangles
        .iter()
        .position(|r| r == &rectangle)
        .unwrap_throw();
    rectangles.move_from_to(position, rectangles.len() - 1);
}

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::exact(180))
        .s(Height::exact(180))
        .layers_signal_vec(RECTANGLES.signal_vec().map(rectangle))
}

fn rectangle(rectangle: Rectangle) -> impl Element {
    println!("render Rectangle '{rectangle:?}'");
    let (color, align) = rectangle.color_and_align();
    let lightness = color.lightness.unwrap_throw();

    let lightness_oscillator = Oscillator::new(Duration::milliseconds(200));
    let color = lightness_oscillator
        .signal()
        .map(interpolate::linear(lightness, lightness + 0.1))
        .map(move |l| color.l(l));

    El::new()
        .s(Transform::with_signal_self(
            SPREAD_OSCILLATOR
                .signal()
                .map(|spread| Transform::new().scale(100. + spread * 20.)),
        ))
        .s(Width::exact(100))
        .s(Height::exact(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(color!("Gray"))]))
        .s(Background::new().color_signal(color))
        .s(align)
        .on_hovered_change(move |is_hovered| lightness_oscillator.go_to(is_hovered))
        .on_click(move || bring_to_front(rectangle))
}
