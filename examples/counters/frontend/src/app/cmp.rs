use zoon::*;

mod element;
use element::counter::{self, Counter};
use crate::counter;

blocks!{

    #[cmp]
    fn root() -> Cmp {
        col![
            control_counters(),
            counters(),
        ]
    }

    #[cmp]
    fn control_counters() -> Cmp {
        row![
            column_counter(),
            row_counter(),
            counter_count(),
            counter_count_hundreds(),
            test_counters(),
            click_me_button(),
        ]
    }

    #[cmp]
    fn click_me_button() -> Cmp {
        let title = cmp_var(|| "Click me!".to_owned());
        let click_count = cmp_var(|| 0);
        row![
            button![
                title.inner(),
                button::on_press(move || {
                    click_count.update(|count| count + 1);
                    title.set(format!("Clicked {}x", click_count.inner()));
                }),
            ],
        ]
    } 

    #[cmp]
    fn test_counters() -> Cmp {
        row![
            "Test counters",
            counter![
                super::test_counter_value().inner(),
                counter::on_change(super::set_test_counter_value),
            ],
            counter![],
        ]
    } 

    #[cmp]
    fn counter_count() -> Cmp {
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
