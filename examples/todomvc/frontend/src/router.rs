use crate::store::*;
use zoon::*;

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| async move {
        store().selected_filter.set_neq(match route {
            Some(Route::Active) => Filter::Active,
            Some(Route::Completed) => Filter::Completed,
            Some(Route::Root) | None => Filter::All,
        });
    })
}

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
