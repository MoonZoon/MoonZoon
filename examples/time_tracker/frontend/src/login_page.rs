use zoon::*;
use crate::{app, router::{router, previous_route, Route}};

mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn login_error() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

#[static_ref]
fn name() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

#[static_ref]
fn password() -> &'static Mutable<String> {
    Mutable::new(String::new())
}

// ------ ------
//   Commands
// ------ ------

fn log_in() {
    login_error().take();
    if name().map(String::is_empty) || password().map(String::is_empty) {
        login_error().set(Some("Sorry, invalid name or password.".to_owned()));
        return;
    }

    app::logged_user().set(Some(name().take()));
    router().go(previous_route().unwrap_or(Route::Root));
}

fn set_name(new_name: String) {
    name().set_neq(new_name)
}

fn set_password(new_password: String) {
    password().set_neq(new_password)
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}


// blocks!{
//     append_blocks![els]

//     #[subscription]
//     fn on_route_change() {
//         if let app::Route::Login = route() {
//             set_password(String::new());
//             invalid_password().set(false);
//         }
//     }

//     #[subscription]
//     fn handle_down_msg() {
//         listen(|msg: Option<DownMsg>| {
//             if let Some(DownMsg::InvalidPassword) = msg {
//                 invalid_password().set(true)
//             }
//             msg
//         })
//     }

//     #[s_var]
//     fn password() -> String {
//         String::new()
//     }

//     #[update]
//     fn set_password(password: String) {
//         password().set(password)
//     }

//     #[s_var]
//     fn invalid_password() -> bool {
//         false
//     }

//     #[update]
//     fn login() {
//         invalid_password().set(false);
//         password().use_ref(|password| {
//             app::login(password);
//         })
//     }
// }
