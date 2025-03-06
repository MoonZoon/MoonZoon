// @TODO remove
#![allow(unused_variables)]

use std::borrow::Cow;
use std::rc::Rc;
use zoon::{eprintln, println, *};

mod boon;
use boon::platform::browser::{bridge::object_with_document_to_element_signal, interpreter};

mod code_editor;
use code_editor::CodeEditor;

static SOURCE_CODE_STORAGE_KEY: &str = "boon-example-source-code";

static OLD_SOURCE_CODE_STORAGE_KEY: &str = "boon-example-old-source-code";
static OLD_SPAN_ID_PAIRS_STORAGE_KEY: &str = "boon-example-span-id-pairs";
static STATES_STORAGE_KEY: &str = "boon-example-states";

#[derive(Clone, Copy)]
struct ExampleData {
    filename: &'static str,
    source_code: &'static str,
}

macro_rules! make_example_data {
    ($name:literal) => {{
        ExampleData {
            filename: concat!($name, ".bn"),
            source_code: include_str!(concat!("examples/", $name, "/", $name, ".bn")),
        }
    }};
}

static EXAMPLE_DATAS: [ExampleData; 4] = [
    make_example_data!("minimal"),
    make_example_data!("hello_world"),
    make_example_data!("interval"),
    make_example_data!("counter"),
];

#[derive(Clone, Copy)]
struct RunCommand {
    filename: Option<&'static str>,
}

fn main() {
    start_app("app", Playground::new);
}

#[derive(Clone)]
struct Playground {
    source_code: Mutable<Rc<Cow<'static, str>>>,
    run_command: Mutable<Option<RunCommand>>,
    _store_source_code_task: Rc<TaskHandle>,
}

impl Playground {
    fn new() -> impl Element {
        let source_code =
            if let Some(Ok(source_code)) = local_storage().get(SOURCE_CODE_STORAGE_KEY) {
                Cow::Owned(source_code)
            } else {
                Cow::Borrowed(EXAMPLE_DATAS[0].source_code)
            };
        let source_code = Mutable::new(Rc::new(source_code));
        Self {
            _store_source_code_task: Rc::new(Task::start_droppable(
                source_code.signal_cloned().for_each_sync(|source_code| {
                    if let Err(error) =
                        local_storage().insert(SOURCE_CODE_STORAGE_KEY, &source_code)
                    {
                        eprintln!("Failed to store source code: {error:#?}");
                    }
                }),
            )),
            source_code,
            run_command: Mutable::new(None),
        }
        .root()
    }

    fn root(&self) -> impl Element + use<> {
        Column::new()
            .s(Width::fill())
            .s(Height::fill())
            .s(Font::new().color(color!("oklch(0.8 0 0)")))
            .s(Scrollbars::both())
            .item(
                Row::new()
                    .item(
                        Row::new().s(Gap::new().x(20)).multiline().items(
                            EXAMPLE_DATAS.map(|example_data| self.example_button(example_data)),
                        ),
                    )
                    .item(self.clear_saved_states_button()),
            )
            .item(self.run_button())
            .item(
                Row::new()
                    .s(Padding::new().top(5))
                    .s(Width::fill())
                    .s(Height::fill())
                    .s(Scrollbars::both())
                    .item(self.code_editor_panel())
                    .item(self.example_panel()),
            )
    }

    fn run_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Align::new().center_x())
            .s(Padding::all(5))
            .label(
                Paragraph::new()
                    .s(Font::new().color_signal(
                        hovered_signal
                            .map_bool(|| color!("MediumSpringGreen"), || color!("LimeGreen")),
                    ))
                    .content("Run (")
                    .content(
                        El::new()
                            .s(Font::new().weight(FontWeight::Bold))
                            .child("Shift + Enter"),
                    )
                    .content(" in editor)"),
            )
            .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
            .on_press({
                let run_command = self.run_command.clone();
                move || {
                    run_command.set(Some(RunCommand { filename: None }));
                }
            })
    }

    fn clear_saved_states_button(&self) -> impl Element {
        let (hovered, hovered_signal) = Mutable::new_and_signal(false);
        Button::new()
            .s(Padding::new().x(10).y(5))
            .s(Font::new()
                .color_signal(hovered_signal.map_bool(|| color!("Coral"), || color!("LightCoral"))))
            .label("Clear saved states")
            .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
            .on_press(|| {
                local_storage().remove(STATES_STORAGE_KEY);
                local_storage().remove(OLD_SOURCE_CODE_STORAGE_KEY);
                local_storage().remove(OLD_SPAN_ID_PAIRS_STORAGE_KEY);
            })
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
                    .on_key_down_event_with_options(
                        EventOptions::new().preventable().parents_first(),
                        {
                            let run_command = self.run_command.clone();
                            move |keyboard_event| {
                                let RawKeyboardEvent::KeyDown(raw_event) =
                                    &keyboard_event.raw_event;
                                if keyboard_event.key() == &Key::Enter && raw_event.shift_key() {
                                    keyboard_event.pass_to_parent(false);
                                    run_command.set(Some(RunCommand { filename: None }));
                                }
                            }
                        },
                    )
                    .content_signal(self.source_code.signal_cloned())
                    .on_change({
                        let source_code = self.source_code.clone();
                        move |content| source_code.set_neq(Rc::new(Cow::from(content)))
                    }),
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
                        Border::new().color(color!("#282c34")).width(4),
                    ))
                    .child_signal(self.run_command.signal().map_some({
                        let this = self.clone();
                        move |run_command| this.example_runner(run_command)
                    })),
            )
    }

    fn example_runner(&self, run_command: RunCommand) -> impl Element + use<> {
        println!("Command to run example received!");
        let filename = run_command.filename.unwrap_or("custom code");
        let source_code = self.source_code.lock_ref();
        let object_and_construct_context = interpreter::run(
            filename,
            &source_code,
            STATES_STORAGE_KEY,
            OLD_SOURCE_CODE_STORAGE_KEY,
            OLD_SPAN_ID_PAIRS_STORAGE_KEY,
        );
        drop(source_code);
        if let Some((object, construct_context)) = object_and_construct_context {
            El::new()
                .child_signal(object_with_document_to_element_signal(
                    object.clone(),
                    construct_context,
                ))
                .after_remove(move |_| drop(object))
                .unify()
        } else {
            El::new()
                .s(Font::new().color(color!("LightCoral")))
                .child("Failed to run the example. See errors in dev console.")
                .unify()
        }
    }

    fn example_button(&self, example_data: ExampleData) -> impl Element {
        Button::new()
            .s(Padding::new().x(10).y(5))
            .s(Font::new().line(FontLine::new().underline().offset(3)))
            .label(example_data.filename)
            .on_press({
                let source_code = self.source_code.clone();
                let run_command = self.run_command.clone();
                move || {
                    source_code.set_neq(Rc::new(Cow::from(example_data.source_code)));
                    run_command.set(Some(RunCommand {
                        filename: Some(example_data.filename),
                    }));
                }
            })
    }
}
