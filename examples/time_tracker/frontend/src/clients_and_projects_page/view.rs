use zoon::*;
use crate::{theme::Theme, app};
use shared::{ClientId, ProjectId};
use std::sync::Arc;

pub fn page() -> impl Element {
    Column::new()
        .item(title())
        .item(content())
}

fn title() -> impl Element {
    El::new()
        .s(Width::fill().max(600))
        .s(Padding::new().y(35))
        .child(
            El::with_tag(Tag::H1)
                .s(Align::center())
                .s(Font::new().size(30).weight(NamedWeight::SemiBold))
                .child("Clients & Projects")
        )
}

fn content() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Padding::new().x(10).bottom(10))
        .item(add_entity_button("Add Client", super::add_client))
        .item(clients())
}

fn add_entity_button(title: &str, on_press: impl Fn() + Copy + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new()
        .child(
            Button::new()
                .s(Align::center())
                .s(Background::new().color_signal(hovered_signal.map_bool(
                    || Theme::Background3Highlighted,
                    || Theme::Background3,
                )))
                .s(Font::new().color(Theme::Font3).weight(NamedWeight::SemiBold))
                .s(Padding::all(5))
                .s(RoundedCorners::all_max())
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                .on_press(on_press)
                .label(
                    Row::new()
                        .item(app::icon_add())
                        .item(
                            El::new()
                                .s(Padding::new().right(8).bottom(1))
                                .child(title)
                        )
                )
        )
}

fn clients() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Align::new().center_x())
        .items_signal_vec(super::clients().signal_vec_cloned().map(client))
}

fn client(client: Arc<super::Client>) -> impl Element {
    let id = client.id;
    Column::new()
        .s(Background::new().color(Theme::Background1))
        .s(RoundedCorners::all(10))
        .s(Padding::all(15))
        .s(Spacing::new(20))
        .item(client_name_and_delete_button(client.clone()))
        .item(add_entity_button("Add Project", move || super::add_project(id)))
        .item(projects(client))
}

fn client_name_and_delete_button(client: Arc<super::Client>) -> impl Element {
    let id = client.id;
    Row::new()
        .s(Spacing::new(10))
        .item(client_name(client.clone()))
        .item(delete_entity_button(move || super::delete_client(id)))
}

fn delete_entity_button(on_press: impl Fn() + Copy + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new()
        .child(
            Button::new()
                .s(Width::new(40))
                .s(Height::new(40))
                .s(Align::center())
                .s(Background::new().color_signal(hovered_signal.map_bool(
                    || Theme::Background3Highlighted,
                    || Theme::Background3,
                )))
                .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
                .s(RoundedCorners::all_max())
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                .on_press(on_press)
                .label(app::icon_delete_forever())
        )
}

fn client_name(client: Arc<super::Client>) -> impl Element {
    TextInput::new()
        .s(Width::fill())
        .s(Font::new().color(Theme::Font1).size(20))
        .s(Background::new().color(Theme::Transparent))
        .s(Borders::new().bottom(
            Border::new().color(Theme::Background3)
        ))
        .s(Padding::all(8))
        .label_hidden("client name")
        .text_signal(client.name.signal_cloned())
}

fn projects(client: Arc<super::Client>) -> impl Element {
    let client_id = client.id;
    Column::new()
        .s(Spacing::new(20))
        .items_signal_vec(client.projects.signal_vec_cloned().map(move |p| {
            project(client_id, p)
        }))
}

fn project(client_id: ClientId, project: Arc<super::Project>) -> impl Element {
    let id = project.id;
    Row::new()
        .s(Background::new().color(Theme::Background0))
        .s(RoundedCorners::new().left(10).right_max())
        .s(Spacing::new(10))
        .s(Padding::new().left(8))
        .item(project_name(project.clone()))
        .item(delete_entity_button(move || super::delete_project(client_id, id)))
}

fn project_name(project: Arc<super::Project>) -> impl Element {
    TextInput::new()
        .s(Width::fill())
        .s(Font::new().color(Theme::Font0))
        .s(Background::new().color(Theme::Transparent))
        .s(Borders::new().bottom(
            Border::new().color(Theme::Background3)
        ))
        .s(Padding::all(5))
        .label_hidden("project name")
        .text_signal(project.name.signal_cloned())
}

// blocks!{

//     #[el]
//     fn page() -> Column {
//         column![
//             el![
//                 region::h1(),
//                 "Clients & Projects",
//             ],
//             button![
//                 button::on_press(super::add_client),
//                 "Add Client",
//             ];
//             client_panels(),
//         ]
//     }

//     // ------ Client ------

//     #[el]
//     fn client_panels() -> Column {
//         let clients = super::clients().map(|clients| {
//             clients.unwrap_or_default().iter_vars().rev().map(client_panel)
//         });
//         column![
//             spacing(30),
//             clients,
//         ]
//     }

//     #[el]
//     fn client_panel(client: Var<super::Client>) -> Column {
//         column![
//             row![
//                 client_name(client),
//                 button![
//                     button::on_press(|| super::remove_client(client)),
//                     "D",
//                 ],
//             ],
//             button![
//                 button::on_press(|| super::add_project(client)),
//                 "Add Project",
//             ],
//             project_panels(client),
//         ]
//     }

//     #[el]
//     fn client_name(client: Var<super::Client>) -> TextInput {
//         let name = el_var(|| client.map(|client| client.name.clone());
//         text_input![
//             do_once(|| super::setting_clients().inner().not().then(focus)).flatten(),,
//             text_input::on_change(|new_name| name.set(new_name)),
//             on_blur(|| name.use_ref(|name| {
//                 super::rename_client(client, name);
//             })),
//             name.inner(),
//         ]
//     }

//     // ------ Project ------

//     #[el]
//     fn project_panels(client: Var<super::Client>) -> Column {
//         let projects = client.map(|client| {
//             client.projects.iter_vars().rev().map(project_panel)
//         });
//         column![
//             spacing(20),
//             projects,
//         ]
//     }

//     #[el]
//     fn project_panel(project: Var<super::Project>) -> Row {
//         row![
//             project_name(project),
//             button![
//                 button::on_press(|| super::remove_project(project)),
//                 "D",
//             ],
//         ]
//     }

//     #[el]
//     fn project_name(project: Var<super::Project>) -> TextInput {
//         let name = el_var(|| project.map(|project| project.name.clone());
//         text_input![
//             do_once(|| super::setting_clients().inner().not().then(focus)).flatten(),,
//             text_input::on_change(|new_name| name.set(new_name)),
//             on_blur(|| name.use_ref(|name| {
//                 super::rename_project(project, name);
//             })),
//             name.inner(),
//         ]
//     }
// }
