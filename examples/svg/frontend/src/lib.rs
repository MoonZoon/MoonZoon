use std::fmt;
use zoon::*;

#[derive(Clone, Copy)]
enum Color {
    Red,
    Blue,
    Green,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Red => "#f00",
            Self::Blue => "#0f0",
            Self::Green => "#00f",
        };
        write!(f, "{}", c)
    }
}

#[static_ref]
fn color() -> &'static Mutable<Color> {
    Mutable::new(Color::Red)
}

fn color_attr_signal() -> impl Signal<Item = Option<impl ToString>> {
    color().signal().map(Some)
}

fn change_color() {
    use Color as C;
    color().update(|c| match c {
        C::Red => C::Blue,
        C::Blue => C::Green,
        C::Green => C::Red,
    })
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
