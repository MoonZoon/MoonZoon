use std::{collections::VecDeque, iter::FromIterator};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};
use zoon::*;

// ------ ------
//    Types
// ------ ------

use LightState::*;

#[derive(Clone, Copy, IntoStaticStr, EnumIter)]
enum LightState {
    Stop,
    Ready,
    Go,
}

// ------ ------
//   States
// ------ ------

#[static_ref]
fn light_state() -> &'static Mutable<VecDeque<LightState>> {
    Mutable::new(VecDeque::from_iter(LightState::iter()))
}

// ------ ------
//   Signals
// ------ ------

fn color_attr_signal() -> impl Signal<Item = &'static str> {
    light_state().signal_ref(|light_state| match light_state[0] {
        Stop => "red",
        Ready => "yellow",
        Go => "green",
    })
}

fn cy_attr_signal() -> impl Signal<Item = &'static str> {
    light_state().signal_ref(|light_state| match light_state[0] {
        Stop => "50",
        Ready => "150",
        Go => "250",
    })
}

// ------ ------
//   Commands
// ------ ------

fn next_light_state() {
    light_state().lock_mut().rotate_left(1);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    let width = "100";
    let height = "300";

    El::new()
        .s(Align::center())
        .child(
            RawSvgEl::new("svg")
            .style("cursor", "pointer")
            .attr("width", width)
            .attr("height", height)
            .event_handler(move |_: events::Click| next_light_state())
            .child(
                RawSvgEl::new("rect")
                    .attr("width", width)
                    .attr("height", height)
                    .attr("fill", "black"),
            )
            .child(
                RawSvgEl::new("circle")
                    .attr("cx", "50")
                    .attr_signal("cy", cy_attr_signal())
                    .attr("r", "40")
                    .attr_signal("fill", color_attr_signal()),
            )
        )
}

// ------ ------
//    Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
