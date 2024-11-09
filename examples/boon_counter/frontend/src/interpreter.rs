use zoon::{*, println};

mod engine;
use engine::*;

pub fn run(_program: &str) -> impl Element {
    let mut engine = Engine::default();

    let function_name = FunctionName::new("root_element");
    let function_body = || {
        VariableKind::Object(VariableKindObject::new({
            let mut variables = Variables::new();
            
            let variable_name = VariableName::new("element");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(1.))
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("direction");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(2.))
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("gap");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(3.))
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("style");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(4.))
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("items");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(5.))
            );
            variables.insert(variable_name, variable);

            let variable_name = VariableName::new("extra");
            let variable = Variable::new(
                variable_name.clone(),
                VariableKind::Number(VariableKindNumber::new(6.))
            );
            variables.insert(variable_name, variable);

            variables
        }))
    };
    let function = Function::new(function_name.clone(), function_body);
    engine.functions.insert(function_name, function);

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
    engine.variables.insert(variable_name, variable);

    let variable_name = VariableName::new("counter");
    let variable = Variable::new(
        variable_name.clone(),
        VariableKind::Number(VariableKindNumber::new(6.))
    );
    engine.variables.insert(variable_name, variable);

    let variable_name = VariableName::new("document");
    let variable = Variable::new(
        variable_name.clone(),
        engine.functions.get(&FunctionName::new("root_element")).unwrap().run(),
    );
    engine.variables.insert(variable_name, variable);

    println!("{engine:#?}");

    El::new().child("Boon root")
} 
