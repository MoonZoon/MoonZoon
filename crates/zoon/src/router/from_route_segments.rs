pub trait FromRouteSegments: Sized {
    fn from_route_segments(segments: Vec<String>) -> Option<Self>;
}
