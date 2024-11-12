use zoon::{*, println};
use std::sync::{Arc, RwLock};

mod engine;
use engine::*;

pub fn run(_program: &str) -> impl Element {
    let mut engine = Arc::new(RwLock::new(Engine::default()));

    let function_name = FunctionName::new("Element/stripe");
    let function_closure = |function_arguments: Arguments| {
        VariableKind::Object(VariableKindObject::new({
            let mut variables = Variables::new();
            
            let variable_name = VariableName::new("element");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(1.))
            );
            function_arguments
                .get(&ArgumentName::new("element"))
                .unwrap()
                .argument_out()
                .unwrap()
                .send_kind(variable.kind());
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("direction");
            let variable = Variable::new(
                variable_name.clone(),
                function_arguments
                    .get(&ArgumentName::new("direction"))
                    .unwrap()
                    .argument_in()
                    .unwrap()
                    .kind()
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("gap");
            let variable = Variable::new(
                variable_name.clone(),
                function_arguments
                    .get(&ArgumentName::new("gap"))
                    .unwrap()
                    .argument_in()
                    .unwrap()
                    .kind()
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
                    .kind()
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
                    .kind()
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("extra");
            let variable = Variable::new(
                variable_name.clone(),
                function_arguments
                    .get(&ArgumentName::new("extra"))
                    .unwrap()
                    .argument_in()
                    .unwrap()
                    .kind()
            );
            variables.insert(variable_name, variable);

            variables
        }))
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let function_name = FunctionName::new("root_element");
    let function_closure = {
        let engine = engine.clone();
        move |function_arguments: Arguments| {
            let mut arguments = Arguments::new();
            
            let argument_name = ArgumentName::new("element");
            let (argument, element_kind_receiver) = Argument::new_out(
                argument_name.clone(),
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("direction");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::Tag(VariableKindTag::new("Row")),
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("gap");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::Number(VariableKindNumber::new(15.)),
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("style");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::Object(VariableKindObject::new({
                    let mut variables = Variables::new();

                    let variable_name = VariableName::new("align");
                    let variable = Variable::new(
                        variable_name.clone(),
                        VariableKind::Tag(VariableKindTag::new("Center"))
                    );
                    variables.insert(variable_name, variable);

                    variables
                })),
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("items");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::List(VariableKindList::new({
                    let mut list = Vec::new();

                    list.push({
                        let mut arguments= Arguments::new();

                        let argument_name = ArgumentName::new("label");
                        let argument = Argument::new_in(
                            argument_name.clone(),
                            VariableKind::Text(VariableKindText::new("-")),
                        );
                        arguments.insert(argument_name, argument);

                        engine
                            .read()
                            .unwrap()
                            .functions
                            .get(&FunctionName::new("counter_button"))
                            .unwrap()
                            .run(arguments)
                    });

                    list.push({
                        engine
                            .read()
                            .unwrap()
                            .variables
                            .get(&VariableName::new("counter"))
                            .unwrap()
                            .kind()
                    });

                    list.push({
                        let mut arguments= Arguments::new();

                        let argument_name = ArgumentName::new("label");
                        let argument = Argument::new_in(
                            argument_name.clone(),
                            VariableKind::Text(VariableKindText::new("+")),
                        );
                        arguments.insert(argument_name, argument);

                        engine
                            .read()
                            .unwrap()
                            .functions
                            .get(&FunctionName::new("counter_button"))
                            .unwrap()
                            .run(arguments)
                    });

                    list
                })),
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("extra");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::Object(VariableKindObject::new(Variables::new())),
            );
            arguments.insert(argument_name, argument);

            engine.read().unwrap().functions.get(&FunctionName::new("Element/stripe")).unwrap().run(arguments)
        }
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let function_name: FunctionName = FunctionName::new("Element/button");
    let function_closure = |function_arguments: Arguments| {
        VariableKind::Object(VariableKindObject::new({
            let mut variables = Variables::new();
            
            let variable_name = VariableName::new("element");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(1.))
            );
            function_arguments
                .get(&ArgumentName::new("element"))
                .unwrap()
                .argument_out()
                .unwrap()
                .send_kind(variable.kind());
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("style");
            let variable = Variable::new(
                variable_name.clone(),
                function_arguments
                    .get(&ArgumentName::new("style"))
                    .unwrap()
                    .argument_in()
                    .unwrap()
                    .kind()
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
                    .kind()
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("extra");
            let variable = Variable::new(
                variable_name.clone(),
                function_arguments
                    .get(&ArgumentName::new("extra"))
                    .unwrap()
                    .argument_in()
                    .unwrap()
                    .kind()
            );
            variables.insert(variable_name, variable);

            variables
        }))
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let function_name: FunctionName = FunctionName::new("counter_button");
    let function_closure = { 
        let engine = engine.clone();
            move |function_arguments: Arguments| {
            let mut arguments = Arguments::new();
            
            let argument_name = ArgumentName::new("element");
            let (argument, element_kind_receiver) = Argument::new_out(
                argument_name.clone(),
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("style");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::Number(VariableKindNumber::new(2.))
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("label");
            let argument = Argument::new_in(
                argument_name.clone(),
                function_arguments
                    .get(&ArgumentName::new("label"))
                    .unwrap()
                    .argument_in()
                    .unwrap()
                    .kind()
            );
            arguments.insert(argument_name, argument);

            let argument_name = ArgumentName::new("extra");
            let argument = Argument::new_in(
                argument_name.clone(),
                VariableKind::Number(VariableKindNumber::new(4.))
            );
            arguments.insert(argument_name, argument);

            engine.read().unwrap().functions.get(&FunctionName::new("Element/button")).unwrap().run(arguments)
        }
    };
    let function = Function::new(function_name.clone(), function_closure);
    engine.write().unwrap().functions.insert(function_name, function);

    let variable_name = VariableName::new("elements");
    let variable = Variable::new(
        variable_name.clone(),
        VariableKind::Object(VariableKindObject::new({
            let mut variables = Variables::new();
            
            let variable_name = VariableName::new("decrement_button");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Link(VariableKindLink::new())
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("increment_button");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Link(VariableKindLink::new())
            );
            variables.insert(variable_name, variable);

            variables
        }))
    );
    engine.write().unwrap().variables.insert(variable_name, variable);

    let variable_name = VariableName::new("counter");
    let variable = Variable::new(
        variable_name.clone(),
        VariableKind::Number(VariableKindNumber::new(6.))
    );
    engine.write().unwrap().variables.insert(variable_name, variable);

    let variable_name = VariableName::new("document");
    let variable = Variable::new(
        variable_name.clone(),
        engine.read().unwrap().functions.get(&FunctionName::new("root_element")).unwrap().run(Arguments::new()),
    );
    engine.write().unwrap().variables.insert(variable_name, variable);

    println!("{engine:#?}");

    El::new().child("Boon root")
} 
