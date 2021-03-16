// #![no_std]

use zoon::*;

mod element;
use element::counter::{self, Counter};

blocks!{

    #[s_var]
    fn column_count() -> SVar<i32> {
        3
    }

    #[s_var]
    fn row_count() -> SVar<i32> {
        2
    }

    #[s_var]
    fn counter_count() -> SVar<i32> {
        6
    }

    #[update]
    fn set_column_count(count: i32) {
        column_count().set(count);
    }

    #[update]
    fn set_row_count(count: i32) {
        row_count().set(count);
    }

    #[cmp]
    fn root<'a>() -> Column<'a> {
        col![
            control_counters(),
            counters(),
        ]
    }

    #[cmp]
    fn control_counters<'a>() -> Row<'a> {
        row![
            column_counter(),
            row_counter(),
            counter_count_text()
        ]
    }

    #[cmp]
    fn counter_count_text<'a>() -> El<'a> {
        el![
            format!("Counters: {}", counter_count().inner().to_string())
        ]
    }

    #[cmp]
    fn column_counter<'a>() -> Row<'a> {
        row![
            "Columns:",
            counter![
                column_count().inner(),
                counter::on_change(set_column_count),
            ]
        ]
    }

    #[cmp]
    fn row_counter<'a>() -> Row<'a> {
        row![
            "Rows:",
            counter![
                row_count().inner(),
                counter::on_change(set_row_count),
            ]
        ]
    }

    #[cmp]
    fn counters<'a>() -> Column<'a> {
        col![
            (0..row_count().inner()).map(|_| counter_row())
        ]
    }

    #[cmp]
    fn counter_row<'a>() -> Row<'a> {
        row![
            (0..column_count().inner()).map(|_| counter())
        ]
    }

    #[cmp]
    fn counter() -> Counter {
        counter![]
    }

}

#[wasm_bindgen(start)]
pub fn start() {
    start!(blocks)
}
