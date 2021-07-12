use crate::{report, set_page_id, PageId};
use zoon::*;

// ------ Router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::ReportWithFrequency { frequency }) => {
            set_page_id(PageId::Report);
            report::set_frequency(frequency);
        }
        Some(Route::Report) => {
            set_page_id(PageId::Report);
        }
        Some(Route::Root) => {
            set_page_id(PageId::Home);
        }
        None => {
            set_page_id(PageId::Unknown);
        }
    })
}

// ------ Route ------

// #[route]
// pub enum Route {
//     #[route("report", frequency)]
//     ReportWithFrequency { frequency: report::Frequency },
//     #[route("report")]
//     Report,
//     #[route()]
//     Root,
// }

// #[route]
pub enum Route {
    // #[route("report", frequency)]
    ReportWithFrequency { frequency: report::Frequency },
    // #[route("report")]
    Report,
    // #[route()]
    Root,
}

impl Route {
    fn route_0_from_route_segments(segments: &[String]) -> Option<Self> {
        if segments.len() != 2 { None? }
        if segments[0] != "report" { None? }
        let route = Self::ReportWithFrequency { 
            frequency: RouteSegment::from_string_segment(&segments[1])? 
        };
        Some(route)
    }

    fn route_1_from_route_segments(segments: &[String]) -> Option<Self> {
        if segments.len() != 1 { None? }
        if segments[0] != "report" { None? }
        let route = Self::Report;
        Some(route)
    }

    fn route_2_from_route_segments(segments: &[String]) -> Option<Self> {
        if segments.len() != 0 { None? }
        let route = Self::Root;
        Some(route)
    }
}

impl FromRouteSegments for Route {
    fn from_route_segments(segments: Vec<String>) -> Option<Self> {
        None
            .or_else(|| Self::route_0_from_route_segments(&segments))
            .or_else(|| Self::route_1_from_route_segments(&segments))
            .or_else(|| Self::route_2_from_route_segments(&segments))
    }
}

impl<'a> IntoCowStr<'a> for Route {
    fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
        match self {
            Self::ReportWithFrequency { frequency } => {
                format!(
                    "/report/{}", 
                    frequency.into_string_segment(),
                ).into()
            }
            Self::Report => {
                "/report".into()
            }
            Self::Root => {
                "/".into()
            }
        }
    }

    fn take_into_cow_str(&mut self) -> std::borrow::Cow<'a, str> {
        unimplemented!()
    }
}
