use crate::*;

pub static STORE: Lazy<Store> = lazy::default();

pub type Username = String;

#[derive(Default)]
pub struct Store {
    pub logged_user: Mutable<Option<Username>>,
    pub calc_page: CalcPageStore,
    pub login_page: LoginPageStore,
    pub report_page: ReportPageStore,
}

#[derive(Default)]
pub struct CalcPageStore {
    pub expression: Mutable<Option<Arc<Cow<'static, str>>>>,
}

#[derive(Educe)]
#[educe(Default)]
pub struct LoginPageStore {
    #[educe(Default(expression = Mutable::new("John".to_owned())))]
    pub username: Mutable<String>,
}

#[derive(Default)]
pub struct ReportPageStore {
    pub frequency: Mutable<Frequency>,
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum Frequency {
    Daily,
    #[default]
    Weekly,
}
