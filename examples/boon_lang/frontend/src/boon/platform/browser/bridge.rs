use std::sync::Arc;

use zoon::futures_util::{select, stream};
use zoon::{eprintln, *};

use super::engine::*;

pub fn object_with_document_to_element_signal(
    root_object: Arc<Object>,
    construct_context: ConstructContext,
) -> impl Signal<Item = Option<RawElOrText>> {
    let element_stream = root_object
        .expect_variable("document")
        .subscribe()
        .flat_map(|value| {
            value
                .expect_object()
                .expect_variable("root_element")
                .subscribe()
        })
        .map(move |value| value_to_element(value, construct_context.clone()));

    signal::from_stream(element_stream)
}

fn value_to_element(value: Value, construct_context: ConstructContext) -> RawElOrText {
    match value {
        Value::Text(text, _) => zoon::Text::new(text.text()).unify(),
        Value::Number(number, _) => zoon::Text::new(number.number()).unify(),
        Value::TaggedObject(tagged_object, _) => match tagged_object.tag() {
            "ElementContainer" => element_container(tagged_object, construct_context).unify(),
            "ElementStripe" => element_stripe(tagged_object, construct_context).unify(),
            "ElementButton" => element_button(tagged_object, construct_context).unify(),
            other => panic!("Element cannot be created from the tagged objectwith tag '{other}'"),
        },
        _ => panic!("Element cannot be created from the given Value type"),
    }
}

fn element_container(
    tagged_object: Arc<TaggedObject>,
    construct_context: ConstructContext,
) -> impl Element {
    let settings_variable = tagged_object.expect_variable("settings");

    let child_stream = settings_variable
        .subscribe()
        .flat_map(|value| value.expect_object().expect_variable("child").subscribe())
        .map(move |value| value_to_element(value, construct_context.clone()));

    El::new().child_signal(signal::from_stream(child_stream))
}

fn element_stripe(
    tagged_object: Arc<TaggedObject>,
    construct_context: ConstructContext,
) -> impl Element {
    let settings_variable = tagged_object.expect_variable("settings");

    let direction_stream = settings_variable
        .subscribe()
        .flat_map(|value| value.expect_object().expect_variable("direction").subscribe())
        .map(|direction| match direction.expect_tag().tag() {
            "Column" => Direction::Column,
            "Row" => Direction::Row,
            other => panic!("Invalid Stripe element direction value: Found: '{other}', Expected: 'Column' or 'Row'"),
        });

    let items_vec_diff_stream = settings_variable
        .subscribe()
        .flat_map(|value| value.expect_object().expect_variable("items").subscribe())
        .flat_map(|value| value.expect_list().subscribe())
        .map(list_change_to_vec_diff);

    Stripe::new()
        .direction_signal(signal::from_stream(direction_stream).map(Option::unwrap_or_default))
        .items_signal_vec(VecDiffStreamSignalVec(items_vec_diff_stream).map_signal(
            move |value_actor| {
                signal::from_stream(value_actor.subscribe().map({
                    let construct_context = construct_context.clone();
                    move |value| value_to_element(value, construct_context.clone())
                }))
            },
        ))
}

fn element_button(
    tagged_object: Arc<TaggedObject>,
    construct_context: ConstructContext,
) -> impl Element {
    type PressEvent = ();

    let (press_event_sender, mut press_event_receiver) = mpsc::unbounded::<PressEvent>();

    let event_stream =
        stream::iter(tagged_object.variable("event")).flat_map(|variable| variable.subscribe());

    let mut press_stream = event_stream
        .filter_map(|value| future::ready(value.expect_object().variable("press")))
        .map(|variable| variable.expect_link_value_sender())
        .fuse();

    let press_handler_task = Task::start_droppable({
        let construct_context = construct_context.clone();
        async move {
            let mut press_link_value_sender = None;
            let mut press_event_object_value_version = 0u64;
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
                            let press_event_object_value = Object::new_value(
                                ConstructInfo::new(format!("bridge::element_button::press_event, version: {press_event_object_value_version}"), None, "Button press event"),
                                construct_context.clone(),
                                ValueIdempotencyKey::new(),
                                [],
                            );
                            press_event_object_value_version += 1;
                            if let Err(error) = press_link_value_sender.unbounded_send(press_event_object_value) {
                                eprintln!("Failed to send button press event to event press link variable: {error}");
                            }
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
        .map(move |value| value_to_element(value, construct_context.clone()));

    Button::new()
        .label_signal(signal::from_stream(label_stream).map(|label| {
            if let Some(label) = label {
                label
            } else {
                zoon::Text::new("").unify()
            }
        }))
        // @TODO Handle press event only when it's defined in Boon code? Add `.on_press_signal` to Zoon?
        .on_press(move || {
            let press_event: PressEvent = ();
            if let Err(error) = press_event_sender.unbounded_send(press_event) {
                eprintln!("Failed to send button press event from on_press handler: {error}");
            }
        })
        .after_remove(move |_| drop(press_handler_task))
}

#[pin_project]
#[derive(Debug)]
#[must_use = "SignalVecs do nothing unless polled"]
struct VecDiffStreamSignalVec<A>(#[pin] A);

impl<A, T> SignalVec for VecDiffStreamSignalVec<A>
where
    A: Stream<Item = VecDiff<T>>,
{
    type Item = T;

    #[inline]
    fn poll_vec_change(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<VecDiff<Self::Item>>> {
        self.project().0.poll_next(cx)
    }
}

fn list_change_to_vec_diff(change: ListChange) -> VecDiff<Arc<ValueActor>> {
    match change {
        ListChange::Replace { items } => VecDiff::Replace { values: items },
        ListChange::InsertAt { index, item } => VecDiff::InsertAt { index, value: item },
        ListChange::UpdateAt { index, item } => VecDiff::UpdateAt { index, value: item },
        ListChange::RemoveAt { index } => VecDiff::RemoveAt { index },
        ListChange::Move {
            old_index,
            new_index,
        } => VecDiff::Move {
            old_index,
            new_index,
        },
        ListChange::Push { item } => VecDiff::Push { value: item },
        ListChange::Pop => VecDiff::Pop {},
        ListChange::Clear => VecDiff::Clear {},
    }
}
