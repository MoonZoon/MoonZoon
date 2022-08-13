use crate::{router::Route, theme};
use zoon::*;

pub fn page() -> impl Element {
    El::new().s(Height::fill().max(500)).child(
        Column::new()
            .s(Align::center())
            .s(Font::new().color_signal(theme::font_0()))
            .s(Gap::both(10))
            .item(title())
            .item(moonzoon_link())
            .item(time_tracker_link()),
    )
}

fn title() -> impl Element {
    El::new()
        .s(Font::new().size(50).weight(FontWeight::SemiBold))
        .child("Time Tracker")
}

fn moonzoon_link() -> impl Element {
    Link::new()
        .s(Font::new().size(35))
        .s(Padding::new().bottom(10))
        .to("https://moonzoon.rs")
        .label("moonzoon.rs")
}

fn time_tracker_link() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Font::new()
            .weight(FontWeight::Bold)
            .color_signal(theme::font_3())
            .size(20)
            .center())
        .s(Padding::all(12).top(10))
        .s(RoundedCorners::all(6))
        .s(
            Background::new().color_signal(hovered_signal.map_bool_signal(
                || theme::background_3_highlighted(),
                || theme::background_3(),
            )),
        )
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(Route::TimeTracker)
        .label("Go to Time Tracker")
}
