use std::marker::PhantomData;
use std::borrow::Cow;

// ------ Router ------

pub struct Router<R> {
    _route_type: PhantomData<R>
}

impl<R> Router<R> {
    pub fn new(on_route_change: impl FnOnce(Option<R>) + Clone + 'static) -> Self {
        Router {
            _route_type: PhantomData
        }
    }
}

// ------ FromRouteSegments ------

pub trait FromRouteSegments: Sized {
    fn from_route_segments(segments: Vec<String>) -> Option<Self>;
}

// ------ RouteSegment ------

pub trait RouteSegment: Sized {
    fn from_string_segment(segment: &str) -> Option<Self>;

    fn into_string_segment(self) -> Cow<'static, str>;
}

// @TODO for basic types (similar to `IntoCowStr` impls)
