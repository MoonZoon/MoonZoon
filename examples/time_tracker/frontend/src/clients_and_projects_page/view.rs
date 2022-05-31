use crate::{app, theme};
use std::sync::Arc;
use zoon::*;

pub fn page() -> impl Element {
    Column::new().item(title()).item(content())
}

fn title() -> impl Element {
    El::with_tag(Tag::H1)
        .s(Padding::new().y(35))
        .s(Align::center())
        .s(Font::new()
            .size(30)
            .weight(FontWeight::SemiBold)
            .color_signal(theme::font_0()))
        .child("Clients & Projects")
}

fn content() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Padding::new().x(10).bottom(10))
        .item(add_entity_button("Add Client", super::add_client))
        .item(clients())
}

fn add_entity_button(title: &str, on_press: impl FnMut() + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new().child(
        Button::new()
            .s(Align::center())
            .s(
                Background::new().color_signal(hovered_signal.map_bool_signal(
                    || theme::background_3_highlighted(),
                    || theme::background_3(),
                )),
            )
            .s(Font::new()
                .color_signal(theme::font_3())
                .weight(FontWeight::SemiBold))
            .s(Padding::all(5))
            .s(RoundedCorners::all_max())
            .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
            .on_press(on_press)
            .label(add_entity_button_label(title)),
    )
}

fn add_entity_button_label(title: &str) -> impl Element {
    Row::new()
        .item(app::icon_add())
        .item(El::new().s(Padding::new().right(8).bottom(1)).child(title))
}

fn clients() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Align::new().center_x())
        .items_signal_vec(super::clients().signal_vec_cloned().map(client))
}

fn client(client: Arc<super::Client>) -> impl Element {
    Column::new()
        .s(Background::new().color_signal(theme::background_1()))
        .s(RoundedCorners::all(10))
        .s(Padding::all(15))
        .s(Spacing::new(20))
        .item(client_name_and_delete_button(client.clone()))
        .item(add_entity_button(
            "Add Project",
            clone!((client) move || super::add_project(&client)),
        ))
        .item(projects(client))
}

fn client_name_and_delete_button(client: Arc<super::Client>) -> impl Element {
    let id = client.id;
    Row::new()
        .s(Spacing::new(10))
        .item(client_name(client.clone()))
        .item(delete_entity_button(move || super::delete_client(id)))
}

fn delete_entity_button(on_press: impl FnMut() + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::exact(40))
        .s(Height::exact(40))
        .s(Align::center())
        .s(
            Background::new().color_signal(hovered_signal.map_bool_signal(
                || theme::background_3_highlighted(),
                || theme::background_3(),
            )),
        )
        .s(Font::new()
            .color_signal(theme::font_3())
            .weight(FontWeight::Bold))
        .s(RoundedCorners::all_max())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(on_press)
        .label(app::icon_delete_forever())
}

fn client_name(client: Arc<super::Client>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    TextInput::new()
        .s(Width::fill())
        .s(Font::new().color_signal(theme::font_1()).size(20))
        .s(Background::new().color_signal(theme::transparent()))
        .s(Borders::new().bottom_signal(theme::border_1().map(|color| Border::new().color(color))))
        .s(Padding::all(8))
        .focus(not(client.is_old))
        .label_hidden("client name")
        .text_signal(client.name.signal_cloned())
        .on_change(move |text| {
            client.name.set_neq(text);
            debounced_rename.set(Some(Timer::once(
                app::DEBOUNCE_MS,
                clone!((client) move || {
                    super::rename_client(client.id, &client.name.lock_ref())
                }),
            )))
        })
}

fn projects(client: Arc<super::Client>) -> impl Element {
    Column::new().s(Spacing::new(20)).items_signal_vec(
        client
            .projects
            .signal_vec_cloned()
            .map(move |p| project(client.clone(), p)),
    )
}

fn project(client: Arc<super::Client>, project: Arc<super::Project>) -> impl Element {
    let id = project.id;
    Row::new()
        .s(Background::new().color_signal(theme::background_0()))
        .s(RoundedCorners::new().left(10).right_max())
        .s(Spacing::new(10))
        .s(Padding::new().left(8))
        .item(project_name(project.clone()))
        .item(delete_entity_button(move || {
            super::delete_project(&client, id)
        }))
}

fn project_name(project: Arc<super::Project>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    TextInput::new()
        .s(Width::fill())
        .s(Font::new().color_signal(theme::font_0()))
        .s(Background::new().color_signal(theme::transparent()))
        .s(Borders::new().bottom_signal(theme::border_1().map(|color| Border::new().color(color))))
        .s(Padding::all(5))
        .focus(not(project.is_old))
        .label_hidden("project name")
        .text_signal(project.name.signal_cloned())
        .on_change(move |text| {
            project.name.set_neq(text);
            debounced_rename.set(Some(Timer::once(
                app::DEBOUNCE_MS,
                clone!((project) move || {
                    super::rename_project(project.id, &project.name.lock_ref())
                }),
            )))
        })
}
