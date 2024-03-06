use zoon::*;

static COUNTER: Lazy<Mutable<i32>> = lazy::default();

fn increment() {
    COUNTER.update(|counter| counter + 1)
}

fn decrement() {
    COUNTER.update(|counter| counter - 1)
}

fn root() -> impl IntoElementIterator {
    element_vec![
        Button::new().label("-").on_press(decrement),
        Text::with_signal(COUNTER.signal()),
        Button::new().label("+").on_press(increment),
    ]
}

fn main() {
    start_app("app", root);
}
