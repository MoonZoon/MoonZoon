// use zoon::*;
// use crate::app;

pub mod view;

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
