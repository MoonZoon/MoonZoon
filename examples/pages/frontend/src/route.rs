use zoon::*;
use crate::{PageId, set_page_id, report};

// ------ Router ------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::Report(Some(frequency))) => {
            set_page_id(PageId::Report);
            report::set_frequency(frequency);
        }
        Some(Route::Report(None)) => {
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

#[route]
pub enum Route {
    Report(Option<report::Frequency>),
    Root,
}

impl Route {  
    #[route("report", frequency)]
    fn report_with_frequency(frequency: report::Frequency) -> Route {
        Route::Report(Some(frequency))
    }

    #[route("report")]
    fn report() -> Route {
        Route::Report(None)
    }

    #[route]
    fn root() -> Route {
        Route::Root
    }
}
