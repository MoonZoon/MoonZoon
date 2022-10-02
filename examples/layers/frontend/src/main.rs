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

#[derive(Clone, Copy)]
enum Breath {
    In,
    Out,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn rectangles() -> &'static MutableVec<Rectangle> {
    MutableVec::new_with_values(Rectangle::iter().collect())
}

#[static_ref]
fn breathing_timeline() -> &'static Timeline<Breath> {
    let timeline = Timeline::new(Breath::Out);
    Task::start(
        timeline
            .previous_signal_ref(|breath| *breath)
            .for_each_sync(clone!((timeline) move |previous_state| {
                let next_state = if matches!(previous_state, Breath::Out) {
                    Breath::In
                } else {
                    Breath::Out
                };
                timeline.push(Duration::seconds(2), next_state);
            })),
    );
    timeline
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
//    Signals
// ------ ------

fn breathing_animation() -> impl Signal<Item = f64> {
    breathing_timeline().linear_animation(|breath| match breath {
        Breath::Out => 100.,
        Breath::In => 120.,
    })
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

    let hover_timeline = Timeline::new(false);
    let lightness_animation =
        hover_timeline.linear_animation(
            move |hovered| {
                if *hovered {
                    color.l() + 5.
                } else {
                    color.l()
                }
            },
        );

    El::new()
        .s(Transform::with_signal(
            breathing_animation().map(|percent| Transform::new().scale(percent)),
        ))
        .s(Width::exact(100))
        .s(Height::exact(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(GRAY_8)]))
        .s(Background::new().color_signal(lightness_animation.map(move |l| color.set_l(l))))
        .s(align)
        .on_hovered_change(move |is_hovered| {
            hover_timeline.push(Duration::milliseconds(200), is_hovered)
        })
        .on_click(move || bring_to_front(rectangle))
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
}
