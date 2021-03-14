// #![no_std]

use zoon::*;
use std::cell::RefCell;

mod element;
use element::counter::{self, Counter};

// blocks!{

//     #[s_var]
//     fn counter_count() -> usize {
//         3
//     }

//     #[update]
//     fn set_counter_count(count: usize) {
//         counter_count().set(count);
//     }

//     #[el]
//     fn root() -> Column {
//         column![
//             main_counter(),
//             counters(),
//         ]
//     }

//     #[el]
//     fn main_counter() -> Counter {
//         counter![
//             counter_count().inner(),
//             counter::on_change(set_counter_count),
//         ]
//     }

//     #[el]
//     fn counters() -> Row {
//         row![
//             (0..counter_count().inner()).iter().map(|_| counter![]),
//         ]
//     }
// }


thread_local! {
    static COUNTER_COUNT: RefCell<i32> = RefCell::new(3);
}


fn counter_count() -> i32 {
    COUNTER_COUNT.with(|counter_count| {
        *counter_count.borrow()
    })
}

fn set_counter_count(count: i32) {
    log!("set_counter_count: {}", count);
    COUNTER_COUNT.with(|counter_count| {
        *counter_count.borrow_mut() = count;
    })
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
        // counter_count().inner(),
        counter_count(),
        counter::on_change(set_counter_count),
    ]
}

#[cmp]
fn counters<'a>() -> Row<'a> {
    row![
        // (0..counter_count().inner()).map(|_| counter![]),
        (0..counter_count()).map(|_| counter![]),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    start!(root)
}
