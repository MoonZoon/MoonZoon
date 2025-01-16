use zoon::*;
use parent::engine::*;

fn root_object_value_to_element_signal(root: ObjectValue) -> impl Signal<Item = RawElOrText> {
    let element_stream = root
        .get_expected_variable("document")
        .value_stream()
        .flat_map(|value| {
            value
                .expect_object_value()
                .object_stream()
                .flat_map(|object| {
                    object
                        .get_expected_variable("root_element")
                        .value_stream()
                        .flat_map(|value| value_to_element_stream(value))
                })
        });

    signal::from_stream(element_stream)
}

fn object_to_element_stripe(object: Object) -> impl Element {
    object
        .get_expected_variable("settings")
        .value_stream()
        .flat_map(|value| value.expect_object_value().object_stream())
        .map(|object| {
            let _direction_tag_stream = 
                object
                    .get_expected_variable("direction")
                    .value_stream()
                    .flat_map(|value| value.expect_tag_value().tag_stream());

            let _style_object_stream = 
                object
                    .get_expected_variable("style")
                    .value_stream()
                    .flat_map(|value| value.expect_object_value().object_stream());

            let items_vec_diff_stream = 
                object
                    .get_expected_variable("items")
                    .value_stream()
                    .flat_map(|value| value.expect_list_value().list_stream())
                    .flat_map(|list| list.change_stream())
                    .map(list_change_to_vec_diff);
                    
            let mut items_mutable_vec = MutableVec::new();

            let items_mutable_vec_setter = Task::start_droppable(
                items_vec_diff_stream.for_each({
                    let items_mutable_vec = items_mutable_vec.clone();
                    move |vec_diff| {
                        MutableVecLockMut::apply_vec_diff(&mut items_mutable_vec.lock_mut(), vec_diff); 
                        future::ready(())
                    }
                })
            );

            let items_signal_vec = items_mutable_vec
                .signal_vec_cloned()
                .map_signal(|value| signal::from_stream(value_to_element_stream(value)));

            // @TODO Stripe::new(direction)
            Column::new()
                .items_signal_vec(items_signal_vec)
                .after_remove(move |_| drop(items_mutable_vec_setter))
        })
}

fn object_to_element_button(object: Object) -> impl Element {
    // "Button" => {
    //     let settings = match tagged_object.variable(&VariableName::new("settings")).unwrap().actor().get_value().await {
    //         VariableValue::Object(object) => object,
    //         _ => panic!("Element settings has to be 'Object'")
    //     };
    //     let label = settings.variable(&VariableName::new("label")).unwrap().actor();
    //     let label = match label.get_value().await {
    //         VariableValue::Text(label) => label.text().to_owned(),
    //         _ => panic!("Button label has to be 'String'")
    //     };
    //     let button = Button::new().label(label);
    //     let button = if let Some(event) = tagged_object.variable(&VariableName::new("event")) {
    //         match event.actor().get_value().await {
    //             VariableValue::Object(object) => {
    //                 if let Some(press) = object.variable(&VariableName::new("press")) {
    //                     let (press_event_sender, press_event_receiver) = mpsc::unbounded();
    //                     let target_actor = VariableActor::new(async move { 
    //                         press_event_receiver
    //                     });
    //                     let button = button.on_press(move || { 
    //                         let mut press_event_sender = press_event_sender.clone();
    //                         Task::start(async move {
    //                             // let item_to_send = VariableValue::Object(VariableValueObject::new({
    //                             //     let mut variables = Variables::new();
    //                             //     let variable_name = VariableName::new("dummy_button_event_press_event");
    //                             //     variables.insert(variable_name.clone(), Variable::new(variable_name, VariableActor::new(
    //                             //         async { stream::once( async { VariableValue::Unset }) }
    //                             //     )));
    //                             //     variables
    //                             // }));
    //                             let item_to_send = VariableValue::Object(VariableValueObject::new(Variables::new()));
    //                             press_event_sender.send(item_to_send).await.unwrap();
    //                             println!("press event sent!!");
    //                         });
    //                     });
    //                     match press.actor().get_value().await {
    //                         VariableValue::Link(variable_value_link) => {
    //                             variable_value_link.set_target(target_actor.clone());
    //                         }
    //                         _ => panic!("Failed to set link value - the variable is not a Link")
    //                     }
    //                     button.after_remove(move |_| drop(target_actor))
    //                 } else {
    //                     button.on_press(||{})
    //                 }
    //             },
    //             _ => panic!("Element event has to be 'Object'")
    //         }
    //     } else {
    //         button.on_press(||{})
    //     };
    //     button.unify()
    // }
}

fn value_to_element_stream(value: Value) -> impl Stream<Item = RawElOrText> {
    match value {
        Value::TaggedObjectValue(tagged_object_value) => {
            tagged_object_value
                .tagged_object_stream()
                .flat_map(|tag, object| {
                    assert_eq!(tag, "Element", "Element cannot be created from Object with tag '{tag}'");
                    object
                        .get_expected_variable("type")
                        .value_stream()
                        .flat_map(|value| value.expect_tag_value().tag_stream())
                        .map({
                            let object = object.clone();
                            move |element_type| {
                                match element_type {
                                    "Stripe" => object_to_element_stripe(object).unify(),
                                    "Button" => object_to_element_button(object).unify(),
                                    other => unreachable!("Element type '{other}' is not supported")
                                }
                            }
                        })

                })
                .boxed()
        }
        Value::NumberValue(number_value) => {
            number_value.number_stream().map(|number| Text::new(number).unify()).boxed()
        }
        Value::TextValue(text_value) => {
            text_value.text_stream().map(|text| Text::new(text).unify()).boxed()
        }
        unsupported_type => unreachable!("Element cannot be created from provided Value")
    }
}

fn list_change_to_vec_diff<T>(change: ListChange<T>) -> VecDiff<T> {
    match change {
        ListChange::Replace {
            values,
        } => {
            VecDiff::Replace {
                values,
            }
        },
        ListChange::InsertAt {
            index,
            value,
        } => {
            VecDiff::InsertAt {
                index,
                value,
            }
        },
        ListChange::UpdateAt {
            index,
            value,
        } => {
            VecDiff::UpdateAt {
                index,
                value,
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
            value,
        } => {
            VecDiff::Push {
                value,
            }
        },
        ListChange::Pop {} => {
            VecDiff::Pop {} 
        },
        ListChange::Clear {} => {
            VecDiff::Clear {}
        },
    }
}
