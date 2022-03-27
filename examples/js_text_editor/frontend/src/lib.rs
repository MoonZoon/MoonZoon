use zoon::*;

mod text_editor;
use text_editor::TextEditor;

// ------ ------
//    States
// ------ ------

#[static_ref]
fn contents() -> &'static Mutable<Option<String>> {
    Mutable::default()
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Column::new()
        .item(text_editor())
        .item(contents_display())
}

fn text_editor() -> impl Element {
    let text_editor = Mutable::new(None);
    El::new()
        .after_insert(clone!((text_editor) move |html_element| {
            let editor = TextEditor::new(html_element).on_change(|json| {
                contents().set(json.map(|json| format!("{json:#}")));
            });
            text_editor.set(Some(editor));
        }))
        .after_remove(|_| drop(text_editor))
}

fn contents_display() -> impl Element {
    El::new()
        .s(Padding::all(10))
        .s(Font::new().family([FontFamily::Monospace]))
        .child_signal(contents().signal_cloned())
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
