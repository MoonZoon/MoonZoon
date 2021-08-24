use zoon::*;
use shared::{UpMsg, DownMsg, User};
use std::mem;
use std::collections::HashSet;
use crate::{
    router::{previous_route, router, Route},
};

pub mod view;

const USER_STORAGE_KEY: &str = "moonzoon-time_tracker-user";
const MENU_BREAKPOINT: u32 = 700;


// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Login,
    ClientsAndProjects,
    TimeTracker,
    TimeBlocks,
    Home,
    Unknown,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
pub fn logged_user() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

#[static_ref]
fn menu_opened() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn saving() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn viewport_width() -> &'static Mutable<u32> {
    Mutable::new(0)
}

// ------ ------
//    Helpers
// ------ ------

pub fn is_user_logged() -> bool {
    logged_user().map(Option::is_some)
}

// ------ ------
//   Handlers
// ------ ------

fn on_viewport_size_change(width: u32, _height: u32) {
    viewport_width().set(width)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

pub fn log_in(name: String) {
    logged_user().set(Some(name));
    router().go(previous_route().unwrap_or(Route::Root));
}

pub fn log_out() {
    logged_user().take();
    router().go(Route::Root);
}

fn toggle_menu() {
    menu_opened().update(|opened| !opened);
}

// ------ ------
//    Signals
// ------ ------


fn wide_screen() -> impl Signal<Item = bool> {
    viewport_width().signal().map(|width| width > MENU_BREAKPOINT).dedupe()
}

fn show_menu_panel() -> impl Signal<Item = bool> {
    map_ref! {
        let wide_screen = wide_screen(),
        let menu_opened = menu_opened().signal() =>
        not(wide_screen) && *menu_opened
    }
}

fn is_user_logged_signal() -> impl Signal<Item = bool> {
    logged_user().signal_ref(Option::is_some)
}




// blocks!{
//     append_blocks![
//         els,
//         crate::login,
//         crate::clients_and_projects,
//         crate::time_tracker,
//         crate::time_blocks,
//         crate::home,
//     ]

//     // ------ User ------

//     #[s_var]
//     fn user() -> Option<User> {
//         LocalStorage::get(USER_STORAGE_KEY).unwrap_or_default()
//     }

//     #[subscription]
//     fn store_user() {
//         user().use_ref(|user| {
//             if let Some(user) = user {
//                 LocalStorage::insert(USER_STORAGE_KEY, user);
//             } else {
//                 LocalStorage::remove(USER_STORAGE_KEY);
//             }
//         });
//     }

//     #[update]
//     fn login(password: &str) {
//         send_up_msg(false, UpMsg::Login(Cow::from(password)));
//     }

//     #[subscription]
//     fn handle_down_msg() {
//         listen(|msg: Option<DownMsg>| {
//             if let Some(DownMsg::LoggedIn(user)) = msg {
//                 user().set(Some(user));
//                 set_route(Route::home());
//                 return None
//             }
//             msg
//         })
//     }

//     #[update]
//     fn logout() {
//         send_up_msg(false, UpMsg::Logout);
//         user().set(None);
//         set_route(Route::home());
//     }

//     // ------ Viewport ------

//     #[s_var]
//     fn viewport_width() -> f64 {
//         0
//     }

//     #[update]
//     fn update_viewport_width(width: f64) {
//         viewport_width().set(width);
//     }

//     // ------ Menu ------

//     #[s_var]
//     fn menu_opened() -> bool {
//         false
//     }

//     #[update]
//     fn toggle_menu() {
//         menu_opened().update(not);
//     }

//     #[update]
//     fn close_menu() {
//         if menu_opened().inner() {
//             toggle_menu();
//         }
//     }

//     #[s_var]
//     fn close_menu_on_view_click() -> bool {
//         true
//     }

//     #[update]
//     fn menu_part_clicked() {
//         close_menu_on_view_click().set(false)
//     }

//     // ------ View ------

//     #[update]
//     fn view_clicked() {
//         if !close_menu_on_view_click().inner() {
//             return close_menu_on_view_click.set(true);
//         } 
//         close_menu();
//     }

//     // ------ Unfinished mutations ------

//     #[s_var]
//     fn unfinished_mutations() -> HashSet<CorId> {
//         HashSet::new()
//     }

//     #[cache]
//     fn saving() -> bool {
//         !unfinished_mutations().map(HashSet::is_empty)
//     }

//     // ------ Connection ------

//     #[s_var]
//     fn connection() -> Connection<UpMsg, DownMsg> {
//         Connection::new(|down_msg, cor_id| {
//             unfinished_mutations().update_mut(|cor_ids| {
//                 cor_ids.remove(cor_id);
//             });
//             notify(Some(down_msg));
//         })
//     }

//     #[update]
//     fn send_up_msg(mutation: bool, msg: UpMsg) {
//         let cor_id = connection().map(move |connection| {
//             let access_token = user().map(|user| { 
//                 user.map(|user| user.access_token)
//             });
//             connection.send_up_msg(msg, access_token);
//         });
//         if mutation {
//             unfinished_mutations().update(|cor_ids| {
//                 cor_ids.insert(cor_id);
//             });
//         }
//     }

// }
