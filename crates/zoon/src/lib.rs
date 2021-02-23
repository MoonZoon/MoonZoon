pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use blocks::blocks;

#[macro_export]
macro_rules! start {
    () => {
        $crate::start();
    };
}

pub fn start() {

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
