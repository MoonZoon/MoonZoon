use crate::*;

#[derive(Clone)]
pub struct Header {
    logged_user: Mutable<Option<Username>>,
}

impl Header {
    pub fn new(logged_user: Mutable<Option<Username>>) -> impl Element {
        Self { logged_user }.root()
    }

    fn root(&self) -> impl Element {
        Row::new()
            .s(Gap::both(20))
            .item(self.back_button())
            .item(self.link("Home", Route::Root))
            .item(self.link("Report", Route::ReportRoot))
            .item(self.link("Calc", Route::CalcRoot))
            .item_signal(
                self.logged_user
                    .signal_cloned()
                    .map(clone!((self => s) move |username| {
                        if let Some(username) = username {
                            s.log_out_button(username).left_either()
                        } else {
                            s.link("Log in", Route::Login).right_either()
                        }
                    })),
            )
    }

    fn back_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Background::new()
                .color_signal(hovered_signal.map_bool(|| color!("Green"), || color!("DarkGreen"))))
            .s(Padding::new().x(7).y(4))
            .s(RoundedCorners::all(4))
            .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
            .label("< Back")
            .on_press(routing::back)
    }

    fn link(&self, label: &str, route: Route) -> impl Element {
        Link::new()
            .s(Font::new()
                .color(color!("RoyalBlue"))
                .line(FontLine::new().underline()))
            .label(label)
            .to(route)
    }

    fn log_out_button(&self, Username(username): Username) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Background::new()
                .color_signal(hovered_signal.map_bool(|| color!("Red"), || color!("DarkRed"))))
            .s(Padding::new().x(7).y(4))
            .s(RoundedCorners::all(4))
            .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
            .label(format!("Log out {username}"))
            .on_press(clone!((self => s) move || {
                s.logged_user.set(None);
                ROUTER.go(Route::Root)
            }))
    }
}
