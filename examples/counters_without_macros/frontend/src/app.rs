use zoon::*;
use zoon::futures_signals::{
    map_ref, 
    signal::{Mutable, Signal, SignalExt}, 
    signal_vec::{MutableVec, SignalVecExt}
};

pub mod cmp;

// ------ Statics ------

#[static_ref]
fn columns() -> &'static MutableVec<()> {
    MutableVec::new_with_values(vec![(); 5])
}

#[static_ref]
fn rows() -> &'static MutableVec<()> {
    MutableVec::new_with_values(vec![(); 5])
}

#[static_ref]
fn test_counter_value() -> &'static Mutable<i32> {
    Mutable::new(0)
}

// ------ Signals ------

fn column_count() -> impl Signal<Item = usize> {
    columns().signal_vec().len()
}

fn row_count() -> impl Signal<Item = usize> {
    rows().signal_vec().len()
}

pub fn counter_count() -> impl Signal<Item = usize> {
    map_ref!{
        let column_count = column_count(),
        let row_count = row_count() =>
        column_count * row_count
    }
}

pub fn counter_count_hundreds() -> impl Signal<Item = String> {
    counter_count()
        .map(|count| format!("{:.2}", count as f64 / 1_000.))
}

// ------ Handlers ------

pub fn on_column_counter_change(step: i32) {
    let mut columns = columns().lock_mut();
    if step > 0 {
        (0..step).for_each(|_| columns.push(()))
    } else if step < 0 {
        (step..0).for_each(|_| { columns.pop(); })
    }
}

pub fn on_row_counter_change(step: i32) {
    let mut rows = rows().lock_mut();
    if step > 0 {
        (0..step).for_each(|_| rows.push(()))
    } else if step < 0 {
        (step..0).for_each(|_| { rows.pop(); })
    }
}

pub fn on_test_counter_change(step: i32) {
    test_counter_value().replace_with(|value| *value + step);
}
