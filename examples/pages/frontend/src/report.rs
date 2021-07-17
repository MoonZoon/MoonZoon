use crate::{router::Route, app};
use std::borrow::Cow;
use zoon::*;

const DAILY: &str = "daily";
const WEEKLY: &str = "weekly";

// ------ ------
//     Types
// ------ ------

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Frequency {
    Daily,
    Weekly,
}

impl Frequency {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Daily => DAILY,
            Self::Weekly => WEEKLY,
        }
    }
}

impl RouteSegment for Frequency {
    fn from_string_segment(segment: &str) -> Option<Self> {
        match segment {
            DAILY => Some(Frequency::Daily),
            WEEKLY => Some(Frequency::Weekly),
            _ => None,
        }
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        self.as_str().into()
    }
}

// ------ ------
//    Statics
// ------ ------

#[static_ref]
fn frequency() -> &'static Mutable<Frequency> {
    Mutable::new(Frequency::Daily)
}

// ------ ------
//   Commands
// ------ ------

pub fn set_frequency(new_frequency: Frequency) {
    frequency().set_neq(new_frequency);
}

// ------ ------
//    Signals
// ------ ------

fn frequency_for_link() -> impl Signal<Item = Frequency> {
    frequency().signal().map(|frequency| {
        if let Frequency::Daily = frequency {
            Frequency::Weekly
        } else {
            Frequency::Daily
        }
    })
}

// ------ ------
//     View
// ------ ------

pub fn page() -> impl Element {
    Column::new()
        .s(Spacing::new(20))
        .item(greeting())
        .item(switch_frequency_link())
}

fn greeting() -> impl Element {
    let greeting = move |frequency: Frequency| format!(
        "Hello {}! This is your {} report.",
        app::logged_user().lock_ref().as_ref().unwrap_throw(),
        frequency.as_str()
    );
    Text::with_signal(frequency().signal().map(greeting))
}

fn switch_frequency_link() -> impl Element {
    Link::new()
        .s(Font::new().underline().color(NamedColor::Blue7))
        .label_signal(frequency_for_link().map(|frequency| {
            format!("Switch to {}", frequency.as_str())
        }))
        .to_signal(frequency_for_link().map(|frequency| {
            Route::ReportWithFrequency { frequency }
        }))
}
