// #![no_std]

use zoon::*;

mod element;
use element::counter::{self, Counter};

// blocks!{

//     #[s_var]
//     fn counter_count() -> i32 {
//         3
//     }

//     #[update]
//     fn set_counter_count(count: i32) {
//         counter_count().set(count);
//     }

//     #[cmp]
//     fn root() -> Column {
//         column![
//             main_counter(),
//             counters(),
//         ]
//     }

//     #[cmp]
//     fn main_counter() -> Counter {
//         counter![
//             counter_count().inner(),
//             counter::on_change(set_counter_count),
//         ]
//     }

//     #[cmp]
//     fn counters() -> Row {
//         row![
//             (0..counter_count().inner()).iter().map(|_| counter![]),
//         ]
//     }
// }

fn counter_count() -> SVar<i32> {
    s_var("counter_count", || 3)
}

fn set_counter_count(count: i32) {
    counter_count().set(count);
}

#[cmp]
fn root<'a>() -> Column<'a> {
    col![
        main_counter(),
        counters(),
    ]
}

#[cmp]
fn main_counter() -> Counter {
    counter![
        counter_count().inner(),
        counter::on_change(set_counter_count),
    ]
}

#[cmp]
fn counters<'a>() -> Row<'a> {
    row![
        (0..counter_count().inner()).map(|_| counter![]),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    start!(root)
}
