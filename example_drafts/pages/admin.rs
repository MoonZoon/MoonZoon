use zoon::*;
use crate::app;

mod report;

zoons!{
    append_zoons![report::zoons]

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

    #[view]
    fn view() -> El {
        report::view()
    }

}
