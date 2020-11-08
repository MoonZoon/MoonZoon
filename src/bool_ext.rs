pub trait BoolExt {
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T>;
}

impl BoolExt for bool {
    fn then<T, F: FnOnce() -> T>(self, f: F) -> Option<T> {
        if self {
            Some(f())
        } else {
            None
        }
    }
}
