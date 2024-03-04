use crate::*;

pub fn maybe_view() -> Option<RawElOrText> {
    if STORE.logged_user.lock_ref().is_some() {
        ROUTER.replace(Route::Root);
        return None;
    }
    Some(page_content().into_raw())
}

fn page_content() -> impl Element {
    Row::new().item(name_input()).item(log_in_button())
}

fn name_input() -> impl Element {
    TextInput::new()
        .s(Padding::all(7))
        .label_hidden("Name")
        .placeholder(Placeholder::new("John"))
        .text_signal(STORE.login_page.username.signal_cloned())
        .on_change(|name| STORE.login_page.username.set(name))
}

fn log_in_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("Green"), || color!("DarkGreen"))))
        .s(Padding::all(7))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label("Log in")
        .on_press(|| {
            STORE
                .logged_user
                .set(Some(STORE.login_page.username.get_cloned()));
            ROUTER.go_to_previous_known_or_else(|| Route::Root);
        })
}
