use crate::{println, *};

mod calc;
pub use calc::CalcPage;

mod login;
pub use login::LoginPage;

mod report;
pub use report::ReportPage;

#[derive(Clone)]
pub struct Page {
    logged_user: Mutable<Option<Username>>,
}

impl Page {
    pub fn new() -> impl Element {
        Self {
            logged_user: Mutable::new(None),
        }
        .root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Align::new().center_x())
            .s(Padding::all(20))
            .s(Gap::both(20))
            .item(ui::Header::new(self.logged_user.clone()))
            .item(self.page_content())
    }

    fn page_content(&self) -> impl Element {
        El::new().child_signal(ROUTER.route().signal_cloned().map(clone!((self => s) move |route| {
            println!("{}", routing::url());
            match route {
                NoRoute => None,
                UnknownRoute => El::new().child("404").unify_option(),
                KnownRoute(route) => match route {
                    Route::ReportRoot => ReportPage::new(None, s.logged_user.read_only()).unify_option(),
                    Route::Report { frequency } => ReportPage::new(Some(frequency), s.logged_user.read_only()).unify_option(),
                    Route::Login => LoginPage::new(s.logged_user.clone()).unify_option(),
                    Route::CalcRoot => CalcPage::new(None).unify_option(),
                    Route::Calc { expression } => CalcPage::new(Some(Expression(expression))).unify_option(),
                    Route::Root => El::new().child("Welcome Home!").unify_option(),
                }
            }
        })))
    }
}
