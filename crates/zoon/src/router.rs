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

// ------ RouteSegment ------

pub trait RouteSegment: Sized {
    fn from_route_segment(segment: &str) -> Option<Self>;

    fn into_route_segment(self) -> Cow<'static, str>;
}
