use shared::{DownMsg, UpMsg};
use zoon::{eprintln, *};

static FAVORITE_LANGUAGE: Lazy<Mutable<String>> =
    Lazy::new(|| Mutable::new("Loading...".to_owned()));

static CONNECTION: Lazy<Connection<UpMsg, DownMsg>> = Lazy::new(|| {
    Connection::new(|DownMsg::FavoriteLanguages(languages), _| FAVORITE_LANGUAGE.set_neq(languages))
});

fn main() {
    start_app("app", root);
    Task::start(async {
        if let Err(error) = CONNECTION.send_up_msg(UpMsg::GetFavoriteLanguages).await {
            eprintln!("send UpMsg failed: {error}");
        }
    });
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Padding::all(10))
        .s(Gap::both(60))
        .s(Font::new().size(20))
        .item(
            El::with_tag(Tag::H1)
                .s(Font::new().size(30).wrap_anywhere())
                .child("Variables loaded from MoonZoonCustom.toml"),
        )
        .item(
            Row::new()
                .s(Gap::both(20))
                .multiline()
                .item(El::new().child("MY_API:"))
                .item(
                    El::new()
                        .s(Font::new().weight(FontWeight::Bold))
                        .child(env!("MY_API")),
                )
                .item(
                    El::new()
                        .s(Font::new().italic())
                        .child("(included at compile time)"),
                ),
        )
        .item(
            Row::new()
                .s(Gap::both(20))
                .multiline()
                .item(El::new().child("FAVORITE_LANGUAGES:"))
                .item(
                    El::new()
                        .s(Font::new().weight(FontWeight::Bold))
                        .child_signal(FAVORITE_LANGUAGE.signal_cloned()),
                )
                .item(
                    El::new()
                        .s(Font::new().italic())
                        .child("(loaded at runtime)"),
                ),
        )
}
