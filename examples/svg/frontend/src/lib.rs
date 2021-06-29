use std::{collections::VecDeque, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::*;

#[derive(Clone, Copy, IntoStaticStr, EnumIter)]
enum Color {
    Red,
    Blue,
    Green,
}

#[static_ref]
fn colors() -> &'static Mutable<VecDeque<Color>> {
    Mutable::new(VecDeque::from_iter(Color::iter()))
}

fn color_attr_signal() -> impl Signal<Item = &'static str> {
    colors().signal_ref(|colors| colors[0].into())
}

fn change_color() {
    colors().lock_mut().rotate_left(1);
}

fn root() -> impl Element {
    RawSvgEl::new("svg")
        .attr("width", "100")
        .attr("height", "100")
        .child(
            RawSvgEl::new("circle")
                .attr("cx", "50")
                .attr("cy", "50")
                .attr("r", "40")
                .attr("stroke", "black")
                .attr("stroke-width", "3")
                .attr_signal("fill", color_attr_signal())
                .event_handler(move |_: events::Click| change_color()),
        )
}

// ---------- // -----------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
