// @TODO replace `once_cell::sync::Lazy` with `std::sync::LazyLock as Lazy` once stable
pub use once_cell::sync::Lazy;

pub const fn new<T, F>(f: F) -> Lazy<T, F> {
    Lazy::new(f)
}

pub const fn default<T: Default>() -> Lazy<T> {
    Lazy::new(<_>::default)
}

pub trait LazyExt {
    fn init_lazy(&self);
}

impl<T> LazyExt for Lazy<T> {
    fn init_lazy(&self) {
        Lazy::force(self);
    }
}
