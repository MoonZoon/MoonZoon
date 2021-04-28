use zoon::*;
use zoon::once_cell::sync::OnceCell;
use zoon::futures_signals::{map_ref, signal::{Mutable, Signal, SignalExt}};

mod element;
use element::counter::Counter;

fn root<'a>() -> Column<'a> {
    Column::new()
        .item(control_counters())
        // .item(counters())
}

fn control_counters<'a>() -> Row<'a> {
    Row::new()
        .item(column_counter())
        .item(row_counter())
        .item(counter_count())
        .item(counter_count_hundreds())
        .item(test_counters())
        .item(click_me_button())
}

fn click_me_button<'a>() -> Row<'a> {
    static __TITLE: OnceCell<Mutable<String>> = OnceCell::new();
    let title = __TITLE.get_or_init(|| Mutable::new("Click me!".to_owned()));

    static __CLICK_COUNT: OnceCell<Mutable<u32>> = OnceCell::new();
    let click_count = __CLICK_COUNT.get_or_init(|| Mutable::new(0));

    // static __CLICK_COUNT: Lazy<Mutable<u32>> = Lazy::new(|| Mutable::new(0));
    // let click_count = &__CLICK_COUNT;
    Row::new()
        .item(Button::new()
            // .label_signal(title.signal_ref())
            .on_press(move || {
                click_count.replace_with(|count| *count + 1);
                title.set(format!("Clicked {}x", click_count.get()));
            })
        )
} 

fn test_counters<'a>() -> Row<'a> {
    Row::new()
        .item("Test counters")
        .item(Counter::new()
            // .value_signal(super::test_counter_value().signal())
            .on_change(super::set_test_counter_value)
        )
        .item(Counter::new())
} 

fn counter_count<'a>() -> El<'a> {
    El::new()
        // .child_signal(
        //     super::counter_count()
        //         .map(|count| format!("Counters: {}", count))
        // )
}

fn counter_count_hundreds<'a>() -> El<'a> {
    El::new()
        // .child_signal(
        //     super::counter_count_hundreds()
        //         .map(|count| format!("Thousands: {}", count))
        // )
}

fn column_counter<'a>() -> Row<'a> {
    Row::new()
        .item("Columns:")
        .item(Counter::new()
            // .value_signal(super::column_count().signal())
            .on_change(super::set_column_count)
            .step(5)
        )
}

fn row_counter<'a>() -> Row<'a> {
    Row::new()
        .item("Rows:")
        .item(Counter::new()
            // .value_signal(super::row_count().signal())
            .on_change(super::set_row_count)
            .step(5)
        )
}

// fn counters<'a>() -> Column<'a> {
//     Column::new()
//         .items((0..super::row_count().inner()).map(|_| counter_row()))
// }

// fn counter_row<'a>() -> Row<'a> {
//     Row::new()
//         .items((0..super::column_count().inner()).map(|_| counter()))
// }

// fn counter() -> Counter {
//     Counter::new()
// }


// blocks!{

//     #[cmp]
//     fn root() -> Cmp {
//         Column::new()
//             .item(control_counters())
//             .item(counters())
//     }

//     #[cmp]
//     fn control_counters() -> Cmp {
//         Row::new()
//             .item(column_counter())
//             .item(row_counter())
//             .item(counter_count())
//             .item(counter_count_hundreds())
//             .item(test_counters())
//             .item(click_me_button())
//     }

//     #[cmp]
//     fn click_me_button() -> Cmp {
//         let title = cmp_var(|| "Click me!".to_owned());
//         let click_count = cmp_var(|| 0);
//         Row::new()
//             .item(Button::new()
//                 .label(title.inner())
//                 .on_press(move || {
//                     click_count.update(|count| count + 1);
//                     title.set(format!("Clicked {}x", click_count.inner()));
//                 })
//             )
//     } 

//     #[cmp]
//     fn test_counters() -> Cmp {
//         Row::new()
//             .item("Test counters")
//             .item(Counter::new()
//                 .value(super::test_counter_value().inner())
//                 .on_change(super::set_test_counter_value)
//             )
//             .item(Counter::new())
//     } 

//     #[cmp]
//     fn counter_count() -> Cmp {
//         El::new().child(format!("Counters: {}", super::counter_count().inner()))
//     }

//     #[cmp]
//     fn counter_count_hundreds() -> Cmp {
//         El::new().child(super::counter_count_hundreds().map(|count| {
//             format!("Thousands: {}", count)
//         }))
//     }

//     #[cmp]
//     fn column_counter() -> Cmp {
//         Row::new()
//             .item("Columns:")
//             .item(Counter::new()
//                 .value(super::column_count().inner())
//                 .on_change(super::set_column_count)
//                 .step(5)
//             )
//     }

//     #[cmp]
//     fn row_counter() -> Cmp {
//         Row::new()
//             .item("Rows:")
//             .item(Counter::new()
//                 .value(super::row_count().inner())
//                 .on_change(super::set_row_count)
//                 .step(5)
//             )
//     }

//     #[cmp]
//     fn counters() -> Cmp {
//         Column::new()
//             .items((0..super::row_count().inner()).map(|_| counter_row()))
//     }

//     #[cmp]
//     fn counter_row() -> Cmp {
//         Row::new()
//             .items((0..super::column_count().inner()).map(|_| counter()))
//     }

//     #[cmp]
//     fn counter() -> Cmp {
//         Counter::new()
//     }

// }
