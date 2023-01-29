// @TODO remove once `f64::from(bool)` is implemented in Rust
// https://github.com/rust-lang/rust/issues/74015

pub trait IntoF64: Sized {
    fn into_f64(self) -> f64;
}

macro_rules! make_into_f64_impls {
    ($($type:ty),*) => (
        $(
        impl IntoF64 for $type {
            fn into_f64(self) -> f64 {
                self.into()
            }
        }
        )*
    )
}
make_into_f64_impls!(u8, u16, u32, i8, i16, i32, f32, f64);

impl IntoF64 for bool {
    fn into_f64(self) -> f64 {
        self as u8 as f64
    }
}
