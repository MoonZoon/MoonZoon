use crate::app;
use zoon::*;

// ------ ------
//    Statics
// ------ ------

#[static_ref]
fn name() -> &'static Mutable<String> {
    Mutable::new("John".to_owned())
}

// ------ ------
//   Commands
// ------ ------

fn set_name(new_name: String) {
    name().set(new_name);
}

fn log_in() {
    app::log_in(name().get_cloned());
}

// ------ ------
//     View
// ------ ------

pub fn page() -> impl Element {
    Row::new().item(name_input()).item(log_in_button())
}

fn name_input() -> impl Element {
    TextInput::new()
        .s(Padding::new().all(7))
        .label_hidden("Name")
        .placeholder(Placeholder::new("John"))
        .text(name().get_cloned())
        .on_change(set_name)
}

fn log_in_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| NamedColor::Green5, || NamedColor::Green2)))
        .s(Padding::new().all(7))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Log in")
        .on_press(log_in)
}
