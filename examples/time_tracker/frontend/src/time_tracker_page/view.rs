use zoon::*;
use crate::{theme::Theme, app};
use std::sync::Arc;

// @TODO try rewrite some clone()s to references (applies to all pages)
// @TODO refactor some parts to shared views in app.rs (applies to all pages)
// @TODO const DELETE_ENTITY_BUTTON_RADIUS in app.rs? + replace 40 / 2 with it
// @TODO dark/light theme (applies to all pages)
// @TODO fixed header? (applies to all pages)

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
                .child("Time Tracker")
        )
}

fn content() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Padding::new().x(10).bottom(10))
        .item(clients())
}

// -- clients --

fn clients() -> impl Element {
    Column::new()
        .s(Spacing::new(35))
        .s(Align::new().center_x())
        .items_signal_vec(super::clients().signal_vec_cloned().map(client))
}

fn client(client: Arc<super::Client>) -> impl Element {
    Column::new()
        .s(Background::new().color(Theme::Background1))
        .s(RoundedCorners::all(10))
        .s(Padding::all(15))
        .s(Spacing::new(20))
        .item(client_name(client.clone()))
        .item(projects(client))
}

fn client_name(client: Arc<super::Client>) -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Font::new().color(Theme::Font1).size(20))
        .s(Background::new().color(Theme::Transparent))
        .s(Padding::all(8))
        .child(&client.name)
}

// -- projects --

fn projects(client: Arc<super::Client>) -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .items(client.projects.iter().map(|p| {
            project(p.clone())
        }))
}

fn project(project: Arc<super::Project>) -> impl Element {
    Column::new()
        .s(Background::new().color(Theme::Background0))
        .s(RoundedCorners::all(10))
        .s(Spacing::new(20))
        .s(Padding::all(10))
        .item(project_name_and_start_stop_button(project.clone()))
        .item(time_entries(project))
}

fn project_name_and_start_stop_button(project: Arc<super::Project>) -> impl Element {
    Row::new()
        .item(project_name(project.clone()))
        .item(start_stop_button(project))
}

fn project_name(project: Arc<super::Project>) -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Font::new().color(Theme::Font0).size(18))
        .s(Background::new().color(Theme::Transparent))
        .s(Padding::all(8))
        .child(&project.name)
}

fn start_stop_button(project: Arc<super::Project>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background3Highlighted,
            || Theme::Background3,
        )))
        .s(Font::new().color(Theme::Font3).weight(NamedWeight::SemiBold))
        .s(RoundedCorners::all_max())
        .s(Padding::new().x(12).y(10))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || super::toggle_tracker(&project))
        .label("Start")
}

// -- time_entries --

fn time_entries(project: Arc<super::Project>) -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .items_signal_vec(project.time_entries.signal_vec_cloned().map(move |t| {
            time_entry(project.clone(), t.clone())
        }))
}

fn time_entry(project: Arc<super::Project>, time_entry: Arc<super::TimeEntry>) -> impl Element {
    Column::new()
        .s(Background::new().color(Theme::Background1))
        .s(RoundedCorners::all(10).top_right(40 / 2))
        .s(Padding::new().bottom(15))
        .item(time_entry_name_and_delete_button(project, time_entry.clone()))
        .item(time_entry_times(time_entry))
}

fn time_entry_name_and_delete_button(project: Arc<super::Project>, time_entry: Arc<super::TimeEntry>) -> impl Element {
    let id = time_entry.id;
    Row::new()
        .item(time_entry_name(time_entry.clone()))
        .item(delete_entity_button(move || super::delete_time_entry(&project, id)))
}

fn time_entry_name(time_entry: Arc<super::TimeEntry>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    El::new()
        .s(Padding::all(10))
        .child(
            TextInput::new()
            .s(Width::fill())
            .s(Font::new().color(Theme::Font1))
            .s(Background::new().color(Theme::Transparent))
            .s(Borders::new().bottom(
                Border::new().color(Theme::Background3)
            ))
            .s(Padding::all(5))
            .focus(not(time_entry.is_old))
            .label_hidden("time_entry name")
            .text_signal(time_entry.name.signal_cloned())
            .on_change(move |text| {
                time_entry.name.set_neq(text);
                debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, move || {
                    super::rename_time_entry(time_entry.id, &time_entry.name.lock_ref())
                })))
            })
        )
}

fn delete_entity_button(on_press: impl FnOnce() + Clone + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Align::new().top())
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || Theme::Background3Highlighted,
            || Theme::Background3,
        )))
        .s(Font::new().color(Theme::Font3).weight(NamedWeight::Bold))
        .s(RoundedCorners::all_max())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(on_press)
        .label(app::icon_delete_forever())
}

fn time_entry_times(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Column::new()
        .s(Font::new().color(Theme::Font1))
        .item(time_entry_started(time_entry.clone()))
        .item(time_entry_duration(time_entry.clone()))
        .item(time_entry_stopped(time_entry))
}

fn time_entry_started(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Padding::all(5))
        .item(time_entry_started_date(time_entry.clone()))
        .item(time_entry_started_time(time_entry.clone()))
}

fn time_entry_started_date(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(2))
        .item(number_input(2021, 4, false))
        .item("-")
        .item(number_input(8, 2, false))
        .item("-")
        .item(number_input(22, 2, false))
}

fn time_entry_started_time(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(2))
        .item(number_input(19, 2, false))
        .item(":")
        .item(number_input(41, 2, false))
        .item(":")
        .item(number_input(42, 2, false))
}

fn time_entry_duration(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Padding::all(5))
        .s(Spacing::new(10))
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item(number_input(0, None, true))
                .item("h"))
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item(number_input(1, 2, true))
                .item("m"))
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item(number_input(27, 2, true))
                .item("s"))
}

fn time_entry_stopped(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Padding::all(5))
        .s(Spacing::new(2))
        .item(time_entry_stopped_date(time_entry.clone()))
        .item(time_entry_stopped_time(time_entry.clone()))
}

fn time_entry_stopped_date(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(2))
        .item(number_input(2021, 4, false))
        .item("-")
        .item(number_input(8, 2, false))
        .item("-")
        .item(number_input(22, 2, false))
}

fn time_entry_stopped_time(time_entry: Arc<super::TimeEntry>) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(2))
        .item(number_input(19, 2, false))
        .item(":")
        .item(number_input(43, 2, false))
        .item(":")
        .item(number_input(7, 2, false))
}

fn number_input(number: i32, max_chars: impl Into<Option<u32>>, bold: bool) -> impl Element {
    let max_chars = max_chars.into();
    TextInput::new()
        .s(Width::zeros(max_chars.unwrap_or(4)))
        .s(
            Font::new()
                .color(Theme::Font1)
                .center()
                .weight(if bold { NamedWeight::Bold } else { NamedWeight::Regular } )
        )
        .s(Background::new().color(Theme::Transparent))
        .s(Borders::new().bottom(
            Border::new().color(Theme::Background3)
        ))
        .label_hidden("time entry started date")
        .text(number)
        .input_type(InputType::text().max_chars(max_chars))
}





// blocks!{

//     #[el]
//     fn page() -> Column {
//         column![
//             el![
//                 region::h1(),
//                 "Time Tracker",
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
//         column![
//             el![client.map(|client| client.name.clone())],
//             project_panels(client),
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
//     fn project_panel(project: Var<super::Project>) -> Column {
//         column![
//             row![
//                 el![project.map(|project| project.name.clone())],
//                 start_stop_button(project),
//             ],
//             time_entry_panels(project),
//         ]
//     }

//     #[el]
//     fn start_stop_button(project: Var<super::Project>) -> Button {
//         if let Some(time_entry) = project.map(|project| project.active_time_entry) {
//             button![
//                 background::color(color::yellow()),
//                 button::on_press(|| super::set_time_entry_stopped(time_entry, Local::now())),
//                 "Stop",
//             ]
//         } else {
//             button![
//                 background::color(color::green()),
//                 button::on_press(|| super::add_time_entry(project)),
//                 "Start",
//             ]
//         }
//     }

//     // ------ TimeEntry ------

//     #[el]
//     fn time_entry_panels(project: Var<super::Project>) -> Column {
//         let time_entries = project.map(|project| {
//             project.time_entries.iter_vars().rev().map(time_entry_panel)
//         });
//         column![
//             spacing(20),
//             time_entries,
//         ]
//     }

//     #[el]
//     fn time_entry_panel(time_entry: Var<super::TimeEntry>) -> Column {
//         let show_duration_row = app::viewport_width().inner() < DURATION_BREAKPOINT;
//         let active = time_entry.map(|time_entry| time_entry.stopped.is_none());

//         if active {
//             el_var(|| Timer::new(1_000, || {
//                 notify(RecomputeDuration);
//                 notify(RecomputeStopped);
//             }))
//         }

//         column![
//             row![
//                 time_entry_name(time_entry),
//                 button![
//                     button::on_press(|| super::remove_time_entry(time_entry)),
//                     "D",
//                 ],
//             ],
//             show_duration_row.then(|| {
//                 row![
//                     duration_input(time_entry)
//                 ]
//             }),
//             row![
//                 started_inputs(time_entry),
//                 show_duration_row.not().then(|| {
//                     column![
//                         duration_input(time_entry)
//                     ]
//                 }),
//                 stopped_inputs(time_entry),
//             ]
//         ]
//     }

//     #[el]
//     fn time_entry_name(time_entry: Var<super::TimeEntry>) -> TextInput {
//         let name = el_var(|| {
//             time_entry.map(|time_entry| time_entry.name.clone())
//         });
//         text_input![
//             text_input::on_change(|new_name| name.set(new_name)),
//             on_blur(|| name.use_ref(|name| {
//                 super::rename_time_entry(time_entry, name);
//             })),
//             name.inner(),
//         ]
//     }

//     #[el]
//     fn duration_input(time_entry: Var<super::TimeEntry>) -> TextInput {
//         let (active, started, stopped) = time_entry.map(|time_entry| (
//             time_entry.stopped.is_none(),
//             time_entry.started,
//             time_entry.stopped.unwrap_or_else(Local::now)
//         ));
//         let recompute = listen_ref(|RecomputeDuration| ()).is_some();
//         let duration = el_var_reset(recompute, || (stopped - started).hhmmss());
//         // 3:40:20
//         text_input![
//             active.not().then(|| text_input::on_change(|new_duration| duration.set(new_duration))),
//             active.not().then(|| on_blur(|| {
//                 let new_duration = (|| {
//                     let duration = duration.inner();
//                     let negative = duration.chars().next()? == '-';
//                     if negative {
//                         duration.remove(0);
//                     }
//                     let mut duration_parts = duration.split(':');
//                     let hours: i64 = duration_parts.next()?.parse().ok()?;
//                     let minutes: i64 = duration_parts.next()?.parse().ok()?;
//                     let seconds: i64 = duration_parts.next()?.parse().ok()?;

//                     let mut total_seconds = hours * 3600 + minutes * 60 + seconds;
//                     if negative {
//                         total_seconds = -total_seconds;
//                     }
//                     Some(Duration::seconds(total_seconds))
//                 })();
//                 if let Some(new_duration) = new_duration {
//                     notify(RecomputeStopped);
//                     return super::set_time_entry_stopped(time_entry, started + duration)
//                 }
//                 duration.remove()
//             })),
//             duration.inner()
//         ]
//     }

//     #[el]
//     fn started_inputs(time_entry: Var<super::TimeEntry>) -> Column {
//         let (active, started) = time_entry.map(|time_entry| (
//             time_entry.stopped.is_none(),
//             time_entry.started,
//         ));
//         let started_date = el_var(|| started.format("%F").to_string());
//         let started_time = el_var(|| started.format("%X").to_string());
//         column![
//             // 2020-11-03
//             text_input![
//                 active.not().then(|| text_input::on_change(|date| started_date.set(date))),
//                 active.not().then(|| on_blur(|| {
//                     let new_started = (|| {
//                         let date = started_date.map(|date| {
//                             NaiveDate::parse_from_str(&date, "%F").ok() 
//                         })?;
//                         let time = started.time();
//                         Local.from_local_date(&date).and_time(time).single()
//                     })();
//                     if Some(new_started) = new_started {
//                         notify(RecomputeDuration);
//                         return super::set_time_entry_started(time_entry, started);
//                     }
//                     started_date.remove();
//                 })),
//                 started_date.inner(),
//             ],
//             // 14:17:34
//             text_input![
//                 active.not().then(|| text_input::on_change(|time| started_time.set(time))),
//                 active.not().then(|| on_blur(|| {
//                     let new_started = (|| {
//                         let time = started_time.map(|time| {
//                             NaiveTime::parse_from_str(&time, "%X").ok() 
//                         })?;
//                         let date = started.naive_local().date();
//                         Local.from_local_date(&date).and_time(time).single()
//                     })();
//                     if Some(new_started) = new_started {
//                         notify(RecomputeDuration);
//                         return super::set_time_entry_started(time_entry, started);
//                     }
//                     started_time.remove();
//                 })),
//                 started_time.inner(),
//             ],
//         ]
//     }

//     #[el]
//     fn stopped_inputs(time_entry: Var<super::TimeEntry>) -> Column {
//         let (active, stopped) = time_entry.map(|time_entry| (
//             time_entry.stopped.is_none(),
//             time_entry.stopped.unwrap_or_else(Local::now),
//         ));
//         let recompute = listen_ref(|RecomputeStopped| ()).is_some();
//         let stopped_date = el_var_reset(recompute, || stopped.format("%F").to_string());
//         let stopped_time = el_var_reset(recompute, || stopped.format("%X").to_string());
//         column![
//             // 2020-11-03
//             text_input![
//                 active.not().then(|| text_input::on_change(|date| stopped_date.set(date))),
//                 active.not().then(|| on_blur(|| {
//                     let new_stopped = (|| {
//                         let date = stopped_date.map(|date| {
//                             NaiveDate::parse_from_str(&date, "%F").ok() 
//                         })?;
//                         let time = stopped.time();
//                         Local.from_local_date(&date).and_time(time).single()
//                     })();
//                     if Some(new_stopped) = new_stopped {
//                         notify(RecomputeDuration);
//                         return super::set_time_entry_stopped(time_entry, stopped);
//                     }
//                     stopped_date.remove();
//                 })),
//                 stopped_date.inner(),
//             ],
//             // 14:17:34
//             text_input![
//                 active.not().then(|| text_input::on_change(|time| stopped_time.set(time))),
//                 active.not().then(|| on_blur(|| {
//                     let new_stopped = (|| {
//                         let time = stopped_time.map(|time| {
//                             NaiveTime::parse_from_str(&time, "%X").ok() 
//                         })?;
//                         let date = stopped.naive_local().date();
//                         Local.from_local_date(&date).and_time(time).single()
//                     })();
//                     if Some(new_stopped) = new_stopped {
//                         notify(RecomputeDuration);
//                         return super::set_time_entry_stopped(time_entry, stopped);
//                     }
//                     stopped_time.remove();
//                 })),
//                 stopped_time.inner(),
//             ],
//         ]
//     }
    
// }
