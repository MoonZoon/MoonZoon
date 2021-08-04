use std::borrow::Cow;

pub trait RouteSegment: Sized {
    fn from_string_segment(segment: &str) -> Option<Self>;

    fn into_string_segment(self) -> Cow<'static, str>;
}

//-- impls --

impl RouteSegment for String {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Some(segment.to_owned())
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        self.into()
    }
}

impl RouteSegment for Cow<'static, str> {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Some(segment.to_owned().into())
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        self
    }
}

macro_rules! make_route_segment_impls {
    ($($type:ty),*) => (
        $(
        impl RouteSegment for $type {
            fn from_string_segment(segment: &str) -> Option<Self> {
                segment.parse().ok()
            }

            fn into_string_segment(self) -> Cow<'static, str> {
                self.to_string().into()
            }
        }
        )*
    )
}
make_route_segment_impls!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
