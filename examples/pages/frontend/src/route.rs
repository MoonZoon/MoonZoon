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
//     #[route]
//     Root,
// }

// #[route]
pub enum Route {
    // #[route("report", frequency)]
    ReportWithFrequency { frequency: report::Frequency },
    // #[route("report")]
    Report,
    // #[route]
    Root,
}

impl<'a> IntoCowStr<'a> for Route {
    fn into_cow_str(self) -> std::borrow::Cow<'a, str> {
        match self {
            Route::ReportWithFrequency { frequency } => {
                format!("/report/{}", frequency.into_route_segment()).into()
            }
            Route::Report => {
                "/report".into()
            }
            Route::Root => {
                "/".into()
            }
        }
    }

    fn take_into_cow_str(&mut self) -> std::borrow::Cow<'a, str> {
        unimplemented!("take_into_cow_str not implemented for Route")
    }
}
