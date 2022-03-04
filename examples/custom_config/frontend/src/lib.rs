use shared::{DownMsg, UpMsg};
use zoon::{eprintln, *};

// ------ ------
//    States
// ------ ------

#[static_ref]
fn favorite_languages() -> &'static Mutable<String> {
    Mutable::new("Loading...".to_owned())
}

#[static_ref]
pub fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::FavoriteLanguages(languages), _| {
        favorite_languages().set_neq(languages)
    })
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Padding::all(10))
        .s(Spacing::new(60))
        .s(Font::new().size(20))
        .item(
            El::with_tag(Tag::H1)
                .s(Font::new().size(30).no_wrap().wrap_anywhere())
                .s(Font::new().size(30))
                .child("Variables loaded from MoonZoonCustom.toml"),
        )
        .item(
            Row::new()
                .s(Spacing::new(20))
                .multiline()
                .item(El::new().child("my_api / MY_API:"))
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
                .s(Spacing::new(20))
                .multiline()
                .item(El::new().child("favorite_languages / FAVORITE_LANGUAGES:"))
                .item(
                    El::new()
                        .s(Font::new().weight(FontWeight::Bold))
                        .child_signal(favorite_languages().signal_cloned()),
                )
                .item(
                    El::new()
                        .s(Font::new().italic())
                        .child("(loaded at runtime)"),
                ),
        )
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
    Task::start(async {
        if let Err(error) = connection().send_up_msg(UpMsg::GetFavoriteLanguages).await {
            eprintln!("send UpMsg failed: {error}");
        }
    });
}
