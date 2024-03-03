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
            UnknownRoute => Some(El::new().child("404").into_raw()),
            KnownRoute(route) => match route {
                Route::ReportRoot => page::report::maybe_view(None),
                Route::Report { frequency } => page::report::maybe_view(frequency),
                Route::Login => page::login::maybe_view(),
                Route::CalcRoot => page::calc::maybe_view(None),
                Route::Calc { expression } => page::calc::maybe_view(expression),
                Route::Root => Some(El::new().child("Welcome Home!").into_raw()),
            },
        }
    }))
}
