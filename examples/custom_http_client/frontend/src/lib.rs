use zoon::{eprintln, routing::origin, *};

// ------ ------
//     Types
// ------ ------

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stars {
    Loading,
    Loaded(u32),
    Failed,
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn stars() -> &'static Mutable<Stars> {
    Mutable::new(Stars::Loading)
}

// ------ ------
//   Commands
// ------ ------

fn load_stars() {
    async fn stars_request() -> reqwest::Result<u32> {
        // @TODO remove `origin()` once a wasm-compatible HTTP client supports relative urls
        // Surf issue: https://github.com/http-rs/surf/issues/314
        // Reqwest issue: https://github.com/seanmonstar/reqwest/issues/988
        Ok(reqwest::get(origin() + "/_api/moonzoon_stars")
            .await?
            .error_for_status()?
            .text()
            .await?
            .parse()
            .unwrap_throw())
    }
    Task::start(async {
        stars().set_neq(Stars::Loading);
        match stars_request().await {
            Ok(loaded_stars) => stars().set_neq(Stars::Loaded(loaded_stars)),
            Err(error) => {
                eprintln!("stars request failed: {:#?}", error);
                stars().set_neq(Stars::Failed)
            }
        }
    });
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Row::new().item("MoonZoon stars: ").item(stars_text())
}

fn stars_text() -> impl Element {
    Text::with_signal(stars().signal().map(|stars| match stars {
        Stars::Loading => "Loading...".into_cow_str(),
        Stars::Loaded(stars) => stars.into_cow_str(),
        Stars::Failed => "Loading failed!".into_cow_str(),
    }))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
    load_stars();
}
