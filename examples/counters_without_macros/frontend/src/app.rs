use zoon::*;
use zoon::futures_signals::{
    map_ref, 
    signal::{Mutable, Signal, SignalExt}, 
    signal_vec::{MutableVec, SignalVecExt}
};
use std::iter::repeat;

pub mod view;

// ------ ------
//    Statics 
// ------ ------

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

// ------ ------
//    Signals 
// ------ ------

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

// ------ ------
//   Handlers 
// ------ ------

pub fn on_column_counter_change(step: i32) {
    change_vec_size(columns(), step)
}

pub fn on_row_counter_change(step: i32) {
    change_vec_size(rows(), step)
}

pub fn on_test_counter_change(step: i32) {
    test_counter_value().replace_with(|value| *value + step);
}

// ------ ------
//    Helpers 
// ------ ------

fn change_vec_size(vec: &MutableVec<()>, step: i32) {
    let mut vec = vec.lock_mut();
    if step.is_positive() {
        vec.extend(repeat(()).take(step as usize))
    } else if step.is_negative() {
        vec.truncate(vec.len().saturating_sub(step.abs() as usize))
    }
}
