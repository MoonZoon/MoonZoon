use zoon::*;
use crate::{theme::Theme, app};
use std::sync::Arc;

pub fn page() -> impl Element {
    Column::new()
        .item(title())
        // .item(content())
}

fn title() -> impl Element {
    El::new()
        .s(Width::fill().max(600))
        .s(Padding::new().y(35))
        .child(
            El::with_tag(Tag::H1)
                .s(Align::center())
                .s(Font::new().size(30).weight(NamedWeight::SemiBold))
                .child("Time Blocks")
        )
}

// fn content() -> impl Element {
//     Column::new()
//         .s(Spacing::new(35))
//         .s(Padding::new().x(10).bottom(10))
//         .item(add_entity_button("Add Client", super::add_client))
//         .item(clients())
// }

// fn add_entity_button(title: &str, on_press: impl FnOnce() + Clone + 'static) -> impl Element {
//     let (hovered, hovered_signal) = Mutable::new_and_signal(false);
//     El::new()
//         .child(
//             Button::new()
//                 .s(Align::center())
//                 .s(Background::new().color_signal(hovered_signal.map_bool(
//                     || Theme::Background3Highlighted,
//                     || Theme::Background3,
//                 )))
//                 .s(Font::new().color(Theme::Font3).weight(NamedWeight::SemiBold))
//                 .s(Padding::all(5))
//                 .s(RoundedCorners::all_max())
//                 .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
//                 .on_press(on_press)
//                 .label(add_entity_button_label(title))
//         )
// }

// fn add_entity_button_label(title: &str) -> impl Element {
//     Row::new()
//     .item(app::icon_add())
//     .item(
//         El::new()
//             .s(Padding::new().right(8).bottom(1))
//             .child(title)
//     )
// }

// fn clients() -> impl Element {
//     Column::new()
//         .s(Spacing::new(35))
//         .s(Align::new().center_x())
//         .items_signal_vec(super::clients().signal_vec_cloned().map(client))
// }

// fn client(client: Arc<super::Client>) -> impl Element {
//     Column::new()
//         .s(Background::new().color(Theme::Background1))
//         .s(RoundedCorners::all(10))
//         .s(Padding::all(15))
//         .s(Spacing::new(20))
//         .item(client_name_and_delete_button(client.clone()))
//         .item(add_entity_button("Add Project", clone!((client) move || super::add_project(&client))))
//         .item(projects(client))
// }

// fn client_name_and_delete_button(client: Arc<super::Client>) -> impl Element {
//     let id = client.id;
//     Row::new()
//         .s(Spacing::new(10))
//         .item(client_name(client.clone()))
//         .item(delete_entity_button(move || super::delete_client(id)))
// }

// fn delete_entity_button(on_press: impl FnOnce() + Clone + 'static) -> impl Element {
//     let (hovered, hovered_signal) = Mutable::new_and_signal(false);
//     Button::new()
//         .s(Width::new(40))
//         .s(Height::new(40))
//         .s(Align::center())
//         .s(Background::new().color_signal(hovered_signal.map_bool(
//             || Theme::Background3Highlighted,
//             || Theme::Background3,
//         )))
//         .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
//         .s(RoundedCorners::all_max())
//         .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
//         .on_press(on_press)
//         .label(app::icon_delete_forever())
// }

// fn client_name(client: Arc<super::Client>) -> impl Element {
//     let debounced_rename = Mutable::new(None);
//     TextInput::new()
//         .s(Width::fill())
//         .s(Font::new().color(Theme::Font1).size(20))
//         .s(Background::new().color(Theme::Transparent))
//         .s(Borders::new().bottom(
//             Border::new().color(Theme::Background3)
//         ))
//         .s(Padding::all(8))
//         .focus(not(client.is_old))
//         .label_hidden("client name")
//         .text_signal(client.name.signal_cloned())
//         .on_change(move |text| {
//             client.name.set_neq(text);
//             debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
//                 super::rename_client(client.id, &client.name.lock_ref())
//             })))
//         })
// }

// fn projects(client: Arc<super::Client>) -> impl Element {
//     Column::new()
//         .s(Spacing::new(20))
//         .items_signal_vec(client.projects.signal_vec_cloned().map(move |p| {
//             project(client.clone(), p)
//         }))
// }

// fn project(client: Arc<super::Client>, project: Arc<super::Project>) -> impl Element {
//     let id = project.id;
//     Row::new()
//         .s(Background::new().color(Theme::Background0))
//         .s(RoundedCorners::new().left(10).right_max())
//         .s(Spacing::new(10))
//         .s(Padding::new().left(8))
//         .item(project_name(project.clone()))
//         .item(delete_entity_button(move || super::delete_project(&client, id)))
// }

// fn project_name(project: Arc<super::Project>) -> impl Element {
//     let debounced_rename = Mutable::new(None);
//     TextInput::new()
//         .s(Width::fill())
//         .s(Font::new().color(Theme::Font0))
//         .s(Background::new().color(Theme::Transparent))
//         .s(Borders::new().bottom(
//             Border::new().color(Theme::Background3)
//         ))
//         .s(Padding::all(5))
//         .focus(not(project.is_old))
//         .label_hidden("project name")
//         .text_signal(project.name.signal_cloned())
//         .on_change(move |text| {
//             project.name.set_neq(text);
//             debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
//                 super::rename_project(project.id, &project.name.lock_ref())
//             })))
//         })
// }






// blocks!{

//     #[el]
//     fn page() -> Column {
//         column![
//             el![
//                 region::h1(),
//                 "Time Blocks",
//             ],
//             client_panels();
//         ]
//     }

//     // ------ Client ------

//     #[el]
//     fn client_panels() -> Column {
//         let clients = super::clients().map(|clients| {
//             clients.map(|clients| clients.iter_vars().rev().map(client_panel))
//         });
//         column![
//             spacing(30),
//             clients,
//         ]
//     }

//     #[el]
//     fn client_panel(client: Var<super::Client>) -> Column {
//         let statistics = client.map(|client| client.statistics);
//         column![
//             row![
//                 el![client.map(|client| client.name.clone())],
//                 statistics(statistics),
//             ],
//             button![
//                 button::on_press(|| super::add_time_block(client)),
//                 "Add Time Block",
//             ],
//             time_block_panels(client),
//         ]
//     }

//     #[el]
//     fn statistics(statistics: Var<super::Statistics>) -> Row {
//         let statistics = statistics.inner();
//         let format = |value: f64| format!("{:.1}", value);
//         row![
//             column![
//                 row!["Blocked", format(statistics.blocked)],
//                 row!["Unpaid", format(statistics.unpaid)],
//                 row!["Paid", format(statistics.paid)],
//             ],
//             column![
//                 row!["Tracked", format(statistics.tracked)],
//                 row!["To Block", format(statistics.to_block)],
//             ],
//         ]
//     }

//     // ------ TimeBlock ------

//     #[el]
//     fn time_block_panels(client: Var<super::Client>) -> Column {
//         let time_blocks = client.map(|client| {
//             client.time_blocks.iter_vars().rev().map(time_block_panel)
//         });
//         column![
//             spacing(20),
//             time_blocks,
//         ]
//     }

//     #[el]
//     fn time_block_panel(time_block: Var<super::TimeBlock>) -> Column {
//         let invoice = time_block.map(|time_block| time_block.invoice.var());
//         column![
//             row![
//                 time_block_name(time_block),
//                 row![
//                     duration(time_block),
//                     "h",
//                 ]
//                 button![
//                     button::on_press(|| super::remove_time_block(time_block)),
//                     "D",
//                 ],
//             ],
//             row![
//                 status_switch(time_block),
//                 invoice.is_none().then(|| attach_invoice_button(time_block)),
//             ],
//             invoice.map(|invoice| {
//                 row![
//                     invoice_panel(invoice),
//                 ],
//             })
//         ]
//     }

//     #[el]
//     fn time_block_name(time_block: Var<super::TimeBlock>) -> TextInput {
//         let name = el_var(|| {
//             time_block.map(|time_block| time_block.name.clone())
//         });
//         text_input![
//             do_once(|| super::setting_clients().inner().not().then(focus)).flatten(),
//             text_input::on_change(|new_name| name.set(new_name)),
//             on_blur(|| name.use_ref(|name| {
//                 super::rename_time_block(time_block, name);
//             })),
//             name.inner(),
//         ]
//     }

//     #[el]
//     fn duration(time_block: Var<super::TimeBlock>) -> TextInput {
//         let saved_duration = || time_block.map(|time_block| {
//             format!("{:.1}", time_block.duration.num_seconds as f64 / 3600.)
//         });
//         let duration = el_var(saved_duration);
//         text_input![
//             text_input::on_change(|new_duration| duration.set(new_duration)),
//             on_blur(|| {
//                 let valid_duration = duration.map(|duration| {
//                     duration.parse::<f64>().ok().map(|duration| {
//                         Duration::seconds((duration * 3600.) as i64)
//                     })
//                 });
//                 if let Some(duration) = valid_duration {
//                     return super::set_time_block_duration(time_block, duration);
//                 }
//                 duration.set(saved_duration());
//             }),
//             duration.inner(),
//         ]
//     }

//     #[el]
//     fn status_switch(time_block: Var<super::TimeBlock>) -> Row {
//         let current_status = time_block.map(|time_block| time_block.status);

//         let button = |index: u8, text: &'static str, status: super::TimeBlockStatus| {
//             let active = status == current_status;
//             button![
//                 active.then(|| background::color(color::green)),
//                 button::on_press(|| super::set_time_block_status(time_block, status)),
//                 (index == 0).then(|| border::rounded!(left(fully()))),
//                 (index == 2).then(|| border::rounded!(right(fully()))),
//                 text,
//             ]
//         };
//         row![
//             button(0, "Non-billable", super::TimeBlockStatus::NonBillable),
//             button(1, "Unpaid", super::TimeBlockStatus::NonBillable),
//             button(2, "Paid", super::TimeBlockStatus::NonBillable),
//         ]
//     }

//     #[el]
//     fn attach_invoice_button(time_block: Var<super::TimeBlock>) -> Button {
//         button![
//             button::on_press(|| super::add_invoice(time_block)),
//             "Attach Invoice",
//         ]
//     }

//     // ------ Invoice ------

//     #[el]
//     fn invoice_panel(invoice: Var<super::Invoice>) -> Column {
//         let url = invoice.map(|invoice| invoice.url.clone());
//         column![
//             row![
//                 "Invoice ID",
//                 custom_id_input(invoice),
//                 button![
//                     button::on_press(|| super::remove_invoice(invoice)),
//                     "D",
//                 ]
//             ],
//             row![
//                 "URL",
//                 url_input(invoice),
//                 link![
//                     link::url(url),
//                     "L",
//                 ],
//             ],
//         ]
//     }

//     #[el]
//     fn custom_id_input(invoice: Var<super::Invoice>) -> TextInput {
//         let custom_id = el_var(|| invoice.map(|invoice| invoice.custom_id.clone()));
//         text_input![
//             text_input::on_change(|new_custom_id| custom_id.set(new_custom_id))
//             on_blur(|| custom_id.use_ref(|custom_id| {
//                 super::set_invoice_custom_id(invoice, custom_id);
//             })),
//             custom_id.inner(),
//         ]
//     }

//     #[el]
//     fn url_input(invoice: Var<super::Invoice>) -> TextInput {
//         let url = el_var(|| invoice.map(|invoice| invoice.url.clone()));
//         text_input![
//             text_input::on_change(|new_url| url.set(new_url))
//             on_blur(|| url.use_ref(|url| {
//                 super::set_invoice_url(invoice, url);
//             })),
//             url.inner(),
//         ]
//     }
// }
