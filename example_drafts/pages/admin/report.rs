use zoon::*;
use crate::app;

zoons!{

    #[route]
    #[derive(Copy, Clone)]
    enum Route {
        #[route(frequency)]
        Frequency(Option<Frequency>),
    }

    #[cache]
    fn route() -> Option<Route> {
        if let Some(admin::Route::Report(route)) = admin::route() {
            Some(route)
        } else {
            None
        }
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

    #[subscription]
    fn update_frequency() {
        route().map(|route| {
            if let Some(Route::Frequency(Some(frequency))) = route {
                frequency().set(frequency)
            }
        })
    }

    #[var]
    fn frequency() -> Frequency {
        Frequency::Daily
    }

    #[cache]
    fn frequency_for_link() -> Frequency {
        use Frequency::{Daily, Weekly};
        if let Daily = frequency().inner() { Weekly } else { Daily }
    }

    #[el]
    fn page() -> El {
        let logged_user = app::logged_user().inner();
        let frequency = frequency().inner().as_str();
        let frequency_for_link = frequency_for_link().inner();
        el![
            row![
                format!(
                    "Hello {}! This is your {} report.", 
                    logged_user, frequency,
                ),
                link![
                    link::url(Route::frequency(frequency_for_link)),
                    format!{"Switch to {}", frequency_for_link.as_str()}
                ]
            ]
        ]
    }

}
