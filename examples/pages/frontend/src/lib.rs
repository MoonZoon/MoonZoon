use zoon::*;

mod admin;

// ------ ------
//    Statics
// ------ ------

static USER_NAME: &str = "John Doe";

// ------ ------
//    Signals
// ------ ------

#[route]
#[derive(Copy, Clone)]
enum Route {
    #[route("admin", ..)]
    Admin(admin::Route),
    #[route()]
    Root,
}

fn route_signal() -> impl Signal<Item = Option<Route>> {
    url().map(Route::from_url)
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
        .item(Link::new().label("Home").to(Route::root()))
        .item(Link::new().label("Report").to(Route::admin().report().frequency(None)))
}

fn page() -> impl Element {
    El::new()
        .child_signal(route_signal().map(|route| match route {
                Some(Route::Admin(route)) => {
                    admin::page(route).into_raw_element()
                }
                Some(Route::Root) => {
                    El::new().child("welcome home!").into_raw_element()
                }
                None => {
                    El::new().child("404").into_raw_element()
                }
        }))
        
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
