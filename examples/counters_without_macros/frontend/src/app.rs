// use zoon::*;
use zoon::once_cell::sync::Lazy;
use zoon::futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};

mod cmp;

fn column_count() -> &'static Mutable<i32> {
    &Lazy::new(|| Mutable::new(5))
}

fn row_count() -> &'static Mutable<i32> {
    &Lazy::new(|| Mutable::new(5))
}

fn test_counter_value() -> &'static Mutable<i32> {
    &Lazy::new(|| Mutable::new(0))
}

pub fn counter_count() -> impl Signal<Item = i32> {
    map_ref!{
        let column_count = column_count().signal(),
        let row_count = row_count().signal() =>
        *column_count * *row_count
    }
}

pub fn counter_count_hundreds() -> impl Signal<Item = String> {
    counter_count()
        .map(|count| format!("{:.2}", f64::from(count) / 1_000.))
}

pub fn set_column_count(count: i32) {
    column_count().set(count)
}

pub fn set_row_count(count: i32) {
    row_count().set(count)
}

pub fn set_test_counter_value(count: i32) {
    test_counter_value().set(count)
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
