use zoon::*;

zoons!{

    #[var]
    fn counter() -> i32 {
        0
    }

    #[update]
    fn increment() {
        counter().update(|counter| counter + 1);
    }

    #[update]
    fn decrement() {
        counter().update(|counter| counter - 1);
    }

    #[el]
    fn root() -> Column {
        column![
            button![button::on_press(decrement), "-"],
            counter().inner(),
            button![button::on_press(increment), "+"],
        ]
    }

}

fn main() {
    start!()
}
