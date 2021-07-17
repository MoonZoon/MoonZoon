use crate::{
    app::{self, PageId},
    report_page,
};
use zoon::*;

// ------ Router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::ReportWithFrequency { frequency }) => {
            if not(app::is_user_logged()) {
                return router().replace(Route::Login);
            }
            app::set_page_id(PageId::Report);
            report_page::set_frequency(frequency);
        }
        Some(Route::Report) => {
            if not(app::is_user_logged()) {
                return router().replace(Route::Login);
            }
            app::set_page_id(PageId::Report);
        }
        Some(Route::Login) => {
            if app::is_user_logged() {
                return router().replace(Route::Root);
            }
            app::set_page_id(PageId::Login);
        }
        Some(Route::Root) => {
            app::set_page_id(PageId::Home);
        }
        None => {
            app::set_page_id(PageId::Unknown);
        }
    })
}

// ------ Route ------

#[route]
pub enum Route {
    #[route("report", frequency)]
    ReportWithFrequency { frequency: report_page::Frequency },
    #[route("report")]
    Report,
    #[route("login")]
    Login,
    #[route()]
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
        if segments.len() != 1 { None? }
        if segments[0] != "login" { None? }
        let route = Self::Login;
        Some(route)
    }

    fn route_3_from_route_segments(segments: &[String]) -> Option<Self> {
        if segments.len() != 0 { None? }
        let route = Self::Root;
        Some(route)
    }
}

impl<'a> IntoCowStr<'a> for Route {
    fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
        match self {
            Self::ReportWithFrequency { frequency } => {
                format!(
                    "/report/{}", 
                    encode_uri_component(frequency.into_string_segment()),
                ).into()
            }
            Self::Report => "/report".into(),
            Self::Login => "/login".into(),
            Self::Root => "/".into(),
        }
    }

    fn take_into_cow_str(&mut self) -> std::borrow::Cow<'a, str> {
        unimplemented!()
    }
}
