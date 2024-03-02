use std::iter;
use zoon::{format, *};

mod counter;
use counter::Counter;

fn main() {
    start_app("app", root);
}

#[static_ref]
fn STORE -> &'static Store {
    Store::new()
}

#[derive(Educe)]
#[educe(Default(new))]
struct Store {
    #[educe(Default(expression = Mutable::new(5)))]
    column_count: Mutable<usize>,
    #[educe(Default(expression = Mutable::new(5)))]
    row_count: Mutable<usize>,
    test_counter_value: Mutable<i32>,
}

pub fn root() -> impl Element {
    Column::new().item(control_counters()).item(counters())
}

fn control_counters() -> impl Element {
    let counter_count = map_ref! {
        let column_count = STORE.column_count.signal(),
        let row_count = STORE.row_count.signal() =>
        column_count * row_count
    }
    .broadcast();
    Row::new()
        .item(column_counter())
        .item(row_counter())
        .item(
            El::new().child_signal(
                counter_count
                    .signal()
                    .map(|count| format!("Counters: {}", count)),
            ),
        )
        .item(
            El::new().child_signal(
                counter_count
                    .signal()
                    .map(|count| format!("Thousands: {:.2}", count as f64 / 1_000.)),
            ),
        )
        .item(test_counters())
        .item(click_me_button())
}

fn click_me_button() -> impl Element {
    let click_count = Mutable::new(0);
    Row::new().item(
        Button::new()
            .label_signal(click_count.signal().map(|count| {
                if count == 0 {
                    "Click me!".into_cow_str()
                } else {
                    format!("Clicked {count}x").into_cow_str()
                }
            }))
            .on_press(move || *click_count.lock_mut() += 1),
    )
}

fn test_counters() -> impl Element {
    Row::new()
        .item("Test counters")
        .item(
            Counter::with_signal(STORE.test_counter_value.signal())
                .on_change(|value| STORE.test_counter_value.set(value)),
        )
        .item(Counter::new(1))
}

fn column_counter() -> impl Element {
    Row::new().item("Columns:").item(
        Counter::with_signal(STORE.column_count.signal())
            .on_change(|value| STORE.column_count.set(value))
            .step(5),
    )
}

fn row_counter() -> impl Element {
    Row::new().item("Rows:").item(
        Counter::with_signal(STORE.row_count.signal())
            .on_change(|value| STORE.row_count.set(value))
            .step(5),
    )
}

fn counters() -> impl Element {
    let (columns, columns_updater) = count_signal_to_mutable_vec(STORE.column_count.signal());
    Row::new()
        .items_signal_vec(columns.signal_vec().map(|()| counter_column()))
        .after_remove(move |_| drop(columns_updater))
}

fn counter_column() -> impl Element {
    let (rows, rows_updater) = count_signal_to_mutable_vec(STORE.row_count.signal());
    Column::new()
        .items_signal_vec(rows.signal_vec().map(|()| Counter::new(0)))
        .after_remove(move |_| drop(rows_updater))
}

// --

fn count_signal_to_mutable_vec(
    count: impl Signal<Item = usize> + 'static,
) -> (MutableVec<()>, TaskHandle) {
    let mutable_vec = MutableVec::new();
    let mutable_vec_updater =
        Task::start_droppable(count.for_each_sync(clone!((mutable_vec) move |count| {
            let mut mutable_vec = mutable_vec.lock_mut();
            let current_count = mutable_vec.len();
            if count > current_count {
                mutable_vec.extend(iter::repeat(()).take(count - current_count))
            } else if count < current_count {
                mutable_vec.truncate(count)
            }
        })));
    (mutable_vec, mutable_vec_updater)
}
