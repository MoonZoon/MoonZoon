use std::{collections::VecDeque, iter::FromIterator};
use zoon::{
    strum::{EnumIter, IntoEnumIterator, IntoStaticStr},
    *,
};

use LightState::*;

#[derive(Clone, Copy, IntoStaticStr, EnumIter)]
#[strum(crate = "strum")]
enum LightState {
    Stop,
    Ready,
    Go,
}

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    let light_states = Mutable::new(VecDeque::from_iter(LightState::iter()));
    let width = "100";
    let height = "300";
    El::new().s(Align::center()).child(
        RawSvgEl::new("svg")
            .style("cursor", "pointer")
            .attr("width", width)
            .attr("height", height)
            .child(
                RawSvgEl::new("rect")
                    .attr("width", width)
                    .attr("height", height)
                    .attr("fill", "black"),
            )
            .child(
                RawSvgEl::new("circle")
                    .attr("cx", "50")
                    .attr_signal(
                        "cy",
                        light_states.signal_ref(|states| match states[0] {
                            Stop => "50",
                            Ready => "150",
                            Go => "250",
                        }),
                    )
                    .attr("r", "40")
                    .attr_signal(
                        "fill",
                        light_states.signal_ref(|states| match states[0] {
                            Stop => "red",
                            Ready => "yellow",
                            Go => "green",
                        }),
                    ),
            )
            .event_handler(move |_: events::Click| light_states.lock_mut().rotate_left(1)),
    )
}
