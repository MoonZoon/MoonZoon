use crate::theme;
use zoon::*;

pub fn page() -> impl Element {
    Column::new()
        .s(Height::fill().max(500))
        .s(Padding::new().y(10))
        .item(
            Column::new()
                .s(Background::new().color_signal(theme::background_1()))
                .s(Align::center())
                .s(Font::new().color_signal(theme::font_1()))
                .s(Gap::both(10))
                .s(Padding::all(30))
                .s(Gap::both(20))
                .s(RoundedCorners::all(25))
                .item(title())
                .item(login_fields())
                .item(error())
                .item(login_button()),
        )
}

fn title() -> impl Element {
    El::new()
        .s(Align::new().center_x())
        .s(Font::new().size(25).weight(FontWeight::SemiBold))
        .child("Login")
}

fn login_fields() -> impl Element {
    Column::new()
        .s(Gap::both(15))
        .item(name_field())
        .item(password_field())
}

fn name_field() -> impl Element {
    Column::new()
        .s(Gap::both(5))
        .item(name_label())
        .item(name_input())
}

fn name_label() -> impl Element {
    Label::new().for_input("name").label("Name")
}

fn name_input() -> impl Element {
    TextInput::new()
        .id("name")
        .s(Padding::all(5))
        .s(RoundedCorners::all(4))
        .s(Font::new().color_signal(theme::font_0()))
        .s(Background::new().color_signal(theme::background_0()))
        .focus(true)
        .on_change(super::set_name)
        .text_signal(super::name().signal_cloned())
        .on_key_down_event(|event| event.if_key(Key::Enter, super::log_in))
        .placeholder(Placeholder::new("john@example.com"))
}

fn password_field() -> impl Element {
    Column::new()
        .s(Gap::both(5))
        .item(password_label())
        .item(password_input())
}

fn password_label() -> impl Element {
    Label::new().for_input("password").label("Password")
}

fn password_input() -> impl Element {
    TextInput::new()
        .id("password")
        .s(Padding::all(5))
        .s(RoundedCorners::all(4))
        .s(Font::new().color_signal(theme::font_0()))
        .s(Background::new().color_signal(theme::background_0()))
        .on_change(super::set_password)
        .text_signal(super::password().signal_cloned())
        .on_key_down_event(|event| event.if_key(Key::Enter, super::log_in))
        .placeholder(Placeholder::new("Password1"))
        .input_type(InputType::password())
}

fn error() -> impl Element {
    El::new().child_signal(super::login_error().signal_cloned())
}

fn login_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(
            Background::new().color_signal(hovered_signal.map_bool_signal(
                || theme::background_3_highlighted(),
                || theme::background_3(),
            )),
        )
        .s(Font::new()
            .color_signal(theme::font_3())
            .weight(FontWeight::Bold))
        .s(Padding::new().x(15).y(10))
        .s(RoundedCorners::all(4))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(super::log_in)
        .label("Log in")
}
