use crate::*;

pub static STORE: Lazy<Store> = lazy::default();

pub type Username = String;

#[derive(Default)]
pub struct Store {
    pub logged_user: Mutable<Option<Username>>,
    pub calc_page: CalcPage,
    pub login_page: LoginPage,
    pub report_page: ReportPage,
}

#[derive(Default)]
pub struct CalcPage {
    pub expression: Mutable<Option<Arc<Cow<'static, str>>>>,
}

#[derive(Educe)]
#[educe(Default)]
pub struct LoginPage {
    #[educe(Default(expression = Mutable::new("John".to_owned())))]
    pub username: Mutable<String>,
}

#[derive(Default)]
pub struct ReportPage {
    pub frequency: Mutable<Frequency>,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum Frequency {
    Daily,
    #[default]
    Weekly,
}
