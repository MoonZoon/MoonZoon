use zoon::*;
use crate::{theme, app};
use std::{sync::Arc, convert::TryFrom};

pub fn page() -> impl Element {
    Column::new()
        .item(title())
        .item(content())
}

fn title() -> impl Element {
    El::with_tag(Tag::H1)
        .s(Padding::new().y(35))
        .s(Align::center())
        .s(
            Font::new()
                .size(30)
                .weight(FontWeight::SemiBold)
                .color_signal(theme::font_0())
        )
        .child("Time Tracker")
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
        .s(Background::new().color_signal(theme::background_1()))
        .s(RoundedCorners::all(10))
        .s(Padding::all(15))
        .s(Spacing::new(20))
        .item(client_name(client.clone()))
        .item(projects(client))
}

fn client_name(client: Arc<super::Client>) -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Font::new().color_signal(theme::font_1()).size(20))
        .s(Background::new().color_signal(theme::transparent()))
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
        .s(Background::new().color_signal(theme::background_0()))
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
        .s(Font::new().color_signal(theme::font_0()).size(18))
        .s(Background::new().color_signal(theme::transparent()))
        .s(Padding::all(8))
        .child(&project.name)
}

fn start_stop_button(project: Arc<super::Project>) -> impl Element {
    let mutable_has_active_entry = Mutable::new(false);
    let has_active_entry = mutable_has_active_entry.read_only();
    let has_active_entry_updater = Task::start_droppable(
        project
            .time_entries
            .signal_vec_cloned()
            .filter_signal_cloned(|time_entry| {
                time_entry.stopped.signal().map(|stopped| stopped.is_none())
            })
            .len()
            .map(|active_entries_count| active_entries_count > 0)
            .for_each_sync(move |has_active_entry| {
                mutable_has_active_entry.set_neq(has_active_entry);
            })
    );

    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let background_color = map_ref! {
        let hovered = hovered_signal,
        let has_active_entry = has_active_entry.signal() =>
        match (has_active_entry, hovered) {
            (true, false) => theme::background_4().boxed_local(),
            (true, true) => theme::background_4_highlighted().boxed_local(),
            (false, false) => theme::background_3().boxed_local(),
            (false, true) => theme::background_3_highlighted().boxed_local(),
        }
    }.flatten();

    Button::new()
        .s(Background::new().color_signal(background_color))
        .s(Font::new().color_signal(has_active_entry.signal().map_bool_signal(
            || theme::font_4(), 
            || theme::font_3(),
        )))
        .s(RoundedCorners::all_max())
        .s(Padding::new().x(20).y(10))
        .after_remove(move |_| drop(has_active_entry_updater))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || super::toggle_tracker(&project))
        .label_signal(has_active_entry.signal().map_bool(|| "Stop", || "Start"))
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
    let debounced_started = Mutable::new(None);
    let debounced_started_first_value = Mutable::new(true);
    let debounced_started_updater = Task::start_droppable(time_entry
        .started
        .signal()
        .dedupe()
        .for_each_sync(clone!((time_entry) move |started| {
            if debounced_started_first_value.get() {
                debounced_started_first_value.set(false);
            } else {
                debounced_started.set(Some(Timer::once(app::DEBOUNCE_MS, clone!((time_entry) move || {
                    super::set_time_entry_started(&time_entry, started.into())
                }))));
            }
        }))
    );

    let debounced_stopped = Mutable::new(None);
    let debounced_stopped_first_value = Mutable::new(true);
    let debounced_stopped_updater = Task::start_droppable(time_entry
        .stopped
        .signal()
        .dedupe()
        .for_each_sync(clone!((time_entry) move |stopped| {
            if debounced_stopped_first_value.get() {
                debounced_stopped_first_value.set(false);
            } else {
                debounced_stopped.set(Some(Timer::once(app::DEBOUNCE_MS, clone!((time_entry) move || {
                    super::set_time_entry_stopped(&time_entry, stopped.unwrap_throw().into())
                }))));
            }
        }))
    );

    let is_active_mutable = Mutable::new(false);
    let is_active = is_active_mutable.read_only();
    let is_active_updater = Task::start_droppable(time_entry
        .stopped
        .signal()
        .for_each_sync(move |stopped| {
            is_active_mutable.set_neq(stopped.is_none());
        })
    );
    Column::new()
        .s(Background::new().color_signal(is_active.signal().map_bool_signal(
            || theme::background_4(), 
            || theme::background_1(),
        )))
        .s(RoundedCorners::all(10).top_right(40 / 2))
        .s(Padding::new().bottom(15))
        .after_remove(move |_| {
            drop(debounced_started_updater);
            drop(debounced_stopped_updater);
            drop(is_active_updater);
        })
        .item(time_entry_name_and_delete_button(project, time_entry.clone(), is_active.clone()))
        .item_signal(time_entry_times(time_entry, is_active))
}

fn time_entry_name_and_delete_button(
    project: Arc<super::Project>, 
    time_entry: Arc<super::TimeEntry>,
    is_active: ReadOnlyMutable<bool>,
) -> impl Element {
    let id = time_entry.id;
    Row::new()
        .item(time_entry_name(time_entry.clone(), is_active.clone()))
        .item(delete_entity_button(move || super::delete_time_entry(&project, id), is_active))
}

fn time_entry_name(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let debounced_rename = Mutable::new(None);
    El::new()
        .s(Padding::all(10))
        .child(
            TextInput::new()
                .s(Width::fill())
                .s(Font::new().color_signal(is_active.signal().map_bool_signal(
                    || theme::font_4(), 
                    || theme::font_1(),
                )))
                .s(Background::new().color_signal(theme::transparent()))
                .s(Borders::new().bottom_signal(
                    is_active.signal().map_bool_signal(
                        || theme::border_4().map(|color| Border::new().color(color)), 
                        || theme::border_1().map(|color| Border::new().color(color)),
                    )
                ))
                .s(Padding::all(5))
                .focus(not(time_entry.is_old))
                .label_hidden("time_entry name")
                .text_signal(time_entry.name.signal_cloned())
                .on_change(move |text| {
                    time_entry.name.set_neq(text);
                    debounced_rename.set(Some(Timer::once(app::DEBOUNCE_MS, clone!((time_entry) move || {
                        super::rename_time_entry(time_entry.id, &time_entry.name.lock_ref())
                    }))))
                })
        )
}

fn delete_entity_button(on_press: impl FnMut() + 'static, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let background_color = map_ref! {
        let hovered = hovered_signal,
        let is_active = is_active.signal() =>
        match (is_active, hovered) {
            (true, false) => theme::background_1().boxed_local(),
            (true, true) => theme::background_1_highlighted().boxed_local(),
            (false, false) => theme::background_3().boxed_local(),
            (false, true) => theme::background_3_highlighted().boxed_local(),
        }
    }.flatten();
    Button::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Align::new().top().right())
        .s(Background::new().color_signal(background_color))
        .s(Font::new()
            .color_signal(is_active.signal().map_bool_signal(
                || theme::font_1(), 
                || theme::font_3(),
            ))
            .weight(FontWeight::Bold)
        )
        .s(RoundedCorners::all_max())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(on_press)
        .label(app::icon_delete_forever())
}

fn time_entry_times(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Signal<Item = RawElement> {
    super::show_wide_time_entry().map(move |show_wide| {
        let items = element_vec![
            time_entry_started(time_entry.clone(), is_active.clone()),
            time_entry_duration(time_entry.clone(), is_active.clone()),
            time_entry_stopped(time_entry.clone(), is_active.clone()),
        ];
        if show_wide {
            time_entry_times_wide(items, is_active.clone()).into_raw_element()
        } else {
            time_entry_times_narrow(items, is_active.clone()).into_raw_element()
        }
    })
}

fn time_entry_times_narrow(items: Vec<RawElement>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    Column::new()
        .s(Font::new().color_signal(is_active.signal().map_bool_signal(
            || theme::font_4(), 
            || theme::font_1(),
        )))
        .items(items)
}

fn time_entry_times_wide(items: Vec<RawElement>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    Row::new()
        .s(Font::new().color_signal(is_active.signal().map_bool_signal(
            || theme::font_4(), 
            || theme::font_1(),
        )))
        .s(Padding::new().x(10))
        .s(Spacing::new(20))
        .items(items)
}

fn time_entry_date(
    year: impl Signal<Item = i32> + Unpin + 'static, 
    on_year_change: impl FnMut(String) -> Option<()> + 'static,
    month: impl Signal<Item = u32> + Unpin + 'static, 
    on_month_change: impl FnMut(String) -> Option<()> + 'static,
    day: impl Signal<Item = u32> + Unpin + 'static,
    on_day_change: impl FnMut(String) -> Option<()> + 'static,
    is_active: ReadOnlyMutable<bool>,
    read_only_when_active: bool,
) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(2))
        .item(
            date_time_part_input(
                year, 
                4, 
                false,
                is_active.clone(),
                read_only_when_active,
                on_year_change,
            )
        )
        .item("-")
        .item(
            date_time_part_input(
                month.map(|month| i32::try_from(month).unwrap_throw()), 
                2, 
                false,
                is_active.clone(),
                read_only_when_active,
                on_month_change,
            )
        )
        .item("-")
        .item(
            date_time_part_input(
                day.map(|day| i32::try_from(day).unwrap_throw()), 
                2, 
                false,
                is_active,
                read_only_when_active,
                on_day_change,
            )
        )
}

fn time_entry_time(
    hour: impl Signal<Item = u32> + Unpin + 'static, 
    on_hour_change: impl FnMut(String) -> Option<()> + 'static,
    minute: impl Signal<Item = u32> + Unpin + 'static, 
    on_minute_change: impl FnMut(String) -> Option<()> + 'static,
    second: impl Signal<Item = u32> + Unpin + 'static,
    on_second_change: impl FnMut(String) -> Option<()> + 'static,
    is_active: ReadOnlyMutable<bool>,
    read_only_when_active: bool,
) -> impl Element {
    Row::new()
        .s(Align::new().center_x())
        .s(Spacing::new(2))
        .item(
            date_time_part_input(
                hour.map(|hour| i32::try_from(hour).unwrap_throw()), 
                
                2, 
                false, 
                is_active.clone(),
                read_only_when_active,
                on_hour_change,
            ),
        )
        .item(":")
        .item(
            date_time_part_input(
                minute.map(|minute| i32::try_from(minute).unwrap_throw()), 
                2, 
                false,
                is_active.clone(),
                read_only_when_active,
                on_minute_change,
            )
        )
        .item(":")
        .item(
            date_time_part_input(
                second.map(|second| i32::try_from(second).unwrap_throw()), 
                2, 
                false,
                is_active,
                read_only_when_active,
                on_second_change,
            )
        )
}

fn time_entry_started(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    Row::new()
        .s(Padding::all(5))
        .s(Spacing::new(15))
        .item(time_entry_started_date(time_entry.clone(), is_active.clone()))
        .item(time_entry_started_time(time_entry.clone(), is_active))
}

fn time_entry_started_date(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let started = time_entry.started.clone();
    let year = started.signal().map(|date| date.year());
    let month = started.signal().map(|date| date.month());
    let day = started.signal().map(|date| date.day());
    time_entry_date(
        year,
        clone!((started) move |year| Some(started.set(started.get().with_year(year.parse().ok()?)?.into()))),
        month, 
        clone!((started) move |month| Some(started.set(started.get().with_month(month.parse().ok()?)?.into()))),
        day, 
        move |day| Some(started.set(started.get().with_day(day.parse().ok()?)?.into())),
        is_active, 
        false,
    )
}

fn time_entry_started_time(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let started = time_entry.started.clone();
    let hour = started.signal().map(|time| time.hour());
    let minute = started.signal().map(|time| time.minute());
    let second = started.signal().map(|time| time.second());
    time_entry_time(
        hour, 
        clone!((started) move |hour| Some(started.set(started.get().with_hour(hour.parse().ok()?)?.into()))),
        minute, 
        clone!((started) move |minute| Some(started.set(started.get().with_minute(minute.parse().ok()?)?.into()))),
        second, 
        move |second| Some(started.set(started.get().with_second(second.parse().ok()?)?.into())),
        is_active, 
        false,
    )
}

fn time_entry_duration(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let started = time_entry.started.clone();
    let stopped = time_entry.stopped.clone();

    let mutable_duration = Mutable::new((0, 0, 0));
    let duration = mutable_duration.read_only();

    let duration_signal = map_ref! {
        let current = super::current_time().signal(),
        let started = started.signal(),
        let stopped = stopped.signal() =>
        if let Some(stopped) = stopped {
            **stopped - **started
        } else {
            *current - **started
        }
    };
    let duration_updater = Task::start_droppable(
        duration_signal.for_each_sync(move |duration| {
            let num_seconds = duration.num_seconds();
            let seconds = num_seconds % 60;
            let minutes = (num_seconds / 60) % 60;
            let hours = (num_seconds / 60) / 60;
            mutable_duration.set((
                i32::try_from(hours).unwrap_throw(), 
                i32::try_from(minutes).unwrap_throw(), 
                i32::try_from(seconds).unwrap_throw(),
            ));
        })
    );
    let hours = duration.signal().map(|(hours, _, _)| hours);
    let minutes = duration.signal().map(|(_, minutes, _)| minutes);
    let seconds = duration.signal().map(|(_, _, seconds)| seconds);

    Row::new()
        .s(Align::new().center_x())
        .s(Padding::all(5))
        .s(Spacing::new(10))
        .after_remove(move |_| drop(duration_updater))
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item(
                    date_time_part_input(
                        hours, 
                        None, 
                        true, 
                        is_active.clone(), 
                        true,
                        clone!((started, stopped, duration) move |hours| {
                            let hours = hours.parse::<i32>().ok()?;
                            let (_, minutes, seconds) = duration.get();
                            let new_duration = Duration::seconds(i64::from(hours * 3600 + minutes * 60 + seconds));
                            let new_stopped = *started.get() + new_duration;
                            Some(stopped.set(Some(new_stopped.into())))
                        }),
                    )
                )
                .item("h"))
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item(
                    date_time_part_input(
                        minutes, 
                        3, 
                        true, 
                        is_active.clone(), 
                        true,
                        clone!((started, stopped, duration) move |minutes| {
                            let minutes = minutes.parse::<i32>().ok()?;
                            if minutes < 0 || minutes >= 60 {
                                None?
                            }
                            let (hours, _, seconds) = duration.get();
                            let new_duration = Duration::seconds(i64::from(hours * 3600 + minutes * 60 + seconds));
                            let new_stopped = *started.get() + new_duration;
                            Some(stopped.set(Some(new_stopped.into())))
                        }),
                    )
                )
                .item("m"))
        .item(
            Row::new()
                .s(Spacing::new(2))
                .item(
                    date_time_part_input(
                        seconds, 
                        3, 
                        true, 
                        is_active, 
                        true,
                        move |seconds| {
                            let seconds = seconds.parse::<i32>().ok()?;
                            if seconds < 0 || seconds >= 60 {
                                None?
                            }
                            let (hours, minutes, _) = duration.get();
                            let new_duration = Duration::seconds(i64::from(hours * 3600 + minutes * 60 + seconds));
                            let new_stopped = *started.get() + new_duration;
                            Some(stopped.set(Some(new_stopped.into())))
                        },
                    )
                )
                .item("s"))
}

fn time_entry_stopped(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    Row::new()
        .s(Padding::all(5))
        .s(Spacing::new(15))
        .item(time_entry_stopped_date(time_entry.clone(), is_active.clone()))
        .item(time_entry_stopped_time(time_entry.clone(), is_active))
}

fn time_entry_stopped_date(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let stopped = time_entry.stopped.clone();
    let year = map_ref! {
        let current_date = super::current_time().signal(),
        let stopped_date = stopped.signal() =>
        if let Some(stopped_date) = stopped_date {
            stopped_date.year()
        } else {
            current_date.year()
        }
    };
    let month = map_ref! {
        let current_date = super::current_time().signal(),
        let stopped_date = stopped.signal() =>
        if let Some(stopped_date) = stopped_date {
            stopped_date.month()
        } else {
            current_date.month()
        }
    };
    let day = map_ref! {
        let current_date = super::current_time().signal(),
        let stopped_date = stopped.signal() =>
        if let Some(stopped_date) = stopped_date {
            stopped_date.day()
        } else {
            current_date.day()
        }
    };
    time_entry_date(
        year, 
        clone!((stopped) move |year| Some(
            stopped.set(
                Some(stopped.get().unwrap_throw().with_year(year.parse().ok()?)?.into())
            )
        )),
        month, 
        clone!((stopped) move |month| Some(
            stopped.set(
                Some(stopped.get().unwrap_throw().with_month(month.parse().ok()?)?.into())
            )
        )),
        day, 
        move |day| Some(
            stopped.set(
                Some(stopped.get().unwrap_throw().with_day(day.parse().ok()?)?.into())
            )
        ),
        is_active, 
        true,
    )
}

fn time_entry_stopped_time(time_entry: Arc<super::TimeEntry>, is_active: ReadOnlyMutable<bool>) -> impl Element {
    let stopped = time_entry.stopped.clone();
    let hour = map_ref! {
        let current_time = super::current_time().signal(),
        let stopped_time = stopped.signal() =>
        if let Some(stopped_time) = stopped_time {
            stopped_time.hour()
        } else {
            current_time.hour()
        }
    };
    let minute = map_ref! {
        let current_time = super::current_time().signal(),
        let stopped_time = stopped.signal() =>
        if let Some(stopped_time) = stopped_time {
            stopped_time.minute()
        } else {
            current_time.minute()
        }
    };
    let second = map_ref! {
        let current_time = super::current_time().signal(),
        let stopped_time = stopped.signal() =>
        if let Some(stopped_time) = stopped_time {
            stopped_time.second()
        } else {
            current_time.second()
        }
    };
    time_entry_time(
        hour, 
        clone!((stopped) move |hour| Some(
            stopped.set(
                Some(stopped.get().unwrap_throw().with_hour(hour.parse().ok()?)?.into())
            )
        )),
        minute, 
        clone!((stopped) move |minute| Some(
            stopped.set(
                Some(stopped.get().unwrap_throw().with_minute(minute.parse().ok()?)?.into())
            )
        )),
        second, 
        move |second| Some(
            stopped.set(
                Some(stopped.get().unwrap_throw().with_second(second.parse().ok()?)?.into())
            )
        ),
        is_active, 
        true,
    )
}

fn date_time_part_input(
    number: impl Signal<Item = i32> + Unpin + 'static,
    max_chars: impl Into<Option<u32>>, 
    bold: bool,
    is_active: ReadOnlyMutable<bool>,
    read_only_when_active: bool,
    mut on_change: impl FnMut(String) -> Option<()> + 'static,
) -> impl Element {
    let max_chars = max_chars.into();
    let (valid, valid_signal) = Mutable::new_and_signal(true);
    let (text, text_signal) = Mutable::new_and_signal_cloned(String::new());
    let focused = Mutable::new(false);

    let text_updater = Task::start_droppable(
        number.for_each_sync(clone!((valid, focused) move |number| {
            if not(focused.get()) {
                valid.set_neq(true);
                if max_chars == Some(2) {
                    text.set_neq(format!("{:02}", number));
                } else {
                    text.set_neq(number.to_string());
                }
            }
        }))
    );

    TextInput::new()
        .s(RoundedCorners::all(3))
        .s(Width::zeros(max_chars.unwrap_or(4)))
        .s(
            Font::new()
                .color_signal(is_active.signal().map_bool_signal(
                    || theme::font_4(), 
                    || theme::font_1(),
                ))
                .center()
                .weight(if bold { FontWeight::Bold } else { FontWeight::Regular } )
        )
        .s(Background::new().color_signal(valid_signal.map_bool_signal(
            || theme::transparent(), 
            || theme::background_invalid(),
        )))
        .s(Borders::new().bottom_signal(
            theme::border_1().map(|color| Border::new().color(color))
        ))
        .s(Borders::new().bottom_signal(
            is_active.signal().map_bool_signal(
                move || {
                    let color = if read_only_when_active {
                        theme::transparent().left_either() 
                    } else { 
                        theme::border_4().right_either()
                    };
                    color.map(|color| Border::new().color(color))
                }, 
                || theme::border_1().map(|color| Border::new().color(color)),
            )
        ))
        .after_remove(move |_| drop(text_updater))
        .on_focused_change(move |is_focused| focused.set(is_focused))
        .label_hidden("date_time_part_input")
        .text_signal(text_signal)
        .input_type(InputType::text().max_chars(max_chars))
        .read_only_signal(is_active.signal().map_bool(move || read_only_when_active, || false))
        .on_change(move |text| {
            valid.set_neq(on_change(text).is_some())
        })
}
