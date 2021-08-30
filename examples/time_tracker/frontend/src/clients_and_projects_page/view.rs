use zoon::*;
use crate::theme::Theme;
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
        .item(add_client_button())
        .item(clients())
}

fn add_client_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    El::new()
        .child(
            Button::new()
                .s(Align::center())
                .s(Background::new().color_signal(hovered_signal.map_bool(
                    || Theme::Background3Highlighted,
                    || Theme::Background3,
                )))
                .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
                .s(Padding::new().x(15).y(10))
                .s(RoundedCorners::all_fully())
                .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
                .on_press(super::add_client)
                .label("Add Client")
        )
}

fn clients() -> impl Element {
    Column::new()
        .items_signal_vec(super::clients().signal_vec_cloned().map(client))
}

fn client(client: Arc<super::Client>) -> impl Element {
    El::new()
        .child("Client")
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
