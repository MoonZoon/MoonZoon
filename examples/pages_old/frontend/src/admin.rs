use zoon::*;
use crate::app;

mod report;

blocks!{
    append_blocks![report]

    #[route]
    #[derive(Copy, Clone)]
    enum Route {
        #[route("report", ..)]
        Report(report::Route),
    }

    #[cache]
    fn route() -> Option<Route> {
        if let app::Route::Admin(route) = app::route() {
            Some(route)
        } else {
            None
        }
    }

    #[el]
    fn page() -> El {
        report::page()
    }

}
