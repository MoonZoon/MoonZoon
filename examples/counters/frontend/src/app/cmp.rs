use zoon::*;

#[macro_use]
mod element;
use element::counter::{self, Counter};

blocks!{

    #[cmp]
    fn root<'a>() -> Cmp<'a> {
        // log!("CMP root ID: {:#?}", __TrackedCallId::current());
        col![
            control_counters(),
            // counters(),
        ]
    }

    #[cmp]
    fn control_counters<'a>() -> Cmp<'a> {
        row![
            column_counter(),
            // row_counter(),
            // counter_count(),
            // counter_count_hundreds(),
        ]
    }

    #[cmp]
    fn counter_count<'a>() -> Cmp<'a> {
        el![
            format!("Counters: {}", super::counter_count().inner())
        ]
    }

    #[cmp]
    fn counter_count_hundreds<'a>() -> Cmp<'a> {
        el![
            super::counter_count_hundreds().map(|count| {
                format!("Thousands: {}", count)
            })
        ]
    }

    #[cmp]
    fn column_counter<'a>() -> Cmp<'a> {
        row![
            "Columns:",
            counter![
                super::column_count().inner(),
                counter::on_change(super::set_column_count),
                counter::step(5),
            ]
        ]
    }

    #[cmp]
    fn row_counter<'a>() -> Cmp<'a> {
        row![
            "Rows:",
            counter![
                super::row_count().inner(),
                counter::on_change(super::set_row_count),
                counter::step(5),
            ]
        ]
    }

    #[cmp]
    fn counters<'a>() -> Cmp<'a> {
        col![
            (0..super::row_count().inner()).map(|_| counter_row())
        ]
    }

    #[cmp]
    fn counter_row<'a>() -> Cmp<'a> {
        row![
            (0..super::column_count().inner()).map(|_| counter())
        ]
    }

    #[cmp]
    fn counter<'a>() -> Cmp<'a> {
        counter![]
    }

}
