use std::borrow::Cow;
use std::rc::Rc;
use std::sync::Arc;

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

impl RouteSegment for Arc<String> {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Some(segment.to_owned().into())
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        Arc::unwrap_or_clone(self).into()
    }
}

impl RouteSegment for Rc<String> {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Some(segment.to_owned().into())
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        Rc::unwrap_or_clone(self).into()
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

impl RouteSegment for Arc<Cow<'static, str>> {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Some(Arc::new(segment.to_owned().into()))
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        Arc::unwrap_or_clone(self)
    }
}

impl RouteSegment for Rc<Cow<'static, str>> {
    fn from_string_segment(segment: &str) -> Option<Self> {
        Some(Rc::new(segment.to_owned().into()))
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        Rc::unwrap_or_clone(self)
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
