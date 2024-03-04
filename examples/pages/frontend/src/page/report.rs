use crate::*;

pub struct ReportPage;
impl ReportPage {
    pub fn new(frequency: Option<Frequency>) -> Option<impl Element> {
        if STORE.logged_user.lock_ref().is_none() {
            ROUTER.replace(Route::Login);
            return None;
        }
        if let Some(frequency) = frequency {
            STORE.report_page.frequency.set_neq(frequency);
        }
        Some(page_content())
    }
}

fn page_content() -> impl Element {
    Column::new()
        .s(Gap::both(20))
        .item_signal(
            map_ref! {
                let frequency = STORE.report_page.frequency.signal().map(|frequency| match frequency {
                    Frequency::Daily => "daily",
                    Frequency::Weekly => "weekly"
                }),
                let username = STORE.logged_user.signal_cloned() => {
                    username.as_ref().map(|username| format!("Hello {username}! This is your {frequency} report."))
                }
            }
        )
        .item(switch_frequency_link())
}

fn switch_frequency_link() -> impl Element {
    Link::new()
        .s(Font::new()
            .color(color!("RoyalBlue"))
            .line(FontLine::new().underline()))
        .label_signal(STORE.report_page.frequency.signal_ref(|frequency| {
            if let Frequency::Daily = frequency {
                "Switch to weekly"
            } else {
                "Switch to daily"
            }
        }))
        .to_signal(STORE.report_page.frequency.signal_ref(|frequency| {
            if let Frequency::Daily = frequency {
                Route::Report {
                    frequency: Frequency::Weekly,
                }
            } else {
                Route::Report {
                    frequency: Frequency::Daily,
                }
            }
        }))
}
