use crate::*;

mod from_route_segments;
mod route_segment;
mod router;

pub use from_route_segments::FromRouteSegments;
pub use route_segment::RouteSegment;
pub use router::Router;

pub fn current_url() -> String {
    window().location().href().unwrap_throw()
}

pub fn back() {
    history().back().unwrap_throw();
}

pub fn decode_uri_component(component: impl AsRef<str>) -> Result<String, JsValue> {
    let decoded = js_sys::decode_uri_component(component.as_ref())?;
    Ok(String::from(decoded))
}

pub fn encode_uri_component(component: impl AsRef<str>) -> String {
    let encoded = js_sys::encode_uri_component(component.as_ref());
    String::from(encoded)
}
