use zoon::*;

mod admin;

#[route]
#[derive(Copy, Clone)]
enum Route {
    #[route("admin", ..admin)]
    Admin(admin::Route),
    #[route()]
    Root,
    Unknown,
}

#[cache]
fn route() -> Route {
    zoon::model::url().map(Route::from)
}

#[model]
fn logged_user() -> &'static str {
    "John Doe"
}

#[view]
fn view() -> Column {
    column![
        header(),
        page(),
    ]
}

#[view]
fn header() -> Row {
    row![
        link![
            link::url(Route::root())
            "Home"
        ],
        link![
            link::url(Route::admin().report().frequency(None)),
            "Report"
        ],
    ]
}

#[view]
fn page() -> El {
    let route = route().inner();

    match route {
        Route::Admin(_) => {
            admin::view()
        }
        Route::Root => {
            el![
                "welcome home!",
            ]
        }
        Route::Unknown => {
            el![
                "404"
            ]
        }
    }
}
