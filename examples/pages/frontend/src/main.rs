use evalexpr::eval;
use zoon::{println, *};

use std::borrow::Cow;
use std::sync::Arc;

mod page;
mod router;
mod store;
mod ui;

use page::*;
use router::*;
use store::*;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    ui::Page::new(ROUTER.route().signal_cloned().map(|route| {
        println!("{}", routing::url());
        match route {
            NoRoute => None,
            UnknownRoute => El::new().child("404").unify_option(),
            KnownRoute(route) => match route {
                Route::ReportRoot => ReportPage::new(None).unify_option(),
                Route::Report { frequency } => ReportPage::new(Some(frequency)).unify_option(),
                Route::Login => LoginPage::new().unify_option(),
                Route::CalcRoot => CalcPage::new(None).unify_option(),
                Route::Calc { expression } => CalcPage::new(Some(expression)).unify_option(),
                Route::Root => El::new().child("Welcome Home!").unify_option(),
            },
        }
    }))
}
