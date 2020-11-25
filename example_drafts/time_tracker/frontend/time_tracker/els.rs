use zoon::*;
use crate::app;
use std::ops::Not;
use hhmmss::Hhmmss;

const DURATION_BREAKPOINT: f64 = 800.;

// -- Notifications --
struct RecomputeDuration;
struct RecomputeStopped;

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
        let active = time_entry.map(|time_entry| time_entry.stopped.is_none());

        if active {
            el_ref(|| Timer::new(1_000, || {
                notify(RecomputeDuration);
                notify(RecomputeStopped);
            }))
        }

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
                started_inputs(time_entry),
                show_duration_row.not().then(|| {
                    column![
                        duration_input(time_entry)
                    ]
                }),
                stopped_inputs(time_entry),
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
        let (active, started, stopped) = time_entry.map(|time_entry| (
            time_entry.stopped.is_none(),
            time_entry.started,
            time_entry.stopped.unwrap_or_else(Local::now)
        ));
        let recompute = listen_ref(|RecomputeDuration| ()).is_some();
        let duration = el_var_reset(recompute, || (stopped - started).hhmmss());
        // 3:40:20
        text_input![
            active.not().then(|| text_input::on_change(|new_duration| duration.set(new_duration))),
            active.not().then(|| on_blur(|| {
                let new_duration = (|| {
                    let duration = duration.inner();
                    let negative = duration.chars().next()? == '-';
                    if negative {
                        duration.remove(0);
                    }
                    let mut duration_parts = duration.split(':');
                    let hours: i64 = duration_parts.next()?.parse().ok()?;
                    let minutes: i64 = duration_parts.next()?.parse().ok()?;
                    let seconds: i64 = duration_parts.next()?.parse().ok()?;

                    let mut total_seconds = hours * 3600 + minutes * 60 + seconds;
                    if negative {
                        total_seconds = -total_seconds;
                    }
                    Some(Duration::seconds(total_seconds))
                })();
                if let Some(new_duration) = new_duration {
                    notify(RecomputeStopped);
                    return super::set_time_entry_stopped(time_entry, started + duration)
                }
                duration.remove()
            })),
            duration.inner()
        ]
    }

    #[el]
    fn started_inputs(time_entry: Var<super::TimeEntry>) -> Column {
        let (active, started) = time_entry.map(|time_entry| (
            time_entry.stopped.is_none(),
            time_entry.started,
        ));
        let started_date = el_ref(|| started.format("%F").to_string());
        let started_time = el_ref(|| started.format("%X").to_string());
        column![
            // 2020-11-03
            text_input![
                active.not().then(|| text_input::on_change(|date| started_date.set(date))),
                active.not().then(|| on_blur(|| {
                    let new_started = (|| {
                        let date = started_date.map(|date| {
                            NaiveDate::parse_from_str(&date, "%F").ok() 
                        })?;
                        let time = started.time();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_started) = new_started {
                        notify(RecomputeDuration);
                        return super::set_time_entry_started(time_entry, started);
                    }
                    started_date.remove();
                })),
                started_date.inner(),
            ],
            // 14:17:34
            text_input![
                active.not().then(|| text_input::on_change(|time| started_time.set(time))),
                active.not().then(|| on_blur(|| {
                    let new_started = (|| {
                        let time = started_time.map(|time| {
                            NaiveTime::parse_from_str(&time, "%X").ok() 
                        })?;
                        let date = started.naive_local().date();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_started) = new_started {
                        notify(RecomputeDuration);
                        return super::set_time_entry_started(time_entry, started);
                    }
                    started_time.remove();
                })),
                started_time.inner(),
            ],
        ]
    }

    #[el]
    fn stopped_inputs(time_entry: Var<super::TimeEntry>) -> Column {
        let (active, stopped) = time_entry.map(|time_entry| (
            time_entry.stopped.is_none(),
            time_entry.stopped.unwrap_or_else(Local::now),
        ));
        let recompute = listen_ref(|RecomputeStopped| ()).is_some();
        let stopped_date = el_ref_reset(recompute, || stopped.format("%F").to_string());
        let stopped_time = el_ref_reset(recompute, || stopped.format("%X").to_string());
        column![
            // 2020-11-03
            text_input![
                active.not().then(|| text_input::on_change(|date| stopped_date.set(date))),
                active.not().then(|| on_blur(|| {
                    let new_stopped = (|| {
                        let date = stopped_date.map(|date| {
                            NaiveDate::parse_from_str(&date, "%F").ok() 
                        })?;
                        let time = stopped.time();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_stopped) = new_stopped {
                        notify(RecomputeDuration);
                        return super::set_time_entry_stopped(time_entry, stopped);
                    }
                    stopped_date.remove();
                })),
                stopped_date.inner(),
            ],
            // 14:17:34
            text_input![
                active.not().then(|| text_input::on_change(|time| stopped_time.set(time))),
                active.not().then(|| on_blur(|| {
                    let new_stopped = (|| {
                        let time = stopped_time.map(|time| {
                            NaiveTime::parse_from_str(&time, "%X").ok() 
                        })?;
                        let date = stopped.naive_local().date();
                        Local.from_local_date(&date).and_time(time).single()
                    })();
                    if Some(new_stopped) = new_stopped {
                        notify(RecomputeDuration);
                        return super::set_time_entry_stopped(time_entry, stopped);
                    }
                    stopped_time.remove();
                })),
                stopped_time.inner(),
            ],
        ]
    }
    
}
