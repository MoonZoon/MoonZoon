use zoon::*;
use crate::app;
use std::ops::Not;

const DURATION_BREAKPOINT: f64 = 800.;

blocks!{

    #[el]
    fn page() -> Column {
        column![
            el![
                region::h1(),
                "Time Tracker",
            ],
            client_panels();
        ]
    }

    // ------ Client ------

    #[el]
    fn client_panels() -> Column {
        let clients = super::clients();
        column![
            spacing(30),
            clients.map(|clients| {
                clients.map(|clients| clients.iter().rev().map(client_panel))
            }),
        ]
    }

    #[el]
    fn client_panel(client: Var<super::Client>) -> Column {
        column![
            el![client.map(|client| client.name.clone())],
            project_panels(client),
        ]
    }

    // ------ Project ------

    #[el]
    fn project_panels(client: Var<super::Client>) -> Column {
        column![
            spacing(20),
            client.map(|client| {
                client.projects.iter().rev().map(project_panel)
            }),
        ]
    }

    #[el]
    fn project_panel(project: Var<super::Project>) -> Column {
        column![
            row![
                el![project.map(|project| project.name.clone())],
                start_stop_button(project),
            ],
            time_entry_panels(project),
        ]
    }

    #[el]
    fn start_stop_button(project: Var<super::Project>) -> Button {
        if let Some(time_entry) = project.map(|project| project.active_time_entry) {
            button![
                background::color(color::yellow()),
                button::on_press(|| super::set_time_entry_stopped(time_entry, Local::now())),
                "Stop",
            ]
        } else {
            button![
                background::color(color::green()),
                button::on_press(|| super::add_time_entry(project)),
                "Start",
            ]
        }
    }

    // ------ TimeEntry ------

    #[el]
    fn time_entry_panels(project: Var<super::Project>) -> Column {
        column![
            spacing(20),
            project.map(|project| {
                project.time_entries.iter().rev().map(time_entry_panel)
            }),
        ]
    }

    #[el]
    fn time_entry_panel(time_entry: Var<super::TimeEntry>) -> Column {
        let show_duration_row = app::viewport_width().inner() < DURATION_BREAKPOINT;
        column![
            row![
                time_entry_name(time_entry),
                button![
                    button::on_press(|| super::remove_time_entry(time_entry)),
                    "D",
                ],
            ],
            show_duration_row.then(|| {
                row![
                    duration(time_entry)
                ]
            }),
            row![
                started(),
                show_duration_row.not().then(|| {
                    column![
                        duration(time_entry)
                    ]
                }),
                stopped(),
            ]
        ]
    }

    #[el]
    fn time_entry_name(time_entry: Var<super::TimeEntry>) -> TextInput {
        let name = el_var(|| {
            time_entry.map(|time_entry| time_entry.name.clone())
        });
        text_input![
            text_input::on_change(|new_name| name.set(new_name)),
            on_blur(|| name.use_ref(|name| {
                super::rename_time_entry(time_entry, name);
            })),
            name.inner(),
        ]
    }

    #[el]
    fn duration(time_entry: Var<super::TimeEntry>) -> TextInput {

    }

    #[el]
    fn started(time_entry: Var<super::TimeEntry>) -> Column {
        column![

        ]
    }

    #[el]
    fn stopped(time_entry: Var<super::TimeEntry>) -> Column {
        column![
            
        ]
    }
    
}
