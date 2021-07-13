use std::borrow::Cow;

pub trait RouteSegment: Sized {
    fn from_string_segment(segment: &str) -> Option<Self>;

    fn into_string_segment(self) -> Cow<'static, str>;
}

//-- impls --

// @TODO for basic types (similar to `IntoCowStr` impls)
