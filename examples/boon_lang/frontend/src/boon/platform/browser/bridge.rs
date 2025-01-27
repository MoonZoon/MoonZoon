use std::sync::Arc;

use zoon::{*, eprintln};
use zoon::futures_util::{stream, select};

use super::engine::*;

pub fn root_object_to_element_signal(root_object: Arc<Object>) -> impl Signal<Item = Option<RawElOrText>> {
    let element_stream = root_object
        .expect_variable("document")
        .subscribe()
        .flat_map(|value| {
            value
                .expect_object()
                .expect_variable("root_element")
                .subscribe()
        })
        .map(value_to_element);

    signal::from_stream(element_stream)
}

fn value_to_element(value: Value) -> RawElOrText {
    match value {
        Value::Text(text) => {
            zoon::Text::new(text.text()).unify()
        }
        Value::Number(number) => {
            zoon::Text::new(number.number()).unify()
        }
        Value::TaggedObject(tagged_object) => {
            match tagged_object.tag() {
                "ElementContainer" => element_container(tagged_object).unify(),
                "ElementStripe" => element_stripe(tagged_object).unify(),
                "ElementButton" => element_button(tagged_object).unify(),
                other => panic!("Element cannot be created from the tagged objectwith tag '{other}'")
            }
        }
        _ => panic!("Element cannot be created from the given Value type")
    }
}

fn element_container(tagged_object: Arc<TaggedObject>) -> impl Element {
    let settings_stream = tagged_object.expect_variable("settings");

    let child_stream = settings_stream
        .subscribe()
        .flat_map(|value| value.expect_object().expect_variable("child").subscribe())
        .map(value_to_element);

    El::new()
        .child_signal(signal::from_stream(child_stream))
}

fn element_stripe(tagged_object: Arc<TaggedObject>) -> impl Element {
    let settings_stream = tagged_object.expect_variable("settings");

    let item_list_stream = settings_stream
        .subscribe()
        .flat_map(|value| value.expect_object().expect_variable("items").subscribe())
        .map(|value| 
            value
                .expect_list()
                .items()
                .iter()
                .map(|actor| signal::from_stream(actor.subscribe().map(value_to_element)))
                .collect::<Vec<_>>()
        );

    // @TODO Column -> Stripe + direction
    Column::new()
        .items_signal_vec(signal::from_stream(item_list_stream)
            .map(Option::unwrap_or_default)
            .to_signal_vec()
            .map_signal(|item| item)
        )
}

fn element_button(tagged_object: Arc<TaggedObject>) -> impl Element {
    let (press_event_sender, mut press_event_receiver) = mpsc::unbounded();

    let event_stream = stream::iter(tagged_object.variable("event"))
        .flat_map(|variable| variable.subscribe());

    let mut press_stream = event_stream
        .filter_map(|value| future::ready(value.expect_object().variable("press")))
        .map(|variable| variable.expect_link_value_sender())
        .fuse();

    let press_handler_task = Task::start_droppable(async move {
        let mut press_link_value_sender = None;
        loop {
            select! {
                new_press_link_value_sender = press_stream.next() => {
                    if let Some(new_press_link_value_sender) = new_press_link_value_sender {
                        press_link_value_sender = Some(new_press_link_value_sender);
                    } else {
                        break
                    }
                }
                press_event = press_event_receiver.select_next_some() => {
                    if let Some(press_link_value_sender) = press_link_value_sender.as_ref() {
                        if let Err(error) = press_link_value_sender.unbounded_send(press_event) {
                            eprintln!("Failed to send button press event to event press link variable: {error}");
                        }
                    }
                }
            }
        }
    });

    let settings_variable = tagged_object.expect_variable("settings");

    let label_stream = settings_variable
        .subscribe()
        .flat_map(|value| value.expect_object().expect_variable("label").subscribe())
        .map(value_to_element);

    Button::new()
        .label_signal(signal::from_stream(label_stream).map(|label| {
            if let Some(label) = label {
                label
            } else {
                zoon::Text::new("").unify()
            }
        }))
        .on_press(move || {
            let press_event = Object::new_value(
                // @TODO generate id
                ConstructInfo::new(123, "Button press event"), 
                []
            );
            if let Err(error) = press_event_sender.unbounded_send(press_event) {
                eprintln!("Failed to send button press event from on_press handler: {error}");
            }
        })
        .after_remove(move |_| drop(press_handler_task))
}
