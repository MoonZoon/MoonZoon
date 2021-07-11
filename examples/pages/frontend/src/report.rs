use crate::{route::Route, USER_NAME};
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
    fn from_route_segment(segment: &str) -> Option<Self> {
        match segment {
            DAILY => Some(Frequency::Daily),
            WEEKLY => Some(Frequency::Weekly),
            _ => None,
        }
    }

    fn into_route_segment(self) -> Cow<'static, str> {
        self.as_str()
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

pub fn set_frequency(frequency: Frequency) {
    frequency().set_neq(frequency);
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
    Row::new()
        .item(Text::with_signal(frequency().signal().map(greeting)))
        .item_signal(frequency_for_link().map(switch_frequency_link))
}

fn greeting(frequency: Frequency) -> impl Element {
    format!(
        "Hello {}! This is your {} report.",
        USER_NAME,
        frequency.as_str()
    )
}

fn switch_frequency_link(frequency: Frequency) -> impl Element {
    Link::new()
        .label(format!("Switch to {}", frequency.as_str()))
        .to(Route::ReportWithFrequency { frequency })
}
