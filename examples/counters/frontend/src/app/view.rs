use zoon::*;
use zoon::futures_signals::{signal::{Mutable, SignalExt}, signal_vec::SignalVecExt};
use std::{borrow::Cow, ops::AddAssign};

mod element;
use element::counter::{self, Counter};
use crate::counter;

pub fn root() -> Column {
    col![
        control_counters(),
        counters(),
    ]
}

fn control_counters() -> Row {
    row![
        column_counter(),
        row_counter(),
        counter_count(),
        counter_count_hundreds(),
        test_counters(),
        click_me_button(),
    ]
}

fn click_me_button() -> Row {
    let click_count = Mutable::new(0);
    let title = click_count.signal().map(|count| {
        if count == 0 { return Cow::from("Click me!") }
        Cow::from(format!("Clicked {}x", count))
    });
    row![
        button![
            button::label_signal(title),
            button::on_press(move || click_count.lock_mut().add_assign(1)),
        ],
    ]
} 

fn test_counters() -> Row {
    row![
        "Test counters",
        counter![
            counter::value_signal(super::test_counter_value().signal()),
            counter::on_change(super::on_test_counter_change),
        ],
        counter![],
    ]
} 

fn counter_count() -> El {
    el![
        el::child_signal(
            super::counter_count()
                .map(|count| format!("Counters: {}", count))
        )
    ]
}

fn counter_count_hundreds() -> El {
    el![
        el::child_signal(
            super::counter_count_hundreds()
                .map(|count| format!("Thousands: {}", count))
        )
    ]
}

fn column_counter() -> Row {
    row![
        "Columns:",
        counter![
            counter::value_signal(super::column_count().map(|count| count as i32)),
            counter::on_change(super::on_column_counter_change),
            counter::step(5),
        ]
    ]
}

fn row_counter() -> Row {
    row![
        "Rows:",
        counter![
            counter::value_signal(super::row_count().map(|count| count as i32)),
            counter::on_change(super::on_row_counter_change),
            counter::step(5),
        ]
    ]
}

fn counters() -> Row {
    row![
        row::items_signal_vec(
            super::columns().signal_vec().map(|_| counter_column())
        )
    ]
}

fn counter_column() -> Column {
    col![
        column::items_signal_vec(
            super::rows().signal_vec().map(|_| Counter::new())
        )
    ]
}

