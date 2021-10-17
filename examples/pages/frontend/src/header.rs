use crate::{app, router::Route};
use zoon::{*, named_color::*};

// ------ ------
//     View
// ------ ------

pub fn header() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item(back_button())
        .item(link("Home", Route::Root))
        .item(link("Report", Route::ReportRoot))
        .item(link("Calc", Route::CalcRoot))
        .item_signal(app::logged_user().signal_ref(|name| {
            if let Some(name) = name {
                log_out_button(name).left_either()
            } else {
                link("Log in", Route::Login).right_either()
            }
        }))
}

fn back_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
        .s(Padding::new().x(7).y(4))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("< Back")
        .on_press(routing::back)
}

fn link(label: &str, route: Route) -> impl Element {
    Link::new()
        .s(Font::new().underline().color(BLUE_4))
        .label(label)
        .to(route)
}

fn log_out_button(name: &str) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| RED_7, || RED_8)))
        .s(Padding::new().x(7).y(4))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(format!("Log out {}", name))
        .on_press(app::log_out)
}
