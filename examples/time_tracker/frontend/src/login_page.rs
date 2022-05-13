use crate::{
    app,
    connection::connection,
    router::{previous_route, router, Route},
};
use shared::{UpMsg, User};
use std::borrow::Cow;
use zoon::{eprintln, *};

mod view;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn login_error() -> &'static Mutable<Option<Cow<'static, str>>> {
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

pub fn set_login_error(error: String) {
    login_error().set(Some(Cow::from(error)))
}

fn log_in() {
    login_error().take();
    if name().map(String::is_empty) || password().map(String::is_empty) {
        login_error().set(Some(Cow::from("Sorry, invalid name or password.")));
        return;
    }
    Task::start(async {
        let msg = UpMsg::Login {
            name: name().get_cloned(),
            password: password().get_cloned(),
        };
        if let Err(error) = connection().send_up_msg(msg).await {
            let error = error.to_string();
            eprintln!("login request failed: {}", error);
            set_login_error(error);
        }
    });
}

fn set_name(new_name: String) {
    name().set_neq(new_name)
}

fn set_password(new_password: String) {
    password().set_neq(new_password)
}

pub fn set_and_store_logged_user(user: User) {
    if let Err(error) = local_storage().insert(app::USER_STORAGE_KEY, &user) {
        return set_login_error(error.to_string());
    }
    password().take();
    app::set_logged_user(user);
    router().go(previous_route().unwrap_or(Route::Root));
}

// ------ ------
//     View
// ------ ------

pub fn view() -> RawElement {
    view::page().into_raw_element()
}
