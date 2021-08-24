use zoon::*;
use crate::router::Route;
// use crate::{login_page, }

pub fn root() -> impl Element {
    El::new()
        .on_viewport_size_change(super::on_viewport_size_change)
        // .on_click(super::view_clicked)
        .child(
            Column::new()
                .item(header())
                .item_signal(super::show_menu_panel().map_true(menu_panel))
                .item(page())
        )
}

fn header() -> impl Element {
    Row::new()
        .item(
            El::new()
                .s(Font::new().weight(NamedWeight::Bold))
                .child("TT")
        )
        .item_signal(super::wide_screen().map_true(|| {
            Row::new().items(menu_links())
        }))
        .item_signal(super::saving().signal().map_true(|| "Saving..."))
        .item_signal(super::wide_screen().map_true(auth_controls))
        .item_signal(super::wide_screen().map_false(hamburger))
}

fn hamburger() -> impl Element {
    Button::new()
        .on_press(super::toggle_menu)
        // .on_click(super::menu_part_clicked)
        .label_signal(super::menu_opened().signal().map_bool(|| "X", || "☰"))
}

fn menu_panel() -> impl Element {
    Column::new()
        // .on_click(super::menu_part_clicked)
        .items(menu_links())
        .item(auth_controls())
}

fn menu_links() -> Vec<impl Element> {
    vec![
        Link::new().to(Route::TimeTracker).label("Timer Tracker"),
        Link::new().to(Route::ClientsAndProjects).label("Clients & Projects"),
        Link::new().to(Route::TimeBlocks).label("Timer Blocks"),
    ]
}

fn auth_controls() -> impl Element {
    Row::new()
        .item_signal(super::logged_user().signal_cloned())
        .item_signal(super::is_user_logged_signal().map_false(login_button))
        .item_signal(super::is_user_logged_signal().map_true(logout_button))
}

fn login_button() -> impl Element {
    Link::new()
        .to(Route::Login)
        .label("Log in")
}

fn logout_button() -> impl Element {
    Button::new()
        .on_press(super::log_out)
        .label("Log out")
}

fn page() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .child_signal(super::page_id().signal().map(|page_id| match page_id {
            super::PageId::Login => crate::login_page::view::page().into_raw_element(),
            super::PageId::ClientsAndProjects => crate::clients_and_projects_page::view::page().into_raw_element(),
            super::PageId::TimeTracker => crate::time_tracker_page::view::page().into_raw_element(),
            super::PageId::TimeBlocks => crate::time_blocks_page::view::page().into_raw_element(),
            super::PageId::Home => crate::home_page::view::page().into_raw_element(),
            super::PageId::Unknown => El::new().child(404).into_raw_element(),
        }))
}
