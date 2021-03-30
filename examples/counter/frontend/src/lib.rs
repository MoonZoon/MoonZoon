// #![no_std]

use zoon::*;

blocks!{

    #[s_var]
    fn counter() -> SVar<i32> {
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

    #[cmp]
    fn root() -> Cmp {
        col![
            button![button::on_press(decrement), "-"],
            counter().inner(),
            button![button::on_press(increment), "+"],
        ]
    }

}

#[wasm_bindgen(start)]
pub fn start() {
    start!()
}
