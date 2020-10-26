use zoon::*;

#[Model]
fn counter() -> ModeL<i32> {
    use_model(|| 0)
}

#[Update]
fn increment() {
    counter().update(|c| *c += 1);
}

#[Update]
fn decrement() {
    counter().update(|c| *c -= 1);
}

#[View]
fn view() -> Row {
    column![
        button![text!["-"], button::on_press(decrement)],
        text![counter.get()],
        button![text!["-"], button::on_press(increment)],
    ]
}

fn main() {
    zoon::start("app", view)
}
