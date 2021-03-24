use zoon::*;

#[macro_use]
mod element;
use element::counter::{self, Counter};

blocks!{

    // #[cmp]
    // fn root() -> Cmp {
    //     test_counter_2()
    // }

    // #[cmp]
    // fn root() -> Cmp {
    //     log!("from ROOT: {:#?}", TrackedCallId::current());
    //     counter![
    //         super::column_count().inner(),
    //         counter::on_change(super::set_column_count),
    //         counter::step(5),
    //     ]
    // }

    #[cmp]
    fn root() -> Cmp {
        // super::test_counter_value();

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
    fn control_counters() -> Cmp {
        // log!("from CONTROL_COUNTERS: {:#?}", TrackedCallId::current());
        // log!("from CONTROL_COUNTERS: {:#?}", TrackedCallId::current());
        row![
            column_counter(),
            row_counter(),
            counter_count(),
            counter_count_hundreds(),
            test_counter(),
            click_me_button(),
        ]
    }

    #[cmp]
    fn click_me_button() -> Cmp {
        let title = l_var(|| "Click me!".to_owned());
        let click_count = l_var(|| 0);
        row![
            button![
                title.inner(),
                button::on_press(move || {
                    log!("CLICKED! {:#?}", title);
                    click_count.update(|count| count + 1);
                    title.set(format!("Clicked {}x", click_count.inner()));
                }),
            ],
            counter![]
        ]
    } 

    #[cmp]
    fn test_counter() -> Cmp {
        row![
            "Test counter",
            counter![
                super::test_counter_value().inner(),
                counter::on_change(super::set_test_counter_value),
            ]
        ]
    } 

    #[cmp]
    fn counter_count() -> Cmp {
        // log!("from COUNTER_COUNT: {:#?}", TrackedCallId::current());
        // log!("from COUNTER_COUNT: {:#?}", TrackedCallId::current());
        el![
            format!("Counters: {}", super::counter_count().inner())
        ]
    }

    #[cmp]
    fn counter_count_hundreds() -> Cmp {
        el![
            super::counter_count_hundreds().map(|count| {
                format!("Thousands: {}", count)
            })
        ]
    }

    #[cmp]
    fn column_counter() -> Cmp {
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
    fn row_counter() -> Cmp {
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
    fn counters() -> Cmp {
        col![
            (0..super::row_count().inner()).map(|_| counter_row())
        ]
    }

    #[cmp]
    fn counter_row() -> Cmp {
        row![
            (0..super::column_count().inner()).map(|_| counter())
        ]
    }

    #[cmp]
    fn counter() -> Cmp {
        counter![]
    }

}
