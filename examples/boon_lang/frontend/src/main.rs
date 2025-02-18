use zoon::*;

mod boon;
use boon::platform::browser::{bridge::object_with_document_to_element_signal, interpreter};

mod code_editor;
use code_editor::{CodeEditorController, CodeEditor};

macro_rules! run_example {
    ($name:literal) => {{
        interpreter::run(
            concat!($name, ".bn"),
            include_str!(concat!("examples/", $name, "/", $name, ".bn")),
        )
    }};
}

macro_rules! example_button {
    ($self:ident, $name:literal) => {{
        $self.example_button(
            concat!($name, ".bn"),
            include_str!(concat!("examples/", $name, "/", $name, ".bn")),
        )
    }};
}

fn main() {
    start_app("app", Playground::new);
}

#[derive(Clone)]
struct Playground {
    source_code: Mutable<String>,
}

impl Playground {
    fn new() -> impl Element {
        let source_code = Mutable::new(String::from("document: Document/new(root: 123)"));
        Self { source_code }.root()
    }

    fn root(&self) -> impl Element {
        Column::new()
            .s(Width::fill())
            .s(Height::fill())
            .s(Background::new().color(color!("Black")))
            .s(Font::new().color(color!("oklch(0.8 0 0)")))
            .s(Scrollbars::both())
            .item(
                Row::new()
                    .s(Gap::new().x(20))
                    .multiline()
                    .items(self.example_buttons())
            )
            .item(
                Paragraph::new()
                    .s(Align::new().center_x())
                    .content("Run: ")
                    .content(
                        El::new()
                            .s(Font::new().weight(FontWeight::Bold))
                            .child("Shift + Enter")
                    )
                    .content(" in editor")
            )
            .item(
                Row::new()
                    .s(Padding::new().top(5))
                    .s(Width::fill())
                    .s(Height::fill())
                    .s(Scrollbars::both())
                    .item(self.code_editor_panel())
                    .item(self.example_panel())
            )
    }
    
    fn code_editor_panel(&self) -> impl Element {
        El::new()
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::all(5))
            .s(Scrollbars::both())
            .child(
                CodeEditor::new()
                    .s(RoundedCorners::all(10))
                    .s(Scrollbars::both())
                    .on_key_down_event_with_options(EventOptions::new().preventable().parents_first(), |keyboard_event| {
                        let RawKeyboardEvent::KeyDown(raw_event) = &keyboard_event.raw_event;
                        if keyboard_event.key() == &Key::Enter && raw_event.shift_key() {
                            keyboard_event.pass_to_parent(false);
                            // @TODO remove + run example
                            zoon::println!("SHIFT + ENTER!");
                        }
                    })
                    .content_signal(self.source_code.signal_cloned())
                    .on_change({
                        let source_code = self.source_code.clone();
                        move |content| {
                            source_code.set_neq(content)
                        }
                    })
            )
    }
    
    fn example_panel(&self) -> impl Element {
        El::new()
            .s(Align::new().top())
            .s(Width::fill())
            .s(Height::fill())
            .s(Padding::all(5))
            .child(
                El::new()
                    .s(RoundedCorners::all(10))
                    .s(Clip::both())
                    .s(Borders::all(
                        Border::new()
                        .color(color!("#282c34"))
                        .width(4)
                    ))
                    .child(self.boon_object_with_document())
            )
    }
    
    fn boon_object_with_document(&self) -> impl Element {
        let object = run_example!("hello_world");
    
        if let Some(object) = object {
            El::new()
                .child_signal(object_with_document_to_element_signal(object.clone()))
                .after_remove(move |_| drop(object))
                .unify()
        } else {
            El::new().child("Failed to run the example. See errors in dev console.").unify()
        }
    }
    
    fn example_buttons(&self) -> Vec<impl Element> {
        vec![
            example_button!(self, "hello_world"),
            example_button!(self, "interval"),
            example_button!(self, "counter"),
        ]
    }
    
    fn example_button(&self, label: &'static str, example_code: &'static str) -> impl Element {
        Button::new()
            .s(Padding::new().x(10).y(5))
            .s(Font::new().line(FontLine::new().underline().offset(3)))
            .label(label)
            .on_press(|| ())
    }
}
