use zoon::*;

fn root() -> impl Element {
    let (content, content_signal) = Mutable::new_and_signal_cloned("".to_string());
    let press = move |c: &str| *content.lock_mut() = c.to_string();

    Column::new()
        .item(
            El::new()
                .update_raw_el(|el| el.attr("class", "standalone-container"))
                .child(
                    El::new().update_raw_el(|el| el.attr("id", "snow-container")),
                ),
        )
        .item(
            El::with_tag(Tag::Custom("script"))
                .update_raw_el(|el| el.style("display", "none"))
                .child(
                    r#"
  var quill = new Quill('#snow-container', {
    placeholder: 'Compose an epic...',
    theme: 'snow'
  });
"#,
                ),
        )
        .item(
            Button::new()
                .label("Display content")
                .on_press(move || press(&*get_content_from_quill())),
        )
        .item(Paragraph::new().content_signal(content_signal))
}

#[wasm_bindgen(
    inline_js = "export function get_content() { return JSON.stringify(quill.getContents()); }"
)]
extern "C" {
    fn get_content() -> String;
}

fn get_content_from_quill() -> String {
    get_content()
}
// ---------- // -----------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
