use zoon::*;

mod text_editor;
use text_editor::TextEditor;

static CONTENTS: Lazy<Mutable<Option<String>>> = lazy::default();

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20).top(40))
        .s(Align::new().center_x())
        .s(Width::fill().max(600))
        .item(text_editor())
        .item(contents_display())
}

fn text_editor() -> impl Element {
    TextEditor::new().on_change(|json| {
        CONTENTS.set(Some(format!("{json:#}")));
    })
}

fn contents_display() -> impl Element {
    El::new().s(Scrollbars::both()).child(
        El::new()
            .s(Padding::all(10))
            .s(Font::new().family([FontFamily::Monospace]))
            .child_signal(CONTENTS.signal_cloned()),
    )
}
