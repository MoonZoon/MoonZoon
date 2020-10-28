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
fn view() -> Column {
    column![
        button![button::on_press(decrement), "-"],
        counter.inner(),
        button![button::on_press(increment), "+"],
    ]
}

fn main() {
    zoon::start("app", view)
}
