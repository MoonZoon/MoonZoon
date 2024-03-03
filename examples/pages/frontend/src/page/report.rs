use crate::*;

pub fn maybe_view(frequency: impl Into<Option<Frequency>>) -> Option<RawElOrText> {
    if STORE.logged_user.lock_ref().is_none() {
        ROUTER.replace(Route::Login);
        return None;
    }
    if let Some(frequency) = frequency.into() {
        STORE.report_page.frequency.set_neq(frequency);
    }
    Some(page_content().into_raw())
}

fn page_content() -> impl Element {
    Column::new()
        .s(Gap::both(20))
        .item(greeting())
        .item(switch_frequency_link())
}

fn greeting() -> impl Element {
    let greeting = move |frequency: Frequency| {
        format!(
            "Hello {}! This is your {} report.",
            STORE.logged_user.lock_ref().as_ref().unwrap_throw(),
            match frequency {
                Frequency::Daily => "daily",
                Frequency::Weekly => "weekly",
            }
        )
    };
    Text::with_signal(STORE.report_page.frequency.signal().map(greeting))
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
