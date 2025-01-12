use zoon::{*, println};
use zoon::futures_util::stream;
use std::sync::{Arc, RwLock};

mod engine;
use engine::*;


// @TODO finish the manual counter code below

// @TODO generate the code automatically

// @TODO PASS + PASSED (decrement_button, increment_button, ..)


pub async fn run(_program: &str) -> impl Element {
    // @TODO get rid of the lock?
    let engine = Arc::new(RwLock::new(Engine::default()));

    // @TODO pass weak `engine` references instead of cloning?

    let function_name: FunctionName = FunctionName::new("Document/new");
    let function_closure = { 
        move |function_arguments: Arguments, _passed_argument: Option<PassedArgument>| { 
            async move {
                VariableActor::new(async move { stream::once(async move { VariableValue::Object(VariableValueObject::new({
                    let mut variables = Variables::new();
                    
                    let variable_name = VariableName::new("root_element");
                    let variable = Variable::new(
                        variable_name.clone(),
                        function_arguments
                            .get(&ArgumentName::new("root"))
                            .unwrap()
                            .argument_in()
                            .unwrap()
                            .actor()
                    );
                    variables.insert(variable_name, variable);

                    variables
                }))})})
            }
        }
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let function_name = FunctionName::new("Element/container");
    let function_closure = { 
        move |function_arguments: Arguments, _passed_argument: Option<PassedArgument>| { 
            async move {
                VariableActor::new(async move { stream::once(async move { VariableValue::TaggedObject(VariableValueTaggedObject::new("Element", {
                    let mut variables = Variables::new();

                    let element_variable_value = function_arguments
                        .get(&ArgumentName::new("element"))
                        .unwrap()
                        .argument_in()
                        .unwrap()
                        .actor()
                        .get_value()  // @TODO `value_changes()`?
                        .await; 
                    match element_variable_value {
                        VariableValue::Object(object) => {
                            variables.append(&mut object.into_variables());
                        }
                        _ => panic!("'element' has to be 'Object'")
                    }

                    let variable_name = VariableName::new("type");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableActor::new(async { stream::once(async { VariableValue::Tag(VariableValueTag::new("Container"))})})
                    );
                    variables.insert(variable_name, variable);
                    
                    let variable_name = VariableName::new("settings");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableActor::new(async move { stream::once(async move { VariableValue::Object(VariableValueObject::new({
                            let mut variables = Variables::new();

                            let variable_name = VariableName::new("style");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("style"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            let variable_name = VariableName::new("child");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("child"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            variables
                        }))})})
                    );
                    variables.insert(variable_name, variable);

                    variables
                }))})
            })}
        }
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let function_name = FunctionName::new("Element/button");
    let function_closure = { 
        move |function_arguments: Arguments, _passed_argument: Option<PassedArgument>| { 
            async move {
                VariableActor::new(async move { stream::once(async move { VariableValue::TaggedObject(VariableValueTaggedObject::new("Element", {
                    let mut variables = Variables::new();

                    let element_variable_value = function_arguments
                        .get(&ArgumentName::new("element"))
                        .unwrap()
                        .argument_in()
                        .unwrap()
                        .actor()
                        .get_value()  // @TODO `value_changes()`?
                        .await; 
                    match element_variable_value {
                        VariableValue::Object(object) => {
                            variables.append(&mut object.into_variables());
                        }
                        _ => panic!("'element' has to be 'Object'")
                    };

                    let variable_name = VariableName::new("type");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableActor::new(async { stream::once(async { VariableValue::Tag(VariableValueTag::new("Button"))})})
                    );
                    variables.insert(variable_name, variable);
                    
                    let variable_name = VariableName::new("settings");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableActor::new(async move { stream::once(async move { VariableValue::Object(VariableValueObject::new({
                            let mut variables = Variables::new();

                            let variable_name = VariableName::new("style");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("style"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            let variable_name = VariableName::new("label");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("label"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            variables
                        }))})})
                    );
                    variables.insert(variable_name, variable);

                    variables
                }))})
            })}
        }
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let function_name = FunctionName::new("Element/stripe");
    let function_closure = { 
        move |function_arguments: Arguments, _passed_argument: Option<PassedArgument>| { 
            async move {
                VariableActor::new(async move { stream::once(async move { VariableValue::TaggedObject(VariableValueTaggedObject::new("Element", {
                    let mut variables = Variables::new();

                    let element_variable_value = function_arguments
                        .get(&ArgumentName::new("element"))
                        .unwrap()
                        .argument_in()
                        .unwrap()
                        .actor()
                        .get_value()  // @TODO `value_changes()`?
                        .await; 
                    match element_variable_value {
                        VariableValue::Object(object) => {
                            variables.append(&mut object.into_variables());
                        }
                        _ => panic!("'element' has to be 'Object'")
                    }

                    let variable_name = VariableName::new("type");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableActor::new(async { stream::once(async { VariableValue::Tag(VariableValueTag::new("Stripe"))})})
                    );
                    variables.insert(variable_name, variable);
                    
                    let variable_name = VariableName::new("settings");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableActor::new(async move { stream::once(async move { VariableValue::Object(VariableValueObject::new({
                            let mut variables = Variables::new();

                            let variable_name = VariableName::new("direction");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("direction"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            let variable_name = VariableName::new("style");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("style"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            let variable_name = VariableName::new("items");
                            let variable = Variable::new(
                                variable_name.clone(),
                                function_arguments
                                    .get(&ArgumentName::new("items"))
                                    .unwrap()
                                    .argument_in()
                                    .unwrap()
                                    .actor()
                            );
                            variables.insert(variable_name, variable);

                            variables
                        }))})})
                    );
                    variables.insert(variable_name, variable);

                    variables
                }))})
            })}
        }
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let variable_name = VariableName::new("document");
    let variable = Variable::new(
        variable_name.clone(),
        engine
            .read()
            .unwrap()
            .functions
            .get(&FunctionName::new("Document/new"))
            .unwrap()
            .run(
                {
                    let mut arguments = Arguments::new();

                    let argument_name = ArgumentName::new("root");
                    let argument = Argument::new_in(
                        argument_name.clone(),
                        engine
                            .read()
                            .unwrap()
                            .functions
                            .get(&FunctionName::new("Element/stripe"))
                            .unwrap()
                            .run(
                                {
                                    let mut arguments = Arguments::new();

                                    let argument_name = ArgumentName::new("element");
                                    let argument = Argument::new_in(
                                        argument_name.clone(),
                                        VariableActor::new(async { stream::once(async { VariableValue::Object(VariableValueObject::new(Variables::new()))})})
                                    );
                                    arguments.insert(argument_name, argument);

                                    let argument_name = ArgumentName::new("direction");
                                    let argument = Argument::new_in(
                                        argument_name.clone(),
                                        VariableActor::new(async { stream::once(async { VariableValue::Tag(VariableValueTag::new("Row"))})})
                                    );
                                    arguments.insert(argument_name, argument);

                                    let argument_name = ArgumentName::new("style");
                                    let argument = Argument::new_in(
                                        argument_name.clone(),
                                        VariableActor::new(async { stream::once(async { VariableValue::Object(VariableValueObject::new(Variables::new()))})})
                                    );
                                    arguments.insert(argument_name, argument);

                                    let engine = engine.clone();
                                    let argument_name = ArgumentName::new("items");
                                    let argument = Argument::new_in(
                                        argument_name.clone(),
                                        VariableActor::new(async move { stream::once(async move { VariableValue::List(VariableValueList::new(vec![
                                            // @TODO remove
                                            // VariableActor::new(async { stream::once(async { VariableValue::Number(VariableValueNumber::new(255.)) })}),
                                            {
                                                let engine = engine.clone();
                                                VariableActor::new(async move { 
                                                    let increment_button = engine.read().unwrap().variables.get(&VariableName::new("increment_button")).unwrap().actor();
                                                    match increment_button.get_value().await {
                                                        VariableValue::TaggedObject(tagged_object) => {
                                                            assert_eq!(tagged_object.tag(), "Element");
                                                            match tagged_object.variable(&VariableName::new("event")).unwrap().actor().get_value().await {
                                                                VariableValue::Object(object) => {
                                                                    match object.variable(&VariableName::new("press")).unwrap().actor().get_value().await {
                                                                        VariableValue::Link(link) => {
                                                                            zoon::println!("XXX increment_button event press: {:#?}", link.async_debug_format().await); 
                                                                            let link_actor = link.link_actor();
                                                                            link_actor.value_changes().then({
                                                                                let link_actor = link_actor.clone();
                                                                                move |_change| {
                                                                                    let link_actor = link_actor.clone();
                                                                                    async move {
                                                                                        match link_actor.get_value().await {
                                                                                            VariableValue::Unset => {
                                                                                                zoon::println!("LINK ACTOR IS UNSET ------------------");
                                                                                                VariableValue::Number(VariableValueNumber::new(0.))
                                                                                            },
                                                                                            VariableValue::Object(_object) => {
                                                                                                zoon::println!("LINK ACTOR IS OBJECT ------------------");
                                                                                                VariableValue::Number(VariableValueNumber::new(12.))
                                                                                            }
                                                                                            _ => {
                                                                                                zoon::println!("LINK ACTOR IS SOMETHING ELSE ------------------");
                                                                                                VariableValue::Number(VariableValueNumber::new(12345.))
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            })
                                                                        }
                                                                        _ => panic!("increment_button event press has to be 'Link'")
                                                                    }
                                                                }
                                                                _ => panic!("increment_button event has to be 'Object'")
                                                            }
                                                        }
                                                        _ => panic!("increment_button has to be 'TaggedObject'")
                                                    }
                                                })
                                            },
                                            engine
                                                .read()
                                                .unwrap()
                                                .variables
                                                .get(&VariableName::new("increment_button"))
                                                .unwrap()
                                                .actor()
                                        ]))})})
                                    );
                                    arguments.insert(argument_name, argument);

                                    arguments
                                },
                                None
                            )
                            .await
                    );
                    arguments.insert(argument_name, argument);

                    arguments
                },
                None
            )
            .await,
    );
    engine.write().unwrap().variables.insert(variable_name, variable);

    let variable_name = VariableName::new("increment_button");
    let variable = Variable::new(
        variable_name.clone(),
        engine
                .read()
                .unwrap()
                .functions
                .get(&FunctionName::new("Element/button"))
                .unwrap()
                .run(
                    {
                        let mut arguments = Arguments::new();

                        let argument_name = ArgumentName::new("element");
                        let argument = Argument::new_in(
                            argument_name.clone(),
                            VariableActor::new(async { stream::once(async { VariableValue::Object(VariableValueObject::new({
                                let mut variables = Variables::new();

                                let variable_name = VariableName::new("event");
                                let variable = Variable::new(
                                    variable_name.clone(),
                                    VariableActor::new(async { stream::once(async { VariableValue::Object(VariableValueObject::new({
                                        let mut variables = Variables::new();
            
                                        let variable_name = VariableName::new("press");
                                        let variable = Variable::new(
                                            variable_name.clone(),
                                            VariableActor::new(async { stream::once(async { VariableValue::Link(VariableValueLink::new())})})
                                        );
                                        variables.insert(variable_name, variable);
            
                                        variables
                                    }))})})
                                );
                                variables.insert(variable_name, variable);

                                variables
                            }))})})
                        );
                        arguments.insert(argument_name, argument);

                        let argument_name = ArgumentName::new("style");
                        let argument = Argument::new_in(
                            argument_name.clone(),
                            VariableActor::new(async { stream::once(async { VariableValue::Object(VariableValueObject::new(Variables::new()))})})
                        );
                        arguments.insert(argument_name, argument);

                        let argument_name = ArgumentName::new("label");
                        let argument = Argument::new_in(
                            argument_name.clone(),
                            VariableActor::new(async { stream::once(async { VariableValue::Text(VariableValueText::new("+"))})}),

                        );
                        arguments.insert(argument_name, argument);

                        arguments
                    },
                    None
                )
                .await
    );
    engine.write().unwrap().variables.insert(variable_name, variable);


    Task::next_macro_tick().await;
    println!("{}", engine.read().unwrap().async_debug_format().await);

    engine_to_element(engine).await
} 

async fn engine_to_element(engine: Arc<RwLock<Engine>>) -> impl Element {
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
                                                    let item_to_send = VariableValue::Object(VariableValueObject::new({
                                                        let mut variables = Variables::new();
                                                        let variable_name = VariableName::new("dummy_button_event_press_event");
                                                        variables.insert(variable_name.clone(), Variable::new(variable_name, VariableActor::new(
                                                            async { stream::once( async { VariableValue::Unset }) }
                                                        )));
                                                        variables
                                                    }));
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
