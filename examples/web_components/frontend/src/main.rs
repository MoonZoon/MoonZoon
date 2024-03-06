use zoon::*;

mod components;
use components::{Search, Tile};

static SEARCH_TERM: Lazy<Mutable<String>> = Lazy::new(|| Mutable::new("Hello!".to_owned()));

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Width::exact(300))
        .s(Align::center())
        .s(RoundedCorners::all(10))
        .s(Clip::both())
        .item(
            Search::new()
                .placeholder("Write something")
                .value_signal(SEARCH_TERM.signal_cloned())
                .on_change(|term| SEARCH_TERM.set(term)),
        )
        .item(Tile::new().child_signal(SEARCH_TERM.signal_cloned()))
}
