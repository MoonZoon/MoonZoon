use zoon::*;
use crate::app;
use std::ops::Not;
use hhmmss::Hhmmss;

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

        // let started_str = el_var(|| time)

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
                    duration_input(time_entry)
                ]
            }),
            row![
                started_input(),
                show_duration_row.not().then(|| {
                    column![
                        duration_input(time_entry)
                    ]
                }),
                stopped_input(),
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
    fn duration_input(time_entry: Var<super::TimeEntry>) -> TextInput {
        let started = time_entry.map(|time_entry| time_entry.started);
        let stopped = time_entry.map(|time_entry| time_entry.stopped);
        let duration: Duration = if let Some(stopped) = stopped {
            stopped - started
        } else {
            // @TODO timer
            Local::now() - started
        }
        // 3:40:20
        text_input![
            duration.hhmmss()
        ],
    }

    #[el]
    fn started_inputs(time_entry: Var<super::TimeEntry>) -> Column {
        let started = time_entry.map(|time_entry| time_entry.started);
        let formatted_started_date = || started.format("%F").to_string();
        let formatted_started_time = || started.format("%X").to_string();
        let started_date = el_ref(formatted_started_date);
        let started_time = el_ref(formatted_started_time);
        column![
            // 2020-11-03
            text_input![
                text_input::on_change(|date| started_date.set(date)),
                on_blur(|| {
                    let new_started = (|| {
                        let date = started_date.map(|date| {
                            NaiveDate::parse_from_str(&date, "%F").ok() 
                        })?;
                        let time = started.time();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_started) = new_started {
                        return super::set_time_entry_started(time_entry, started);
                    }
                    started_date.set(formatted_started_date());
                }),
                started_date.inner(),
            ],
            // 14:17:34
            text_input![
                text_input::on_change(|time| started_time.set(time)),
                on_blur(|| {
                    let new_started = (|| {
                        let time = started_time.map(|time| {
                            NaiveTime::parse_from_str(&time, "%X").ok() 
                        })?;
                        let date = started.naive_local().date();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_started) = new_started {
                        return super::set_time_entry_started(time_entry, started);
                    }
                    started_time.set(formatted_started_time());
                }),
                started_time.inner(),
            ],
        ]
    }

    #[el]
    fn stopped_inputs(time_entry: Var<super::TimeEntry>) -> Column {
        let stopped = time_entry.map(|time_entry| time_entry.stopped).unwrap_or_else(Local::now);
        let formatted_stopped_date = || stopped.format("%F").to_string();
        let formatted_stopped_time = || stopped.format("%X").to_string();
        let stopped_date = el_ref(formatted_stopped_date);
        let stopped_time = el_ref(formatted_stopped_time);
        column![
            // 2020-11-03
            text_input![
                text_input::on_change(|date| stopped_date.set(date)),
                on_blur(|| {
                    let new_stopped = (|| {
                        let date = stopped_date.map(|date| {
                            NaiveDate::parse_from_str(&date, "%F").ok() 
                        })?;
                        let time = stopped.time();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_stopped) = new_stopped {
                        return super::set_time_entry_stopped(time_entry, stopped);
                    }
                    stopped_date.set(formatted_stopped_date());
                }),
                stopped_date.inner(),
            ],
            // 14:17:34
            text_input![
                text_input::on_change(|time| stopped_time.set(time)),
                on_blur(|| {
                    let new_stopped = (|| {
                        let time = stopped_time.map(|time| {
                            NaiveTime::parse_from_str(&time, "%X").ok() 
                        })?;
                        let date = stopped.naive_local().date();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_stopped) = new_stopped {
                        return super::set_time_entry_stopped(time_entry, stopped);
                    }
                    stopped_time.set(formatted_stopped_time());
                }),
                stopped_time.inner(),
            ],
        ]
    }
    
}
