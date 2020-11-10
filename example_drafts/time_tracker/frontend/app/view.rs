use zoons::*;

const MENU_BREAKPOINT: f64 = 600.;

zoons!{
    
    #[view]
    fn view() -> View {
        view![
            viewport::on_width_change(super::update_viewport_width),
            column![
                header(),
                menu_panel(),
                page(),
            ]
        ]
    }

    #[view]
    fn header() -> Row {
        let show_links = super::viewport_width().inner() > MENU_BREAKPOINT;
        let show_hamburger = !show_links;
        row![
            el![
                font::bold(),
                "TT",
            ],
            show_links.then(|| row![
                menu_links()
            ]),
            show_hamburger.then(hamburger),
        ]
    }

    #[view]
    fn hamburger() -> Checkbox {
        // @TODO aria-expanded? svg? animations? aria-label=menu?
        let menu_opened = super::menu_opened().inner();
        checkbox![
            checkbox::checked(menu_opened),
            checkbox::on_change(super::toggle_menu),
            input::label_hidden("Toggle Menu"),
            el![
                if checked {
                    "X"
                } else {
                    "E"
                }
            ],
        ]
    }

    #[view]
    fn menu_panel() -> Option<Column> {
        if !super::menu_opened().inner() {
            return None
        }
        if super::viewport_width().inner() > MENU_BREAKPOINT {
            return None
        }
        Some(column![
            menu_links(),
            username_or_login_button(),
        ])
    }

    #[view]
    fn menu_links() -> Vec<Link> {
        vec![
            link![
                link::url(super::Route::time_tracker()),
                "Time Tracker",
            ],
            link![
                link::url(super::Route::clients_and_projects()),
                "Clients & Projects",
            ],
            link![
                link::url(super::Route::time_blocks()),
                "Time Blocks",
            ],
        ]
    }

    #[view]
    fn username_or_login_button() -> Element {
        if let Some(user) = super::user().inner() {
            return user.name.into_element(),
        } 
        link![
            link::url(super::Route::login()),
            "Log in",
        ].into_element()
    }

    #[view]
    fn page() -> Element {
        el![
            width!(fill()),
            height!(fill()),
            match super::route().inner() {
                super::Route::Login => crate::login::view::page(),
                super::Route::ClientsAndProjects => crate::clients_and_projects::view::page(),
                super::Route::TimeTracker => crate::time_tracker::view::page(),
                super::Route::TimeBlocks => crate::time_blocks::view::page(),
                super::Route::Home => crate::home::view::page(),
                super::Route::Unknown => panic!("cannot display unknown page"),
            }
        ]
    }

}
