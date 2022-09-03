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

    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let (color, align) = rectangle.color_and_align();

    // let animation = global_styles().style_animation_droppable(
    global_styles().style_animation(
        StyleAnimation::new("stretch")
            .keyframe(StyleGroup::new("100%").style("transform", "scale(1.2)")),
    );

    El::new()
        // @TODO replace `animation` and styles below with the future Zoon animation API
        .update_raw_el(|raw_el| {
            raw_el
                .style("animation-name", "stretch")
                .style("animation-duration", "2.0s")
                .style("animation-timing-function", "ease-out")
                .style("animation-direction", "alternate")
                .style("animation-iteration-count", "infinite")
                .style("animation-play-state", "running")
        })
        .s(Width::exact(100))
        .s(Height::exact(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(GRAY_8)]))
        .s(Background::new().color_signal(
            hovered_signal.map_bool(move || color.update_l(|l| l + 5.), move || color),
        ))
        .s(align)
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || bring_to_front(rectangle))
    // .after_remove(move |_| drop(animation))
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
}
