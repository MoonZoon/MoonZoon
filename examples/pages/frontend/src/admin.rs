use zoon::*;

mod report;

#[route]
#[derive(Copy, Clone)]
enum Route {
    #[route("report", ..)]
    Report(report::Route),
}

// ------ ------
//     View
// ------ ------

fn page(route: Route) -> impl Element {
    let Route::Report(route) = route;
    report::page(route)
}
