use evalexpr::eval;
use zoon::{println, *};

use std::borrow::Cow;
use std::sync::Arc;

mod page;
mod router;
mod store;
mod ui;

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
                Route::ReportRoot => page::report::maybe_view(None).unify_option(),
                Route::Report { frequency } => page::report::maybe_view(frequency).unify_option(),
                Route::Login => page::login::maybe_view().unify_option(),
                Route::CalcRoot => page::calc::maybe_view(None).unify_option(),
                Route::Calc { expression } => page::calc::maybe_view(expression).unify_option(),
                Route::Root => El::new().child("Welcome Home!").unify_option(),
            },
        }
    }))
}
