// use zoon::*;
use zoon::once_cell::sync::OnceCell;
use zoon::futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}, signal_vec::{MutableVec, SignalVecExt}};
use std::iter;

pub mod cmp;

fn columns() -> &'static MutableVec<i32> {
    static INSTANCE: OnceCell<MutableVec<i32>> = OnceCell::new();
    INSTANCE.get_or_init(|| MutableVec::new_with_values((0..5).collect()))
}

fn rows() -> &'static MutableVec<i32> {
    static INSTANCE: OnceCell<MutableVec<i32>> = OnceCell::new();
    INSTANCE.get_or_init(|| MutableVec::new_with_values((0..5).collect()))
}

fn column_count() -> impl Signal<Item = usize> {
    columns().signal_vec().len()
}

fn row_count() -> impl Signal<Item = usize> {
    rows().signal_vec().len()
}

fn test_counter_value() -> &'static Mutable<i32> {
    static INSTANCE: OnceCell<Mutable<i32>> = OnceCell::new();
    INSTANCE.get_or_init(|| Mutable::new(0))
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

pub fn on_column_counter_change(step: i32) {
    let mut columns = columns().lock_mut();
    if step > 0 {
        columns.reserve(step as usize);
        let last_column = columns.last().map(|column| column + 1).unwrap_or_default();
        for new_column in last_column..last_column + step {
            columns.push(new_column);
        }
    } else if step < 0 {
        for _ in step..0 {
            columns.pop();
        }
    }
}

pub fn on_row_counter_change(step: i32) {
    let mut rows = rows().lock_mut();
    if step > 0 {
        rows.reserve(step as usize);
        let last_row = rows.last().map(|row| row + 1).unwrap_or_default();
        for new_row in last_row..last_row + step {
            rows.push(new_row);
        }
    } else if step < 0 {
        for _ in step..0 {
            rows.pop();
        }
    }
}

pub fn on_test_counter_change(step: i32) {
    test_counter_value().replace_with(|value| *value + step);
}

// blocks!{

//     append_blocks!{
//         cmp,
//     }

//     #[s_var]
//     fn column_count() -> SVar<i32> {
//         5
//     }

//     #[s_var]
//     fn row_count() -> SVar<i32> {
//         5
//     }

//     #[s_var]
//     fn test_counter_value() -> SVar<i32> {
//         0
//     }

//     #[cache]
//     fn counter_count() -> Cache<i32> {
//         column_count().inner() * row_count().inner()
//     }

//     #[cache]
//     fn counter_count_hundreds() -> Cache<String> {
//         format!("{:.2}", f64::from(counter_count().inner()) / 1_000.)
//     }

//     #[update]
//     fn set_column_count(count: i32) {
//         column_count().set(count);
//     }

//     #[update]
//     fn set_row_count(count: i32) {
//         row_count().set(count);
//     }

//     #[update]
//     fn set_test_counter_value(count: i32) {
//         test_counter_value().set(count);
//     }

// }
