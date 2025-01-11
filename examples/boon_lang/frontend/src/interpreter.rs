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
                        VariableValue::Object(_object) => {

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
                            .get(&FunctionName::new("Element/container"))
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

                                    let argument_name = ArgumentName::new("style");
                                    let argument = Argument::new_in(
                                        argument_name.clone(),
                                        VariableActor::new(async { stream::once(async { VariableValue::Object(VariableValueObject::new(Variables::new()))})})
                                    );
                                    arguments.insert(argument_name, argument);

                                    let argument_name = ArgumentName::new("child");
                                    let argument = Argument::new_in(
                                        argument_name.clone(),
                                        VariableActor::new(async { stream::once(async { VariableValue::Number(VariableValueNumber::new(32.))})})
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
