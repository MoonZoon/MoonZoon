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
