use crate::*;

pub static ROUTER: Lazy<Router<Route>> = lazy::default();

// ------ Route ------

#[route]
#[derive(Clone)]
pub enum Route {
    #[route("report")]
    ReportRoot,
    #[route("report", frequency)]
    Report { frequency: Frequency },

    #[route("login")]
    Login,

    #[route("calc")]
    CalcRoot,
    #[route("calc", "expression", expression)]
    Calc { expression: Arc<Cow<'static, str>> },

    #[route()]
    Root,
}

// ------ RouteSegment implementations ------

const DAILY: &str = "daily";
const WEEKLY: &str = "weekly";

impl RouteSegment for Frequency {
    fn from_string_segment(segment: &str) -> Option<Self> {
        match segment {
            DAILY => Some(Self::Daily),
            WEEKLY => Some(Self::Weekly),
            _ => None,
        }
    }

    fn into_string_segment(self) -> Cow<'static, str> {
        match self {
            Self::Daily => DAILY.into(),
            Self::Weekly => WEEKLY.into(),
        }
    }
}
