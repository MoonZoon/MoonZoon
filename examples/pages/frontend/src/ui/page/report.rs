use crate::*;

#[derive(Clone)]
pub struct ReportPage {
    frequency: Frequency,
    logged_user: ReadOnlyMutable<Option<Username>>,
}

impl ReportPage {
    pub fn new(
        frequency: Option<Frequency>,
        logged_user: ReadOnlyMutable<Option<Username>>,
    ) -> Option<impl Element> {
        if logged_user.lock_ref().is_none() {
            ROUTER.replace(Route::Login);
            return None;
        }
        Some(
            Self {
                frequency: frequency.unwrap_or_default(),
                logged_user,
            }
            .root(),
        )
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Gap::both(20))
            .item_signal({
                let frequency = match self.frequency {
                    Frequency::Daily => "daily",
                    Frequency::Weekly => "weekly",
                };
                self.logged_user
                    .signal_cloned()
                    .map_some(move |Username(username)| {
                        format!("Hello {username}! This is your {frequency} report.")
                    })
            })
            .item(self.switch_frequency_link())
    }

    fn switch_frequency_link(&self) -> impl Element {
        Link::new()
            .s(Font::new()
                .color(color!("RoyalBlue"))
                .line(FontLine::new().underline()))
            .label(match self.frequency {
                Frequency::Daily => "Switch to weekly",
                Frequency::Weekly => "Switch to daily",
            })
            .to(match self.frequency {
                Frequency::Daily => Route::Report {
                    frequency: Frequency::Weekly,
                },
                Frequency::Weekly => Route::Report {
                    frequency: Frequency::Daily,
                },
            })
    }
}
