use zoon::*;

zoons!{

    #[model]
    fn counter() -> i32 {
        0
    }

    #[update]
    fn increment() {
        counter().update(|counter| *counter += 1);
    }

    #[update]
    fn decrement() {
        counter().update(|counter| *counter -= 1);
    }

    #[view]
    fn view() -> Column {
        column![
            button![button::on_press(decrement), "-"],
            counter().inner(),
            button![button::on_press(increment), "+"],
        ]
    }

}

fn main() {
    start!(zoons)
}
