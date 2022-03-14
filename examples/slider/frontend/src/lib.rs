use zoon::{*, named_color::*};

// ------ ------
//    States
// ------ ------

#[static_ref]
fn radius() -> &'static Mutable<u32> {
    Mutable::new(20)
}

// ------ ------
//   Commands
// ------ ------

fn set_radius(new_radius: u32) {
    radius().set_neq(new_radius);
}

// ------ ------
//    Signals
// ------ ------

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Spacing::new(20))
        .item(rectangle())
        .item(rectangle_radius())
        .item(slider())
}

fn rectangle() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Background::new().color(GREEN_8))
        .s(Width::new(150))
        .s(Height::new(150))
        .s(RoundedCorners::all_signal(radius().signal().map(|_| Radius::Px(50))))
}

fn rectangle_radius() -> impl Element {
    El::new()
        .s(Align::center())
        .child_signal(radius().signal())
}

fn slider() -> impl Element {
    // @TODO create Zoon element Slider; inspiration:
    // https://www.w3schools.com/howto/howto_js_rangeslider.asp
    // https://developer.mozilla.org/en-US/docs/Web/HTML/Element/input/range
    // https://package.elm-lang.org/packages/mdgriffith/elm-ui/latest/Element-Input#slider
    // https://korban.net/elm/elm-ui-patterns/slider
    RawHtmlEl::new("input")
        .attr("type", "range")
        .attr("min", "0")
        .attr("max", "75")
        .attr_signal("value", radius().signal())
        .event_handler(|event: events::Input| {
            #[allow(deprecated)]
            let value = event
                .value()
                .expect("slider value")
                .parse()
                .expect("u32 value");
            set_radius(value)
        })
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
