pub trait FnBoxClone<T, U = ()>: Fn(T) -> U {
    fn box_clone(&self) -> Box<dyn FnBoxClone<T, U>>;
}
impl<T, U, F: Fn(T) -> U + Clone + 'static> FnBoxClone<T, U> for F {
    fn box_clone(&self) -> Box<dyn FnBoxClone<T, U>> {
        Box::new(self.clone())
    }
}
impl<T, U> Clone for Box<dyn FnBoxClone<T, U>> {
    fn clone(&self) -> Self {
        use std::ops::Deref;
        self.deref().box_clone()
    }
}
