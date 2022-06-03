use zoon::{named_color::*, *};

// ------ ------
//    States
// ------ ------

#[static_ref]
fn radius() -> &'static Mutable<u32> {
    Mutable::new(20)
}

#[static_ref]
fn radius_max() -> &'static Mutable<bool> {
    Mutable::new(false)
}

// ------ ------
//   Commands
// ------ ------

fn set_radius(new_radius: u32) {
    radius().set_neq(new_radius);
}

fn set_radius_max(is_max: bool) {
    radius_max().set_neq(is_max);
}

// ------ ------
//    Signals
// ------ ------

fn radius_signal() -> impl Signal<Item = Radius> {
    map_ref! {
        let radius = radius().signal(),
        let is_max = radius_max().signal() =>
        if *is_max {
            Radius::Max
        } else {
            Radius::Px(*radius)
        }
    }
}

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
        .item(max_panel())
}

fn rectangle() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Background::new().color(GREEN_8))
        .s(Width::exact(150))
        .s(Height::exact(150))
        .s(RoundedCorners::all_signal(radius_signal()))
}

fn rectangle_radius() -> impl Element {
    El::new().s(Align::center()).child_signal(radius().signal())
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

fn max_panel() -> impl Element {
    let checkbox_id = "max_checkbox";
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(5))
        .item(max_label(checkbox_id))
        .item(max_checkbox(checkbox_id))
}

fn max_label(checkbox_id: &str) -> impl Element {
    Label::new()
        .label("Max")
        .for_input(checkbox_id)
        .text_content_selecting(TextContentSelecting::none())
}

fn max_checkbox(checkbox_id: &str) -> impl Element {
    Checkbox::new()
        .id(checkbox_id)
        .icon(|checked| checkbox::default_icon(checked.signal()))
        .checked_signal(radius_max().signal())
        .on_change(set_radius_max)
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
}
