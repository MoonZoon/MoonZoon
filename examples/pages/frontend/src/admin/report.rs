use zoon::*;
use crate::USER_NAME;

#[route]
#[derive(Copy, Clone)]
enum Route {
    #[route(frequency)]
    Frequency(Frequency),
    #[route()]
    Root,
}

#[route_arg]
#[derive(Copy, Clone)]
enum Frequency {
    #[route_arg("daily")]
    Daily,
    #[route_arg("weekly")]
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

// ------ ------
//     View
// ------ ------

fn page(route: Route) -> impl Element {
    use Frequency::{Daily, Weekly};
    let frequency = match route {
        Route::Frequency(frequency) => frequency,
        Route::Root => Frequency::Daily,
    };
    let frequency_for_link = if let Daily = frequency { Weekly } else { Daily };
    Row::new()
        .item(format!("Hello {}! This is your {} report.", USER_NAME, frequency.as_str()))
        .item(Link::new()
            .label(format!("Switch to {}", frequency_for_link.as_str()))
            .to(Route::frequency(frequency_for_link))
        )
}
