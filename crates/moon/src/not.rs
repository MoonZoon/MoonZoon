use std::ops::Not;

pub fn not<T: Not>(value: T) -> T::Output {
    value.not()
}
