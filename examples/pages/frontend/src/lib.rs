use zoon::*;

mod report;
mod route;

use route::{Route, router};

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum PageId {
    Report,
    Home,
    Unknown,
}

// ------ ------
//    Statics
// ------ ------

static USER_NAME: &str = "John Doe";

#[static_ref]
fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Unknown)
}

// ------ ------
//   Commands
// ------ ------

fn set_page_id(page_id: PageId) {
    page_id().set_neq(page_id);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .item(header())
        .item(page())
}

fn header() -> impl Element {
    Row::new()
        .item(Link::new().label("Home").to(Route::Root))
        .item(Link::new().label("Report").to(Route::Report))
}

fn page() -> impl Element {
    El::new()
        .child_signal(page_id().signal().map(|page_id| match page_id {
            PageId::Report => report::page().into_raw_element(),
            PageId::Home => El::new().child("welcome home!").into_raw_element(),
            PageId::Unknown => El::new().child("404").into_raw_element(),
        }))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    router();
    start_app("app", root);
}
