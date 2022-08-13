use zoon::{println, *};

// @TODO gradually improve event handler APIs according to real-world examples
// because it's quite difficult to choose reasonable default values / behavior
// and terminology could be confusing (capture vs bubbles, propagation, prevent default, passive vs preventable, etc.)

fn root() -> impl Element {
    Column::new()
        .s(Padding::all(30))
        .s(Gap::both(20))
        .item("Open dev console, write something and press Shift + Tab")
        .item(text_field())
        .item(div_with_global_event())
}

fn text_field() -> impl Element {
    TextInput::new()
        .s(Padding::all(5))
        .focus(true)
        .label_hidden("text field")
        .on_key_down_event_with_options(EventOptions::new().preventable(), |event| {
            let RawKeyboardEvent::KeyDown(raw_event) = &event.raw_event;

            if raw_event.repeat() || not(raw_event.shift_key()) {
                return;
            }

            if let Key::Other(key) = event.key() {
                if key == "Tab" {
                    // @TODO `.prevent_default()` should be callable only on a preventable event.
                    raw_event.prevent_default();
                    println!("TextInput: Shift + Tab");
                    println!(
                        "TextInput value: {:?}",
                        raw_event
                            .dyn_target::<web_sys::HtmlInputElement>()
                            .unwrap_throw()
                            .value()
                    );
                }
            }
        })
}

// Register global listener (attach to `window`).
// The listener is removed together with the El who has registered it.
// @TODO New Zoon API?
// @TODO Can we assume there'll always be a root El?
fn div_with_global_event() -> impl Element {
    El::new().update_raw_el(|raw_el| {
        raw_el.global_event_handler(|event: events::KeyDown| {
            let key = event.key();
            println!("Global: {key:#?}");
        })
    })
}

fn main() {
    start_app("app", root);
}
