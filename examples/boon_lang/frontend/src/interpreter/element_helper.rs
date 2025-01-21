use zoon::{*, println, eprintln};
use zoon::futures_util::select;
use super::engine::*;

pub fn root_object_stream_to_element_signal(root_object_stream: impl Stream<Item = Object>) -> impl Signal<Item = Option<RawElOrText>> {
    println!("CCCCCCC");

    let element_stream = root_object_stream
        .flat_map(|mut object| {
            println!("EEEEEE");
            let document_variable = object.take_expected_variable("document");
            std::mem::forget(object);
            println!("KKKKK");
            let stream = document_variable.subscribe().map(|object_value| {
                println!("JJJJJJJJ");
                object_value
            });
            std::mem::forget(document_variable);
            println!("LLLLL");
            stream
        })
        .flat_map(|value| {
            println!("IIIII");
            value
                .expect_object_value()
                .flat_map(|mut object| {
                    object
                        .take_expected_variable("root_element")
                        .flat_map(|value| value_to_element_stream(value))
                })
        });

    signal::from_stream(element_stream)
}

fn object_to_element_stripe(mut object: Object) -> impl Element {
    println!("BBBBBB");

    let (direction_tag_sender, _direction_tag_receiver) = mpsc::unbounded();
    let (style_object_sender, _style_object_receiver) = mpsc::unbounded();
    let (items_vec_diff_sender, items_vec_diff_receiver) = mpsc::unbounded();

    let settings_reader_task = Task::start_droppable(
        object
            .take_expected_variable("settings")
            .flat_map(|value| value.expect_object_value())
            .fold(vec![], move |_tasks, mut object| {
                future::ready(vec![
                    Task::start_droppable(
                        object
                            .take_expected_variable("direction")
                            .flat_map(|value| value.expect_tag_value())
                            .for_each({
                                let direction_tag_sender = direction_tag_sender.clone();
                                move |direction_tag| {
                                    let direction_tag_sender = direction_tag_sender.clone();
                                    async move {
                                        if let Err(error) = direction_tag_sender.unbounded_send(direction_tag) {
                                            eprintln!("Failed to send 'direction_tag' through 'direction_tag_sender': {error:#}")
                                        };
                                    }
                                }
                            })
                    ),
                    Task::start_droppable(
                        object
                            .take_expected_variable("style")
                            .flat_map(|value| value.expect_object_value())
                            .for_each({
                                let style_object_sender = style_object_sender.clone();
                                move |style_object| {
                                    let style_object_sender = style_object_sender.clone();
                                    async move {
                                        if let Err(error) = style_object_sender.unbounded_send(style_object) {
                                            eprintln!("Failed to send 'style_object' through 'style_object_sender': {error:#}")
                                        };
                                    }
                                }
                            })
                    ),
                    Task::start_droppable(
                        object
                            .take_expected_variable("items")
                            .flat_map(|value| value.expect_list_value().flatten())
                            .map(list_change_to_vec_diff)
                            .for_each({
                                let items_vec_diff_sender = items_vec_diff_sender.clone();
                                move |items_vec_diff| {
                                    let items_vec_diff_sender = items_vec_diff_sender.clone();
                                    async move {
                                        if let Err(error) = items_vec_diff_sender.unbounded_send(items_vec_diff) {
                                            eprintln!("Failed to send 'items_vec_diff' through 'items_vec_diff_sender': {error:#}")
                                        };
                                    }
                                }
                            })
                    )
                ])
            })
            .map(|_|())
    );

    println!("AAAAAAAAAAAAAAA");

    // @TODO Stripe::new(direction)
    Column::new()
        .items_signal_vec(
            VecDiffStreamSignalVec(items_vec_diff_receiver).map_signal(|value_stream| { 
                signal::from_stream(value_stream.flat_map(value_to_element_stream))
            })
        )
        .after_remove(move |_| { 
            drop(settings_reader_task);
        })
}

#[pin_project]
#[derive(Debug)]
#[must_use = "SignalVecs do nothing unless polled"]
struct VecDiffStreamSignalVec<A>(#[pin] A);

impl<A, T> SignalVec for VecDiffStreamSignalVec<A> where A: Stream<Item = VecDiff<T>> {
    type Item = T;

    #[inline]
    fn poll_vec_change(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context) -> std::task::Poll<Option<VecDiff<Self::Item>>> {
        self.project().0.poll_next(cx)
    }
}

fn object_to_element_button(mut object: Object) -> impl Element {
    let (style_object_sender, _style_object_receiver) = mpsc::unbounded();
    let (label_text_sender, label_text_receiver) = mpsc::unbounded();

    let settings_reader_task = Task::start_droppable(
        object
            .take_expected_variable("settings")
            
            .flat_map(|value| value.expect_object_value())
            .fold(vec![], move |_tasks, mut object| {
                future::ready(vec![
                    Task::start_droppable(
                        object
                            .take_expected_variable("style")
                            .flat_map(|value| value.expect_object_value())
                            .for_each({
                                let style_object_sender = style_object_sender.clone();
                                move |style_object| { 
                                    let style_object_sender = style_object_sender.clone();
                                    async move {
                                        if let Err(error) = style_object_sender.unbounded_send(style_object) {
                                            eprintln!("Failed to send 'style_object' through 'style_object_sender': {error:#}")
                                        };
                                    }
                                }
                            })
                    ),
                    Task::start_droppable(
                        object
                            .take_expected_variable("label")
                            .flat_map(|value| value.expect_text_value())
                            .for_each({
                                let label_text_sender = label_text_sender.clone();
                                move |label| { 
                                    let label_text_sender = label_text_sender.clone();
                                    async move {
                                        if let Err(error) = label_text_sender.unbounded_send(label) {
                                            eprintln!("Failed to send 'label' through 'label_text_sender: {error:#}'")
                                        };
                                    }
                                }
                            })
                    )
                ])
            })
            .map(|_|())
    );

    let (event_press_link_value_sender, mut event_press_link_value_receiver) = mpsc::unbounded();

    let event_reader_task = Task::start_droppable(
        object
            .take_expected_variable("event")
            .flat_map(|value| value.expect_object_value())
            .fold(vec![], move |_tasks, mut object| {
                future::ready(vec![
                    Task::start_droppable({
                        if let Some(variable) = object.take_variable("press") {
                            variable
                                .map(|value| value.expect_link_value())
                                .for_each({
                                    let event_press_link_value_sender = event_press_link_value_sender.clone();
                                    move |event_press_link_value| { 
                                        let event_press_link_value_sender = event_press_link_value_sender.clone();
                                        async move {
                                            if let Err(error) = event_press_link_value_sender.unbounded_send(event_press_link_value) {
                                                eprintln!("Failed to send 'event_press_link_value' through 'event_press_link_value_sender': {error:#}")
                                            };
                                        }
                                    }
                                }).boxed()
                        } else {
                             future::ready(()).boxed()
                        }
                    }),
                ])
            })
            .map(|_|())
    );

    let (event_press_data_object_sender, mut event_press_data_object_receiver) = mpsc::unbounded();

    let event_press_data_to_link_value_task = Task::start_droppable(async move {
        let mut _previous_link_value = None;
        let mut internal_event_press_data_object_sender = None::<mpsc::UnboundedSender<Object>>;
        loop {
            let result = select! {
                link_value = event_press_link_value_receiver.next() => Either::Left(link_value),
                data_object = event_press_data_object_receiver.next() => Either::Right(data_object),
            };
            match result {
                Either::Left(link_value) => {
                    if let Some(link_value) = link_value.as_ref() {
                        let (new_internal_event_press_data_object_sender, internal_event_press_data_object_receiver) = mpsc::unbounded();
                        let event_press_object_value = ObjectValue::new(
                            "press event", 
                            999, 
                            internal_event_press_data_object_receiver
                        );
                        internal_event_press_data_object_sender = Some(new_internal_event_press_data_object_sender);
                        link_value.send_new_value_stream(stream_one(event_press_object_value));
                    } 
                    _previous_link_value = link_value;
                }
                Either::Right(data_object) => {
                    if let Some(data_object) = data_object {
                        if let Some(internal_event_press_data_object_sender) = internal_event_press_data_object_sender.as_mut() {
                            if let Err(error) = internal_event_press_data_object_sender.send(data_object).await {
                                eprintln!("Failed to send 'data_object' through 'internal_event_press_data_object_sender': {error:#}")
                            }
                        }
                    }
                }
            }
        }
    });

    Button::new()
        .label_signal(signal::from_stream(label_text_receiver).map(Option::unwrap_or_default))
        .on_press(move || { 
            let event_data = Object::new([]);
            if let Err(error) = event_press_data_object_sender.unbounded_send(event_data) {
                eprintln!("Failed to send 'event_data' through 'event_press_data_object_sender': {error:#}")
            }
        })
        .after_remove(move |_| { 
            drop(settings_reader_task);
            drop(event_reader_task);
            drop(event_press_data_to_link_value_task);
        })
}

fn value_to_element_stream(value: Value) -> impl Stream<Item = RawElOrText> {
    println!("DDDDDDDDDDDD");
    match value {
        Value::TaggedObjectValue(tagged_object_value) => {
            tagged_object_value
                .then(|(tag, mut object)| async move {
                    assert_eq!(tag, "Element", "Element cannot be created from Object with tag '{tag}'");
                    let mut type_variable = object.take_expected_variable("type");
                    let type_tag = type_variable
                        .next()
                        .await
                        .expect("failed to get Element type")
                        .expect_tag_value()
                        .next()
                        .await
                        .expect("failed to get Element type tag");
                    match type_tag.as_str() {
                        "Stripe" => object_to_element_stripe(object).unify(),
                        "Button" => object_to_element_button(object).unify(),
                        other => unreachable!("Element type '{other}' is not supported")
                    }

                })
                .boxed()
        }
        Value::NumberValue(number_value) => {
            number_value.map(|number| Text::new(number).unify()).boxed()
        }
        Value::TextValue(text_value) => {
            text_value.map(|text| Text::new(text).unify()).boxed()
        }
        _ => unreachable!("Element cannot be created from the given Value")
    }
}

fn list_change_to_vec_diff(change: ListChange) -> VecDiff<CloneableStream<Value>> {
    match change {
        ListChange::Replace {
            items,
        } => {
            VecDiff::Replace {
                values: items,
            }
        },
        ListChange::InsertAt {
            index,
            item,
        } => {
            VecDiff::InsertAt {
                index,
                value: item,
            }
        },
        ListChange::UpdateAt {
            index,
            item,
        } => {
            VecDiff::UpdateAt {
                index,
                value: item,
            }
        },
        ListChange::RemoveAt {
            index,
        } => {
            VecDiff::RemoveAt {
                index,
            }
        },
        ListChange::Move {
            old_index,
            new_index,
        } => {
            VecDiff::Move {
                old_index,
                new_index,
            }
        },
        ListChange::Push {
            item,
        } => {
            VecDiff::Push {
                value: item,
            }
        },
        ListChange::Pop => {
            VecDiff::Pop {} 
        },
        ListChange::Clear => {
            VecDiff::Clear {}
        },
    }
}
