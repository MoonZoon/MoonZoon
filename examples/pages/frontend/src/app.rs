use zoon::*;
use crate::{router::{router, Route}, report, login};


// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Report,
    Login,
    Home,
    Unknown,
}

// ------ ------
//    Statics
// ------ ------

#[static_ref]
pub fn logged_user() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

// ------ ------
//    Helpers
// ------ ------

pub fn is_user_logged() -> bool {
    logged_user().map(Option::is_some)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

pub fn log_in(name: String) {
    logged_user().set(Some(name));
    router().go(Route::Root);
}

pub fn log_out() {
    logged_user().take();
    router().go(Route::Root);
}

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    Column::new()
        .s(Padding::new().all(20))
        .s(Spacing::new(20))
        .item(header())
        .item(page())
}

fn header() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item(
            Link::new()
                .s(Font::new().underline().color(NamedColor::Blue7))
                .label("Home")
                .to(Route::Root)
        )
        .item(
            Link::new()
                .s(Font::new().underline().color(NamedColor::Blue7))
                .label("Report")
                .to(Route::Report)
        )
        .item_signal(
            logged_user().signal_ref(|name| {
                if let Some(name) = name {
                    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
                    Button::new()
                        .s(Background::new().color_signal(hovered_signal.map_bool(|| NamedColor::Red5, || NamedColor::Red2)))
                        .s(Padding::new().x(7))
                        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
                        .label(format!("Log out {}", name))
                        .on_press(log_out)
                        .left_either()
                } else {
                    Link::new()
                        .s(Font::new().underline().color(NamedColor::Blue7))
                        .label("Log in")
                        .to(Route::Login)
                        .right_either()
                }
            })
        )
}

fn page() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Report => report::page().into_raw_element(),
        PageId::Login => login::page().into_raw_element(),
        PageId::Home => El::new().child("Welcome Home!").into_raw_element(),
        PageId::Unknown => El::new().child("404").into_raw_element(),
    }))
}
