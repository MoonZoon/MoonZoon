use shared::{DownMsg, UpMsg};
use zoon::{eprintln, named_color::*, *};

// ------ ------
//    States
// ------ ------


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "serde")]
pub struct ItemData {
    name: String,
    item_type: String
}

#[static_ref]
fn items_list() -> &'static MutableVec<ItemData> {
    MutableVec::new()
}

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

fn create_item_list(){
    let mut data_list : Vec<ItemData> = Vec::new();
    for i in 0..10 {
        eprintln!("{i}");
        data_list.push(ItemData{ name: format!("user_{i}"), item_type: format!("data_{i}") })

    }
    items_list().lock_mut().replace_cloned(data_list)
}
// ------ ------
//     View
// ------ ------


fn item_list_display() -> impl Element {
    El::new().s(Align::center()).child(
        Row::new()
            .s(Align::center())
            .s(Spacing::new(10))
            .multiline()
            // .update_raw_el(|el| el.style("justify-content", "center"))
            .items_signal_vec(
                items_list()
                    .signal_vec_cloned()
                    .map(item_data_display),
            ),
    )
}

fn item_name(name: &String) -> impl Element {
    El::new()
        .s(Align::new().center_x())

        .s(Font::new().size(25).weight(FontWeight::SemiBold).color(GREEN_2))
        .child(name)
}

fn item_data_display(item_data: ItemData) -> impl Element {

    Column::new()
        .s(Padding::all(20))
        .s(RoundedCorners::all(10))
        .s(Background::new().color(BLUE_3))
        .s(Padding::new().y(10))
        .s(Shadows::new([Shadow::new().y(6).blur(16)]))
        .item(Row::new()
            .s(Spacing::new(10))
            .item(item_name(&item_data.name))
            .item(item_name(&item_data.item_type))
        )

}


fn root() -> impl Element {
    create_item_list();
    Column::new()
        .s(Align::center())
        .s(Padding::all(10))
        .s(Spacing::new(60))
        .s(Font::new().size(20))
        .item(
            El::with_tag(Tag::H1)
                .s(Font::new().size(30).wrap_anywhere())
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
        ).item( item_list_display())
}

// ------ ------
//     Start
// ------ ------

fn main() {
    start_app("app", root);
    Task::start(async {
        if let Err(error) = connection().send_up_msg(UpMsg::GetFavoriteLanguages).await {
            eprintln!("send UpMsg failed: {error}");
        }
    });
}
