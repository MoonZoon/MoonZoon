use zoon::*;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .s(Background::new().color(color!("oklch(0.4 0 0)")))
        .s(Font::new().color(color!("oklch(0.8 0 0)")))
        .child(
            El::new()
                .s(Align::center())
                .s(Padding::all(30))
                .s(Width::default().max(450))
                .child_signal(async_element_function().into_signal_option()),
        )
}

async fn async_element_function() -> impl Element {
    let selected_direction = Mutable::new(Direction::Row);
    let multiline_row_enabled = Mutable::new(true);
    Stripe::new()
        .s(Align::center())
        .s(Gap::both(10))
        // .direction(Direction::Row)
        .direction_signal(selected_direction.signal())
        .multiline_row_signal(multiline_row_enabled.signal())
        .item(direction_change_button(
            Direction::Column,
            selected_direction.clone(),
        ))
        .item(direction_change_button(Direction::Row, selected_direction))
        .item(multiline_change_button(true, multiline_row_enabled.clone()))
        .item(multiline_change_button(false, multiline_row_enabled))
}

fn direction_change_button(
    direction: Direction,
    selected_direction: Mutable<Direction>,
) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().x(15).y(8))
        .s(RoundedCorners::all(8))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("Black"), || color!("Black", 0.8))))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(match direction {
            Direction::Column => "Column layout!",
            Direction::Row => "Row layout!",
        })
        .on_press(move || selected_direction.set_neq(direction))
}

fn multiline_change_button(
    enable_multiline_row: bool,
    multiline_row_enabled: Mutable<bool>,
) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().x(15).y(8))
        .s(RoundedCorners::all(8))
        .s(Background::new().color_signal(
            hovered_signal.map_bool(|| color!("DarkBlue"), || color!("DarkBlue", 0.8)),
        ))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(if enable_multiline_row {
            "Enable multiline row!"
        } else {
            "Disable multiline row!"
        })
        .on_press(move || multiline_row_enabled.set_neq(enable_multiline_row))
}
