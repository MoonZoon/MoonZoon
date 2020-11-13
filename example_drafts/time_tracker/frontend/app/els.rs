use zoons::*;

const MENU_BREAKPOINT: f64 = 700.;

blocks!{
    
    #[el]
    fn root() -> View {
        view![
            viewport::on_width_change(super::update_viewport_width),
            on_click(super::view_clicked),
            column![
                header(),
                menu_panel(),
                page(),
            ]
        ]
    }

    #[el]
    fn header() -> Row {
        let show_links = super::viewport_width().inner() > MENU_BREAKPOINT;
        let show_hamburger = !show_links;
        let saving = super::saving();
        row![
            el![
                font::bold(),
                "TT",
            ],
            show_links.then(|| row![menu_links()]),
            saving.then(|| el!["Saving..."]),
            show_hamburger.then(hamburger),
        ]
    }

    #[el]
    fn hamburger() -> Button {
        let menu_opened = super::menu_opened().inner();
        button![
            button::on_press(super::toggle_menu),
            on_click(super::menu_part_clicked),
            if menu_opened { "X" } else { "☰" }
        ]
    }

    #[el]
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
            on_click(super::menu_part_clicked),
        ])
    }

    #[el]
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

    #[el]
    fn username_or_login_button() -> Element {
        if let Some(user) = super::user().inner() {
            return user.name.into_element(),
        } 
        link![
            link::url(super::Route::login()),
            "Log in",
        ].into_element()
    }

    #[el]
    fn page() -> Element {
        el![
            width!(fill()),
            height!(fill()),
            match super::route().inner() {
                super::Route::Login => crate::login::els::page(),
                super::Route::ClientsAndProjects => crate::clients_and_projects::els::page(),
                super::Route::TimeTracker => crate::time_tracker::els::page(),
                super::Route::TimeBlocks => crate::time_blocks::els::page(),
                super::Route::Home => crate::home::els::page(),
                super::Route::Unknown => panic!("cannot display unknown page"),
            }
        ]
    }

}
