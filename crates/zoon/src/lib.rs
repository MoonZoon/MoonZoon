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

pub struct RenderContext {

}

pub trait Component where Self: Sized {
    fn set(mut self, attribute: impl ApplyToComponent<Self>) -> Self {
        attribute.apply_to_component(&mut self);
        self
    }
    fn render(&mut self, rcx: RenderContext);
}

pub trait ApplyToComponent<T: Component> {
    fn apply_to_component(self, component: &mut T);
}
