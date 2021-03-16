use zoon::*;

#[macro_use]
mod element;
use element::counter::{self, Counter};

blocks!{

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
            format!("Counters: {}", super::counter_count().inner().to_string())
        ]
    }

    #[cmp]
    fn column_counter<'a>() -> Row<'a> {
        row![
            "Columns:",
            counter![
                super::column_count().inner(),
                counter::on_change(super::set_column_count),
            ]
        ]
    }

    #[cmp]
    fn row_counter<'a>() -> Row<'a> {
        row![
            "Rows:",
            counter![
                super::row_count().inner(),
                counter::on_change(super::set_row_count),
            ]
        ]
    }

    #[cmp]
    fn counters<'a>() -> Column<'a> {
        col![
            (0..super::row_count().inner()).map(|_| counter_row())
        ]
    }

    #[cmp]
    fn counter_row<'a>() -> Row<'a> {
        row![
            (0..super::column_count().inner()).map(|_| counter())
        ]
    }

    #[cmp]
    fn counter() -> Counter {
        counter![]
    }

}
