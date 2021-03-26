use zoon::*;

mod element;
use element::counter::Counter;

blocks!{

    #[cmp]
    fn root() -> Cmp {
        Column::new()
            .with(control_counters())
            .with(counters())
    }

    #[cmp]
    fn control_counters() -> Cmp {
        Row::new()
            .with(column_counter())
            .with(row_counter())
            .with(counter_count())
            .with(counter_count_hundreds())
            .with(test_counters())
            .with(click_me_button())
    }

    #[cmp]
    fn click_me_button() -> Cmp {
        let title = cmp_var(|| "Click me!".to_owned());
        let click_count = cmp_var(|| 0);
        Row::new()
            .with(Button::new()
                .label(title.inner())
                .on_press(move || {
                    click_count.update(|count| count + 1);
                    title.set(format!("Clicked {}x", click_count.inner()));
                })
            )
    } 

    #[cmp]
    fn test_counters() -> Cmp {
        Row::new()
            .with("Test counters")
            .with(Counter::new()
                .value(super::test_counter_value().inner())
                .on_change(super::set_test_counter_value)
            )
            .with(Counter::new())
    } 

    #[cmp]
    fn counter_count() -> Cmp {
        El::new().with(format!("Counters: {}", super::counter_count().inner()))
    }

    #[cmp]
    fn counter_count_hundreds() -> Cmp {
        El::new().with(super::counter_count_hundreds().map(|count| {
            format!("Thousands: {}", count)
        }))
    }

    #[cmp]
    fn column_counter() -> Cmp {
        Row::new()
            .with("Columns:")
            .with(Counter::new()
                .value(super::column_count().inner())
                .on_change(super::set_column_count)
                .step(5)
            )
    }

    #[cmp]
    fn row_counter() -> Cmp {
        Row::new()
            .with("Rows:")
            .with(Counter::new()
                .value(super::row_count().inner())
                .on_change(super::set_row_count)
                .step(5)
            )
    }

    #[cmp]
    fn counters() -> Cmp {
        Column::new()
            .with_iter((0..super::row_count().inner()).map(|_| counter_row()))
    }

    #[cmp]
    fn counter_row() -> Cmp {
        Row::new()
            .with_iter((0..super::column_count().inner()).map(|_| counter()))
    }

    #[cmp]
    fn counter() -> Cmp {
        Counter::new()
    }

}
