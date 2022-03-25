use zoon::*;

fn root() -> impl Element {
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
}

// ---------- // -----------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
