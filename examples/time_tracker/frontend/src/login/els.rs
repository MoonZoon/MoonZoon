use zoon::*;
use crate::app;

blocks!{

    #[el]
    fn page() -> Column {
        column![
            el![
                region::h1(),
                "Login",
            ],
            login_form(),
            error(),
        ]
    }

    #[el]
    fn login_form() -> Row {
        row![
            password_input(),
            button![
                button::on_press(super::login),
                "Log in",
            ]
        ]
    }

    #[el]
    fn password_input() -> TextInput {
        let password = super::password().inner();
        text_input![
            do_once(focus),
            text_input::on_change(super::set_password),
            on_key_down(|event| {
                if let Key::Enter = event.key {
                    super::login()
                }
            }),
            password,
        ]
    }

    #[el]
    fn error() -> Option<El> {
        super::invalid_password().inner().then(|| {
            el![
                "Sorry, this isn't a valid password."
            ]
        })
    }
    
}
