use zoon::*;

// ------ Route ------

#[route]
#[derive(Copy, Clone)]
enum Route {
    #[route("admin", "report", frequency)]
    AdminReport(Option<Frequency>),
    #[route()]
    Root,
    Unknown,
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

#[cache]
fn route() -> Route {
    zoon::model::url().map(Route::from)
}

#[sub]
fn update_frequency() {
    route().map(|route| {
        if let app::Route::AdminReport(Some(frequency)) = route {
            frequency().set(frequency)
        }
    })
}

#[model]
fn frequency() -> Frequency {
    Frequency::Daily
}

#[cache]
fn frequency_for_link() -> Frequency {
    use Frequency::{Daily, Weekly};
    if let Daily = frequency().inner() { Weekly } else { Daily }
}

#[model]
fn logged_user() -> &'static str {
    "John Doe"
}
