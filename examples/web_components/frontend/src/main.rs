use zoon::*;

mod components;
use components::{Search, Tile};

#[static_ref]
fn search_term() -> &'static Mutable<String> {
    Mutable::new("Hello!".to_owned())
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
                .value_signal(search_term().signal_cloned())
                .on_change(|term| search_term().set(term)),
        )
        .item(Tile::new().child_signal(search_term().signal_cloned()))
}

fn main() {
    start_app("app", root);
}
