use zoon::{*, println};
use zoon::futures_signals::{signal::{Mutable, SignalExt}, signal_vec::SignalVecExt};
use std::borrow::Cow;

mod element;
use element::counter::Counter;

pub fn root() -> Column {
    Column::new()
        .item(control_counters())
        .item(counters())
}

fn control_counters() -> Row {
    Row::new()
        .item(column_counter())
        .item(row_counter())
        .item(counter_count())
        .item(counter_count_hundreds())
        .item(test_counters())
        .item(click_me_button())
}

fn click_me_button() -> Row {
    let click_count = Mutable::new(0);
    let title = click_count.signal().map(|count| {
        if count == 0 { 
            return Cow::from("Click me!") 
        }
        Cow::from(format!("Clicked {}x", count))
    });
    Row::new()
        .item(Button::new()
            .label_signal(title)
            .on_press(move || {
                click_count.replace_with(|count| *count + 1);
            })
        )
} 

fn test_counters() -> Row {
    Row::new()
        .item("Test counters")
        .item(Counter::new()
            .value_signal(super::test_counter_value().signal())
            .on_change(super::on_test_counter_change)
        )
        .item(Counter::new())
} 

fn counter_count() -> El {
    El::new()
        .child_signal(
            super::counter_count()
                .map(|count| format!("Counters: {}", count))
        )
}

fn counter_count_hundreds() -> El {
    El::new()
        .child_signal(
            super::counter_count_hundreds()
                .map(|count| format!("Thousands: {}", count))
        )
}

fn column_counter() -> Row {
    Row::new()
        .item("Columns:")
        .item(Counter::new()
            .value_signal(super::column_count().map(|count| count as i32))
            .on_change(super::on_column_counter_change)
            .step(5)
        )
}

fn row_counter() -> Row {
    Row::new()
        .item("Rows:")
        .item(Counter::new()
            .value_signal(super::row_count().map(|count| count as i32))
            .on_change(super::on_row_counter_change)
            .step(5)
        )
}

fn counters() -> Row {
    Row::new()
        .items_signal(
            super::columns().signal_vec().map(move |_| counter_column())
        )
}

fn counter_column() -> Column {
    Column::new()
        .items_signal(
            super::rows().signal_vec().map(move |_| Counter::new())
        )
}

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
