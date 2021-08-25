use zoon::*;
use crate::{theme::Theme, router::Route};

pub fn page() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Font::new().color(Theme::Font0))
        .item(title())
        .item(moonzoon_link())
        .item(time_tracker_link())
}

fn title() -> impl Element {
    El::new()
        .s(Font::new().size(50).weight(NamedWeight::SemiBold))
        .child("Time Tracker")
}

fn moonzoon_link() -> impl Element {
    Link::new()
        .s(Font::new().size(35))
        .to("https://moonzoon.rs")
        .label("moonzoon.rs")
}

fn time_tracker_link() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Font::new().weight(NamedWeight::Bold).color(Theme::Font3).size(20).center())
        .s(Padding::all(12))
        .s(RoundedCorners::all(6))
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background3Highlighted,
            || Theme::Background3,
        )))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(Route::TimeTracker)
        .label("Go to Time Tracker")
}
