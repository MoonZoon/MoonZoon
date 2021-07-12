use zoon::*;

mod report;
mod route;

use route::{router, Route};

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
    Mutable::new(PageId::Report)
}

// ------ ------
//   Commands
// ------ ------

fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Padding::new().all(20))
        .s(Spacing::new(20))
        .item(header())
        .item(page())
}

fn header() -> impl Element {
    Row::new()
        .s(Spacing::new(20))
        .item(
            Link::new()
                .s(Font::new().underline().color(NamedColor::Blue7))
                .label("Home")
                .to(Route::Root))
        .item(
            Link::new()
                .s(Font::new().underline().color(NamedColor::Blue7))
                .label("Report")
                .to(Route::Report)
            )
}

fn page() -> impl Element {
    El::new().child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Report => report::page().into_raw_element(),
        PageId::Home => El::new().child("Welcome Home!").into_raw_element(),
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
