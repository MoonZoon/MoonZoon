use zoon::{*, println};
use shared::{UpMsg, DownMsg};
use crate::{app, login_page};

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|down_msg, cor_id| {
        println!("DownMsg received: {:?}", down_msg);

        app::unfinished_mutations().update_mut(|cor_ids| {
            cor_ids.remove(&cor_id);
        });
        match down_msg {
            // ------ Auth ------
            DownMsg::LoginError(error) => login_page::set_login_error(error),
            DownMsg::LoggedIn(user) => login_page::set_and_store_logged_user(user),
            DownMsg::LoggedOut => app::on_logged_out_msg(),
            DownMsg::AccessDenied => (),
            _ => ()
        }
    }).auth_token_getter(app::auth_token)
}
