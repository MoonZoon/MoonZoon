use zoon::*;
use shared::{UpMsg, DownMsg, User};
use std::mem;
use std::collections::HashSet;

mod els;

const USER_STORAGE_KEY: &str = "time_tracker-moonzoon-user";

blocks!{
    append_blocks![
        els,
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

    #[s_var]
    fn user() -> Option<User> {
        LocalStorage::get(USER_STORAGE_KEY).unwrap_or_default()
    }

    #[subscription]
    fn store_user() {
        user().use_ref(|user| LocalStorage::insert(USER_STORAGE_KEY, user));
    }

    #[update]
    fn login(password: &str) {
        send_up_msg(false, UpMsg::Login(Cow::from(password)));
    }

    #[subscription]
    fn handle_down_msg() {
        listen(|msg: Option<DownMsg>| {
            if let Some(DownMsg::LoggedIn(user)) = msg {
                user().set(Some(user));
                set_route(Route::home());
                return None
            }
            msg
        })
    }

    #[update]
    fn logout() {
        send_up_msg(false, UpMsg::Logout);
        user().set(None);
        set_route(Route::home());
    }

    // ------ Viewport ------

    #[s_var]
    fn viewport_width() -> f64 {
        0
    }

    #[update]
    fn update_viewport_width(width: f64) {
        viewport_width().set(width);
    }

    // ------ Menu ------

    #[s_var]
    fn menu_opened() -> bool {
        false
    }

    #[update]
    fn toggle_menu() {
        menu_opened().update(not);
    }

    #[update]
    fn close_menu() {
        if menu_opened().inner() {
            toggle_menu();
        }
    }

    #[s_var]
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

    // ------ Unfinished mutations ------

    #[s_var]
    fn unfinished_mutations() -> HashSet<CorId> {
        HashSet::new()
    }

    #[cache]
    fn saving() -> bool {
        !unfinished_mutations().map(HashSet::is_empty)
    }

    // ------ Connection ------

    #[s_var]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new(|down_msg, cor_id| {
            unfinished_mutations().update_mut(|cor_ids| {
                cor_ids.remove(cor_id);
            });
            notify(Some(down_msg));
        })
    }

    #[update]
    fn send_up_msg(mutation: bool, msg: UpMsg) {
        let cor_id = connection().map(move |connection| {
            let access_token = user().map(|user| { 
                user.map(|user| user.access_token)
            });
            connection.send_up_msg(msg, access_token);
        });
        if mutation {
            unfinished_mutations().update(|cor_ids| {
                cor_ids.insert(cor_id);
            });
        }
    }

}
