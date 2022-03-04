use zoon::*;

#[static_ref]
fn favorite_languages() -> &'static Mutable<String> {
    Mutable::new("Loading...".to_owned())
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Spacing::new(60))
        .s(Font::new().size(20))
        .item(
            El::with_tag(Tag::H1)
                .s(Font::new().size(30))
                .child("Variables loaded from MoonZoonCustom.toml")
        )
        .item(
            Row::new()
                .s(Spacing::new(20))
                .multiline()
                .item(El::new().s(Font::new().no_wrap()).child("my_api / MY_API:"))
                .item(El::new().s(Font::new().weight(FontWeight::Bold)).child(env!("MY_API")))
                .item(El::new().s(Font::new().italic()).child("(included at compile time)"))
        )
        .item(
            Row::new()
                .s(Spacing::new(20))
                .multiline()
                .item(El::new().s(Font::new().no_wrap()).child("favorite_languages / FAVORITE_LANGUAGES:"))
                .item(El::new().s(Font::new().weight(FontWeight::Bold)).child_signal(favorite_languages().signal_cloned()))
                .item(El::new().s(Font::new().italic()).child("(loaded at runtime)"))
        )
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
