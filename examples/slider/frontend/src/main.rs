use zoon::*;

static RADIUS: Lazy<Mutable<u32>> = Lazy::new(|| Mutable::new(20));
static RADIUS_MAX_CHECKED: Lazy<Mutable<bool>> = lazy::default();

fn radius_signal() -> impl Signal<Item = Radius> {
    map_ref! {
        let radius = RADIUS.signal(),
        let radius_max_checked = RADIUS_MAX_CHECKED.signal() =>
        if *radius_max_checked {
            Radius::Max
        } else {
            Radius::Px(*radius)
        }
    }
}

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::both(20))
        .item(rectangle())
        .item(rectangle_radius())
        .item(slider())
        .item(max_panel())
}

fn rectangle() -> impl Element {
    El::new()
        .s(Align::center())
        .s(Background::new().color(color!("green")))
        .s(Width::exact(150))
        .s(Height::exact(150))
        .s(RoundedCorners::all_signal(radius_signal()))
}

fn rectangle_radius() -> impl Element {
    El::new().s(Align::center()).child_signal(RADIUS.signal())
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
        .attr_signal("value", RADIUS.signal())
        .event_handler(|event: events::Input| {
            #[allow(deprecated)]
            let value = event
                .value()
                .expect("slider value")
                .parse()
                .expect("u32 value");
            RADIUS.set_neq(value)
        })
}

fn max_panel() -> impl Element {
    let checkbox_id = "max_checkbox";
    Row::new()
        .s(Align::new().center_x())
        .s(Gap::both(5))
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
        .checked_signal(RADIUS_MAX_CHECKED.signal())
        .on_change(|checked| RADIUS_MAX_CHECKED.set_neq(checked))
}
