use zoon::*;
use std::borrow::Cow;

mod element;
use element::counter::Counter;

pub fn root() -> impl Element {
    Column::new()
        .item(control_counters())
        .item(counters())
}

fn control_counters() -> impl Element {
    Row::new()
        .item(column_counter())
        .item(row_counter())
        .item(counter_count())
        .item(counter_count_hundreds())
        .item(test_counters())
        .item(click_me_button())
}

fn click_me_button() -> impl Element {
    let click_count = Mutable::new(0);
    let title = click_count.signal().map(|count| {
        if count == 0 { return Cow::from("Click me!") }
        Cow::from(format!("Clicked {}x", count))
    });
    Row::new()
        .item(
            Button::new()
                .label_signal(title)
                .on_press(move || click_count.update(|count| count + 1))
        )
} 

fn test_counters() -> impl Element {
    Row::new()
        .item("Test counters")
        .item(
            Counter::new()
                .value_signal(super::test_counter_value().signal())
                .on_change(super::on_test_counter_change)
        )
        .item(
            Counter::new().value(1)
        )
} 

fn counter_count() -> impl Element {
    El::new()
        .child_signal(super::counter_count().map(|count| format!("Counters: {}", count)))
}

fn counter_count_hundreds() -> impl Element {
    El::new()
        .child_signal(super::counter_count_hundreds().map(|count| format!("Thousands: {}", count)))
}

fn column_counter() -> impl Element {
    Row::new()
        .item("Columns:")
        .item(
            Counter::new()
                .value_signal(super::column_count().map(|count| count as i32))
                .on_change(super::on_column_counter_change)
                .step(5)
        )
}

fn row_counter() -> impl Element {
    Row::new()
        .item("Rows:")
        .item(
            Counter::new()
                .value_signal(super::row_count().map(|count| count as i32))
                .on_change(super::on_row_counter_change)
                .step(5)
        )
}

fn counters() -> impl Element {
    Row::new()
        .items_signal_vec(super::columns().signal_vec().map(|_| counter_column()))
}

fn counter_column() -> impl Element {
    Column::new()
        .items_signal_vec(super::rows().signal_vec().map(|_| Counter::new()))
}
