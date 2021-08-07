use zoon::*;
use crate::app;

// ------ router -------

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::Active) => app::select_filter(app::Filter::Active),
        Some(Route::Completed) => app::select_filter(app::Filter::Completed),
        Some(Route::Root) | None => app::select_filter(app::Filter::All),
    })
} 

// ------ Route -------

#[route]
#[derive(Clone, Copy)]
pub enum Route {
    #[route("active")]
    Active,
    #[route("completed")]
    Completed,
    #[route()]
    Root,
}
