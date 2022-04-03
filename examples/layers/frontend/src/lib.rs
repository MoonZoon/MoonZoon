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

// @TODO remove PartialEq + Eq
#[derive(Clone, Copy, PartialEq, Eq)]
enum Breath {
    Out,
    In,
}

#[static_ref]
fn breathing_timeline() -> &'static Timeline<Breath> {
    Task::start(async {
        breathing_timeline()
            .arrived_signal()
            // @TODO remove dedupe once fixed in Zoon
            .dedupe()
            .for_each_sync(|arrived| {
                let next_state = if matches!(arrived, Breath::Out) {
                    // println!("IN");
                    Breath::In
                } else {
                    // println!("OUT");
                    Breath::Out
                };
                breathing_timeline().push(Duration::seconds(2), next_state);
            }).await;
    });
    Timeline::new(Breath::Out)
}

fn breathing_oscillator() -> impl Signal<Item = f64> {
    breathing_timeline().oscillator(|breath| match breath {
        Breath::Out => 100.,
        Breath::In => 120.,
    })
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
        .s(Width::new(180))
        .s(Height::new(180))
        .layers_signal_vec(rectangles().signal_vec().map(rectangle))
}

fn rectangle(rectangle: Rectangle) -> impl Element {
    println!("render Rectangle '{rectangle:?}'");

    let (color, align) = rectangle.color_and_align();

    let hover_timeline = Timeline::new(false);
    let lightness_oscillator = hover_timeline.oscillator(move |hovered| {
        if *hovered {
            color.l() + 5.
        } else {
            color.l()
        }
    });

    El::new()
        .s(Transform::with_signal(
            breathing_oscillator()
                .map(|percent| Transform::new().scale(percent)),
        ))
        .s(Width::new(100))
        .s(Height::new(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(GRAY_8)]))
        .s(Background::new()
            .color_signal(lightness_oscillator.map(move |l| color.set_l(l))))
        .s(align)
        .on_hovered_change(move |is_hovered| {
            // @TODO push -> to ; replace `current`? move `current` to `arrived`?
            hover_timeline.push(Duration::milliseconds(200), is_hovered);
        })
        .on_click(move || bring_to_front(rectangle))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
