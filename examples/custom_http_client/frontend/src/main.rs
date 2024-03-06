use zoon::{eprintln, *};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Stars {
    #[default]
    Loading,
    Loaded(u32),
    Failed,
}

static STARS: Lazy<Mutable<Stars>> = lazy::default();

fn load_stars() {
    async fn stars_request() -> reqwest::Result<u32> {
        // @TODO remove `origin()` once a wasm-compatible HTTP client supports relative urls
        // Surf issue: https://github.com/http-rs/surf/issues/314
        // Reqwest issue: https://github.com/seanmonstar/reqwest/issues/988
        Ok(reqwest::get(routing::origin() + "/_api/moonzoon_stars")
            .await?
            .error_for_status()?
            .text()
            .await?
            .parse()
            .unwrap_throw())
    }
    Task::start(async {
        STARS.set_neq(Stars::Loading);
        match stars_request().await {
            Ok(loaded_stars) => STARS.set_neq(Stars::Loaded(loaded_stars)),
            Err(error) => {
                eprintln!("stars request failed: {error:#?}");
                STARS.set_neq(Stars::Failed)
            }
        }
    });
}

fn main() {
    start_app("app", root);
    load_stars();
}

fn root() -> impl Element {
    Row::new().item("MoonZoon stars: ").item(stars_text())
}

fn stars_text() -> impl Element {
    Text::with_signal(STARS.signal().map(|stars| match stars {
        Stars::Loading => "Loading...".into_cow_str(),
        Stars::Loaded(stars) => stars.into_cow_str(),
        Stars::Failed => "Loading failed!".into_cow_str(),
    }))
}
