use evalexpr::eval;
use zoon::*;

use std::borrow::Cow;
use std::sync::Arc;

mod router;
mod ui;

use router::*;

#[derive(Clone)]
pub struct Username(pub Arc<String>);

#[derive(Clone)]
pub struct Expression(pub Arc<Cow<'static, str>>);

#[derive(Copy, Clone, PartialEq, PartialOrd, Default)]
pub enum Frequency {
    Daily,
    #[default]
    Weekly,
}

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    ui::Page::new()
}
