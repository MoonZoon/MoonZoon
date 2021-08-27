use zoon::*;
use crate::{router::Route, theme::Theme};

pub fn root() -> impl Element {
    Column::new()
        .s(Height::fill().min_screen())
        .on_viewport_size_change(super::on_viewport_size_change)
        .item(header())
        .item_signal(super::page_id().signal().map(page))
}

fn header() -> impl Element {
    Row::with_tag(Tag::Nav)
        .s(Height::new(64))
        .s(Background::new().color(Theme::Background1))
        .s(Font::new().color(Theme::Font1))
        .item(logo())
        .item_signal(super::wide_screen().map_true(|| {
            Row::new().s(Height::fill()).items(menu_links(false))
        }))
        .item_signal(super::saving().signal().map_true(|| "Saving..."))
        .item_signal(super::wide_screen().map_true(auth_controls))
        .item_signal(super::wide_screen().map_false(hamburger))
        .element_below(El::new().child_signal(super::show_menu_panel().map_true(menu_panel)))
}

fn logo() -> impl Element {
    Link::new()
        .s(Height::fill())
        .s(Font::new().weight(NamedWeight::Bold).size(32))
        .s(Padding::new().x(12))
        .to(Route::Root)
        .label(Row::new().s(Height::fill()).item("TT"))
}

fn hamburger() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Height::fill())
        .s(Align::new().right())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background1Highlighted,
            || Theme::Transparent,
        )))
        .s(Font::new().size(25))
        .s(Padding::new().bottom(4))
        .s(Width::new(64))
        .on_press(super::toggle_menu)
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(
            Row::new()
                .s(Height::fill())
                .item_signal(super::menu_opened().signal().map_bool(|| "✕", || "☰"))
        )
        .class_id(super::set_hamburger_class_id)
}

fn menu_panel() -> impl Element {
    Column::new()
        .s(Background::new().color(Theme::Background0))
        .s(Font::new().color(Theme::Font0))
        .s(Height::new(250))
        .s(Align::new().right())
        .s(Padding::all(15))
        .s(Shadows::new(vec![
            Shadow::new().y(8).blur(16).color(Theme::Shadow)
        ]))
        .s(RoundedCorners::new().bottom(10))
        .on_click_outside(
            super::close_menu, 
            Some(super::hamburger_class_id().get_cloned())
        )
        .after_remove(|_| super::close_menu())
        .items(menu_links(true))
        .item(El::new().s(Height::new(10)))
        .item(auth_controls())
}

fn menu_links(in_menu_panel: bool) -> Vec<impl Element> {
    vec![
        menu_link(Route::TimeTracker, "Time Tracker", super::PageId::TimeTracker, in_menu_panel),
        menu_link(Route::ClientsAndProjects, "Clients & Projects", super::PageId::ClientsAndProjects, in_menu_panel),
        menu_link(Route::TimeBlocks, "Time Blocks", super::PageId::TimeBlocks, in_menu_panel),
    ]
}

fn menu_link(route: Route, label: &str, page_id: super::PageId, in_menu_panel: bool) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_or_selected = map_ref! {
        let hovered = hovered_signal,
        let current_page_id = super::page_id().signal() => move {
            *hovered || *current_page_id == page_id
        }
    };
    Link::new()
        .s(Height::fill())
        .s(Padding::new().x(12))
        .s(Background::new().color_signal(hovered_or_selected.map_bool(
            move || if in_menu_panel { Theme::Background2Highlighted } else { Theme::Background1Highlighted },
            || Theme::Transparent,
        )))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(route)
        .label(Row::new().s(Height::fill()).item(label))
}

fn auth_controls() -> impl Element {
    Row::new()
        .s(Align::new().right())
        .s(Padding::new().x(12))
        .item_signal(super::is_user_logged_signal().map_false(login_button))
        .item_signal(super::is_user_logged_signal().map_true(logout_button))
}

fn login_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Link::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background3Highlighted,
            || Theme::Background3,
        )))
        .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
        .s(Padding::new().x(15).y(10))
        .s(RoundedCorners::all(4))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(Route::Login)
        .label("Log in")
}

fn logout_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background2Highlighted,
            || Theme::Background2,
        )))
        .s(Font::new().color(Theme::Font2))
        .s(Padding::new().x(15).y(10))
        .s(RoundedCorners::all(4))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(super::log_out)
        .label(
            Row::new()
                .item(El::new().s(Font::new().weight(NamedWeight::SemiBold)).child("Log out "))
                .item(super::logged_user_name())
        )
}

fn page(page_id: super::PageId) -> impl Element {
    match page_id {
        super::PageId::Login => crate::login_page::view(),
        super::PageId::ClientsAndProjects => crate::clients_and_projects_page::view(),
        super::PageId::TimeTracker => crate::time_tracker_page::view(),
        super::PageId::TimeBlocks => crate::time_blocks_page::view(),
        super::PageId::Home => crate::home_page::view(),
        super::PageId::Unknown => El::new().child(404).into_raw_element(),
    }
}
