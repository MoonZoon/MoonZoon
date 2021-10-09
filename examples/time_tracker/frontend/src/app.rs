use zoon::{*, eprintln};
use crate::{
    router::{router, Route},
    connection::connection,
};
use std::collections::BTreeSet;
use shared::{User, UpMsg};

mod view;

pub static USER_STORAGE_KEY: &str = "moonzoon-time_tracker-user";
pub const DEBOUNCE_MS: u32 = 800;
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
pub fn logged_user() -> &'static Mutable<Option<User>> {
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
pub fn viewport_width() -> &'static Mutable<u32> {
    Mutable::new(0)
}

#[static_ref]
fn hamburger_class_id() -> &'static Mutable<ClassId> {
    Mutable::new(ClassId::default())
}

#[static_ref]
pub fn unfinished_mutations() -> &'static Mutable<BTreeSet<CorId>> {
    Mutable::new(BTreeSet::new())
}

// ------ ------
//    Helpers
// ------ ------

pub fn is_user_logged() -> bool {
    logged_user().map(Option::is_some)
}

fn logged_user_name() -> Option<String> {
    Some(logged_user().lock_ref().as_ref()?.name.clone())
}

pub fn auth_token() -> Option<AuthToken> {
    Some(logged_user().lock_ref().as_ref()?.auth_token.clone())
}

// ------ ------
//   Handlers
// ------ ------

fn on_viewport_size_change(width: u32, _height: u32) {
    viewport_width().set(width)
}

pub fn on_logged_out_msg() {
    logged_user().take();
    local_storage().remove(USER_STORAGE_KEY);
    router().go(Route::Root);
}

// ------ ------
//   Commands
// ------ ------

pub fn set_logged_user(user: User) {
    logged_user().set(Some(user));
}

pub fn load_logged_user() {
    if let Some(Ok(user)) = local_storage().get(USER_STORAGE_KEY) {
        set_logged_user(user);
    }
}

pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

pub fn log_out() {
    Task::start(async {
        if let Err(error) = connection().send_up_msg(UpMsg::Logout).await {
            eprintln!("logout request failed: {}", error)
        }
    });
}

fn toggle_menu() {
    menu_opened().update(|opened| !opened);
}

pub fn close_menu() {
    menu_opened().set_neq(false);
}

fn set_hamburger_class_id(class_id: ClassId) {
    hamburger_class_id().set(class_id);
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

// ------ ------
//     View
// ------ ------

pub fn root() -> impl Element {
    view::root()
}

pub fn icon_add() -> impl Element {
    RawHtmlEl::new("i")
        .class("eos-icons")
        .child("add")
}

pub fn icon_delete_forever() -> impl Element {
    RawHtmlEl::new("i")
        .class("eos-icons")
        .child("delete_forever")
}

pub fn icon_open_in_new() -> impl Element {
    RawHtmlEl::new("i")
        .class("eos-icons")
        .child("open_in_new")
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
