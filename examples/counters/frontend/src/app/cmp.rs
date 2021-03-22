use zoon::*;

#[macro_use]
mod element;
use element::counter::{self, Counter};

blocks!{

    // #[cmp]
    // fn root<'a>() -> Cmp<'a> {
    //     log!("from ROOT: {:#?}", TrackedCallId::current());
    //     counter![
    //         super::column_count().inner(),
    //         counter::on_change(super::set_column_count),
    //         counter::step(5),
    //     ]
    // }

    #[cmp]
    fn root<'a>() -> Cmp<'a> {
        // log!("from ROOT: {:#?}", TrackedCallId::current());
        // log!("from ROOT: {:#?}", TrackedCallId::current());
        // super::counter_count();
        // log!("CMP root ID: {:#?}", __TrackedCallId::current());
        // control_counters()
        col![
            control_counters(),
            counters(),
        ]
    }

    #[cmp]
    fn control_counters<'a>() -> Cmp<'a> {
        // log!("from CONTROL_COUNTERS: {:#?}", TrackedCallId::current());
        // log!("from CONTROL_COUNTERS: {:#?}", TrackedCallId::current());
        row![
            column_counter(),
            row_counter(),
            counter_count(),
            counter_count_hundreds(),
            test_counter(),
        ]
    }

    #[cmp]
    fn test_counter<'a>() -> Cmp<'a> {
        row![
            "Test counter",
            counter![
                super::test_counter_value().inner(),
                counter::on_change(super::set_test_counter_value),
            ]
        ]
    } 

    #[cmp]
    fn counter_count<'a>() -> Cmp<'a> {
        // log!("from COUNTER_COUNT: {:#?}", TrackedCallId::current());
        // log!("from COUNTER_COUNT: {:#?}", TrackedCallId::current());
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
        // log!("from COLUMN COUNTER: {:#?}", TrackedCallId::current());
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
        // log!("from ROW COUNTER: {:#?}", TrackedCallId::current());
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
        // log!("counters!");
        col![
            (0..super::row_count().inner()).map(|_| counter_row())
        ]
    }

    // #[tracked_call]
    // fn counter_row() -> Counter {
    //     // log!("____________________");
    //     // log!("from counter_row: {:#?}", TrackedCallId::current());
    //     counter![]
    //     // row![
    //     //     // (0..super::column_count().inner()).map(|_| counter())
    //     // ]
    // }

    #[cmp]
    fn counter_row<'a>() -> Cmp<'a> {
        // counter![]
        row![
            (0..super::column_count().inner()).map(|_| counter())
        ]
    }

    #[cmp]
    fn counter<'a>() -> Cmp<'a> {
        counter![]
    }

}
