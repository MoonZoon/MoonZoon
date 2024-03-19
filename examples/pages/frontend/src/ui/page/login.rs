use crate::*;

#[derive(Clone)]
pub struct LoginPage {
    username: Mutable<Username>,
    logged_user: Mutable<Option<Username>>,
}

impl LoginPage {
    pub fn new(logged_user: Mutable<Option<Username>>) -> Option<impl Element> {
        if logged_user.lock_ref().is_some() {
            ROUTER.replace(Route::Root);
            return None;
        }
        Some(
            Self {
                username: Mutable::new(Username(Arc::new("John".to_owned()))),
                logged_user,
            }
            .root(),
        )
    }

    fn root(&self) -> impl Element {
        Row::new()
            .item(self.name_input())
            .item(self.log_in_button())
    }

    fn name_input(&self) -> impl Element {
        TextInput::new()
            .s(Padding::all(7))
            .label_hidden("Name")
            .placeholder(Placeholder::new("John"))
            .text_signal(
                self.username
                    .signal_cloned()
                    .map(|Username(username)| username),
            )
            .on_change(clone!((self => s) move |name| s.username.set(Username(Arc::new(name)))))
    }

    fn log_in_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Background::new()
                .color_signal(hovered_signal.map_bool(|| color!("Green"), || color!("DarkGreen"))))
            .s(Padding::all(7))
            .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
            .label("Log in")
            .on_press(clone!((self => s) move || {
                s
                    .logged_user
                    .set(Some(s.username.get_cloned()));
                ROUTER.go_to_previous_known_or_else(|| Route::Root);
            }))
    }
}
