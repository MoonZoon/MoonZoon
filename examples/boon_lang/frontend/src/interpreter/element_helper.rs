use parent::engine::*;

async fn root_actor_to_element(root_actor: ObjectActor) -> impl Element {
    let root_element_stream = root_actor.get_expected_variable_actor("document").actor_stream().map(|actor| {
        actor.expect_object_actor().get_expected_variable_actor("root_element")
    });

    let document_variable_actor = engine
        .read()
        .unwrap()
        .variables
        .get(&VariableName::new("document"))
        .unwrap()
        .actor();

    // @TODO get_value -> changes? (everywhere?)
    let root_element = match document_variable_actor.get_value().await {
        VariableValue::Object(object) => {
            object.variable(&VariableName::new("root_element")).unwrap().actor()
        }
        _ => panic!("'document' has to be 'Object'")
    };

    println!("{}", root_element.async_debug_format().await);
    println!("-----");

    actor_to_element(root_element).await
}

// @TODO `debug_assert_*` instead of `assert_*` everywhere?

async fn actor_to_element(actor: VariableActor) -> impl Element {
    match actor.get_value().await {
        VariableValue::TaggedObject(tagged_object) => {
            assert_eq!(tagged_object.tag(), "Element");
            match tagged_object.variable(&VariableName::new("type")).unwrap().actor().get_value().await {
                VariableValue::Tag(variable_value_tag) => {
                    match variable_value_tag.tag() {
                        "Container" => {
                            let settings = match tagged_object.variable(&VariableName::new("settings")).unwrap().actor().get_value().await {
                                VariableValue::Object(object) => object,
                                _ => panic!("Element settings has to be 'Object'")
                            };
                            let child = settings.variable(&VariableName::new("child")).unwrap().actor();
                            El::new().child_signal(actor_to_element(child).into_signal_option()).unify()
                        }
                        "Button" => {
                            let settings = match tagged_object.variable(&VariableName::new("settings")).unwrap().actor().get_value().await {
                                VariableValue::Object(object) => object,
                                _ => panic!("Element settings has to be 'Object'")
                            };
                            let label = settings.variable(&VariableName::new("label")).unwrap().actor();
                            let label = match label.get_value().await {
                                VariableValue::Text(label) => label.text().to_owned(),
                                _ => panic!("Button label has to be 'String'")
                            };
                            let button = Button::new().label(label);
                            let button = if let Some(event) = tagged_object.variable(&VariableName::new("event")) {
                                match event.actor().get_value().await {
                                    VariableValue::Object(object) => {
                                        if let Some(press) = object.variable(&VariableName::new("press")) {
                                            let (press_event_sender, press_event_receiver) = mpsc::unbounded();
                                            let target_actor = VariableActor::new(async move { 
                                                press_event_receiver
                                            });
                                            let button = button.on_press(move || { 
                                                let mut press_event_sender = press_event_sender.clone();
                                                Task::start(async move {
                                                    // let item_to_send = VariableValue::Object(VariableValueObject::new({
                                                    //     let mut variables = Variables::new();
                                                    //     let variable_name = VariableName::new("dummy_button_event_press_event");
                                                    //     variables.insert(variable_name.clone(), Variable::new(variable_name, VariableActor::new(
                                                    //         async { stream::once( async { VariableValue::Unset }) }
                                                    //     )));
                                                    //     variables
                                                    // }));
                                                    let item_to_send = VariableValue::Object(VariableValueObject::new(Variables::new()));
                                                    press_event_sender.send(item_to_send).await.unwrap();
                                                    println!("press event sent!!");
                                                });
                                            });
                                            match press.actor().get_value().await {
                                                VariableValue::Link(variable_value_link) => {
                                                    variable_value_link.set_target(target_actor.clone());
                                                }
                                                _ => panic!("Failed to set link value - the variable is not a Link")
                                            }
                                            button.after_remove(move |_| drop(target_actor))
                                        } else {
                                            button.on_press(||{})
                                        }
                                    },
                                    _ => panic!("Element event has to be 'Object'")
                                }
                            } else {
                                button.on_press(||{})
                            };
                            button.unify()
                        }
                        "Stripe" => {
                            let settings = match tagged_object.variable(&VariableName::new("settings")).unwrap().actor().get_value().await {
                                VariableValue::Object(object) => object,
                                _ => panic!("Element settings has to be 'Object'")
                            };
                            let direction = settings.variable(&VariableName::new("direction")).unwrap().actor();
                            let _direction = match direction.get_value().await {
                                VariableValue::Tag(direction) => {
                                    match direction.tag() {
                                        "Row" => {},
                                        "Column" => {},
                                        _ => panic!("Stripe direction has to be 'Row' or 'Column'")
                                    }
                                },
                                _ => panic!("Button direction has to be 'Tag'")
                            };
                            let items = settings.variable(&VariableName::new("items")).unwrap().actor();
                            let mut element_items = Vec::new();
                            match items.get_value().await {
                                VariableValue::List(items) => {
                                    for item in items.list() {
                                        element_items.push(actor_to_element(item.to_owned()).boxed_local().await);
                                    }
                                },
                                _ => panic!("Button direction has to be 'Tag'")
                            };
                            // @TODO Column -> Stripe
                            Column::new().items(element_items).unify()
                        }
                        other => panic!("Unknown element type: {other}")
                    }
                }
                _ => panic!("Element type has to be 'Tag'")
            }
        }
        VariableValue::Number(number) => {
            Text::new(number.number()).unify()
        }
        VariableValue::Text(text) => {
            Text::new(text.text()).unify()
        }
        _ => panic!("Element cannot be created from provided VariableActor")
    }
}
