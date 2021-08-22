use zoon::*;

const MENU_BREAKPOINT: f64 = 700.;

pub fn root() -> impl Element {
    // view![
    //     viewport::on_width_change(super::update_viewport_width),
    //     on_click(super::view_clicked),
    //     column![
    //         header(),
    //         menu_panel(),
    //         page(),
    //     ]
    // ]
    Text::new("Time tracker!")
}

// #[el]
// fn header() -> Row {
//     let show_links_and_controls = super::viewport_width().inner() > MENU_BREAKPOINT;
//     let show_hamburger = !show_links;
//     let saving = super::saving();
//     row![
//         el![
//             font::bold(),
//             "TT",
//         ],
//         show_link_and_controls.then(|| row![menu_links()]),
//         saving.then(|| el!["Saving..."]),
//         show_link_and_controls.then(auth_controls),
//         show_hamburger.then(hamburger),
//     ]
// }

// #[el]
// fn hamburger() -> Button {
//     let menu_opened = super::menu_opened().inner();
//     button![
//         button::on_press(super::toggle_menu),
//         on_click(super::menu_part_clicked),
//         if menu_opened { "X" } else { "☰" }
//     ]
// }

// #[el]
// fn menu_panel() -> Option<Column> {
//     if !super::menu_opened().inner() {
//         return None
//     }
//     if super::viewport_width().inner() > MENU_BREAKPOINT {
//         return None
//     }
//     Some(column![
//         menu_links(),
//         auth_controls(),
//         on_click(super::menu_part_clicked),
//     ])
// }

// #[el]
// fn menu_links() -> Vec<Link> {
//     vec![
//         link![
//             link::url(super::Route::time_tracker()),
//             "Time Tracker",
//         ],
//         link![
//             link::url(super::Route::clients_and_projects()),
//             "Clients & Projects",
//         ],
//         link![
//             link::url(super::Route::time_blocks()),
//             "Time Blocks",
//         ],
//     ]
// }

// #[el]
// fn auth_controls() -> Row {
//     row![
//         username_or_login_button(),
//         logout_button(),
//     ]
// }

// #[el]
// fn username_or_login_button() -> Element {
//     if let Some(user) = super::user().inner() {
//         return user.name.into_element(),
//     } 
//     link![
//         link::url(super::Route::login()),
//         "Log in",
//     ].into_element()
// }

// #[el]
// fn logout_button() -> Option<Button> {
//     let logged_in = super::user().map(Option::is_some);
//     logged_in.then({
//         button![
//             button::on_press(super::logout),
//             "Log out",
//         ]
//     })
// }

// fn page() -> impl Element {
//     El::new()
//         .s(Width::fill())
//         .s(Height::fill())
//         .child_signal(page_id().signal().map(|page_id| match page_id {
//             super::Route::Login => login_page::view::page(),
//             super::Route::ClientsAndProjects => clients_and_projects_page::view::page().into_raw_element(),
//             super::Route::TimeTracker => time_tracker_page::view::page().into_raw_element(),
//             super::Route::TimeBlocks => time_blocks_page::view::page().into_raw_element(),
//             super::Route::Home => home_page::view::page().into_raw_element(),
//             super::Route::Unknown => El::new().child(404).into_raw_element(),
//         }))
// }
