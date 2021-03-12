// #![no_std]

use zoon::*;

mod component;
use component::counter::{self, Counter};

blocks!{

    #[s_var]
    fn counter_count() -> usize {
        3
    }

    #[update]
    fn set_counter_count(count: usize) {
        counter_count().set(count);
    }

    #[el]
    fn root() -> Column {
        column![
            main_counter(),
            counters(),
        ]
    }

    #[el]
    fn main_counter() -> Counter {
        counter![
            counter_count().inner(),
            counter::on_change(set_counter_count),
        ]
    }

    #[el]
    fn counters() -> Row {
        row![
            (0..counter_count().inner()).iter().map(|_| counter![]),
        ]
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    start!()
}
