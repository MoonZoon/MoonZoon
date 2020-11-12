use zoon::*;

mod admin;

blocks!{
    append_blocks![admin]

    #[route]
    #[derive(Copy, Clone)]
    enum Route {
        #[route("admin", ..)]
        Admin(admin::Route),
        #[route()]
        Root,
        Unknown,
    }

    #[cache]
    fn route() -> Route {
        url().map(Route::from)
    }

    #[var]
    fn logged_user() -> &'static str {
        "John Doe"
    }

    #[el]
    fn root() -> Column {
        column![
            header(),
            page(),
        ]
    }

    #[el]
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

    #[el]
    fn page() -> El {
        let route = route().inner();
        match route {
            Route::Admin(_) => {
                admin::page()
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

}
