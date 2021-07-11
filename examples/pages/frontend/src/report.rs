use zoon::*;
use crate::USER_NAME;

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
            Self::Daily => "daily",
            Self::Weekly => "weekly",
        }
    }
}

impl FromRouteSegment for Frequency {
    fn from_route_segment(segment: &str) -> Option<Self> {
        match segment {
            "daily" => Some(Frequency::Daily),
            "weekly" => Some(Frequency::Weekly),
            _ => None,
        }
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
    use Frequency::{Daily, Weekly};
    frequency().signal().map(|frequency| {
        if let Daily = frequency { Weekly } else { Daily }
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
    format!("Hello {}! This is your {} report.", USER_NAME, frequency.as_str())
}

fn switch_frequency_link(frequency: Frequency) -> impl Element {
    Link::new()
        .label(format!("Switch to {}", frequency.as_str()))
        .to(Route::report_with_frequency(frequency))
}
