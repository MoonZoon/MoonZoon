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

    engine_to_element(engine)
} 

fn engine_to_element(engine: Arc<RwLock<Engine>>) -> impl Element {
    El::new().child("Boon root")
}
