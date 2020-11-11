use zoon::*;
use shared::{UpMsg, DownMsg, User};
use std::mem;

mod view;

zoons!{
    append_zoons![
        view,
        crate::login,
        crate::clients_and_projects,
        crate::time_tracker,
        crate::time_blocks,
        crate::home,
    ]

    // ------ Route ------

    #[route]
    #[derive(Copy, Clone)]
    enum Route {
        #[route("login")]
        #[before_route(before_login_route)]
        Login,

        #[route("clients_and_projects")]
        #[before_route(before_protected_route)]
        ClientsAndProjects,

        #[route("time_tracker")]
        #[before_route(before_protected_route)]
        TimeTracker,

        #[route("time_blocks")]
        #[before_route(before_protected_route)]
        TimeBlocks,

        #[route()]
        Home,

        #[before_route(before_unknown_route)]
        Unknown,
    }

    fn before_login_route(route: Route) -> Route {
        if user().map(Option::is_none) {
            return route
        }
        Route::home()
    }

    fn before_protected_route(route: Route) -> Route {
        if user().map(Option::is_some) {
            return route
        }
        Route::login()
    }

    fn before_unknown_route(route: Route) -> Route {
        Route::home()
    }

    #[cache]
    fn route() -> Route {
        url().map(Route::from)
    }

    #[subscription]
    fn on_route_change() {
        route();
        close_menu();
    }

    #[update]
    fn set_route(route: Route) {
        url().set(Url::from(route))
    }

    // ------ User ------

    #[model]
    fn user() -> Option<User> {
        None
    }

    #[update]
    fn set_user(user: User) {
        user().set(user)
    }

    // ------ Viewport ------

    #[model]
    fn viewport_width() -> f64 {
        0
    }

    #[update]
    fn update_viewport_width(width: f64) {
        viewport_width().set(width);
    }

    // ------ Menu ------

    #[model]
    fn menu_opened() -> bool {
        false
    }

    #[update]
    fn toggle_menu() {
        menu_opened().update(|opened| *opened = !opened);
    }

    #[update]
    fn close_menu() {
        if menu_opened().inner() {
            toggle_menu();
        }
    }

    #[model]
    fn close_menu_on_view_click() -> bool {
        true
    }

    #[update]
    fn menu_part_clicked() {
        close_menu_on_view_click().set(false)
    }

    // ------ View ------

    #[update]
    fn view_clicked() {
        if !close_menu_on_view_click().inner() {
            return close_menu_on_view_click.set(true);
        } 
        close_menu();
    }

    // ------ Connection ------

    #[model]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new("localhost:9000", |msg| {
            if let DownMsg::ClientsAndProjectsClients(clients) = msg {
                crate::clients_and_projects::set_clients(clients);
            } 
        })
    }

}
