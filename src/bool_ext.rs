pub trait BoolExt {
    fn map_true<T>(self, value: T) -> Option<T>;

    fn map_false<T>(self, value: T) -> Option<T>;
}

impl BoolExt for bool {
    fn map_true<T>(self, value: T) -> Option<T> {
        if self {
            Some(value)
        } else {
            None
        }
    }

    fn map_false<T>(self, value: T) -> Option<T> {
        if self {
            None
        } else {
            Some(value)
        }
    }
}
