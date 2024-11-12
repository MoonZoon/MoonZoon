use indexmap::IndexMap;
use std::fmt;
use std::sync::Arc;

pub type Functions = IndexMap<FunctionName, Function>;
pub type Arguments = IndexMap<ArgumentName, Argument>;
pub type Variables = IndexMap<VariableName, Variable>;

#[derive(Debug, Default)]
pub struct Engine {
    pub functions: Functions,
    pub variables: Variables,
}

pub struct Function {
    name: FunctionName,
    closure: Arc<dyn Fn(Arguments) -> VariableKind>,
}

impl fmt::Debug for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Function")
         .field("name", &self.name)
         .field("closure", &"[closure]")
         .finish()
    }
}

impl Function {
    pub fn new(name: FunctionName, closure: impl Fn(Arguments) -> VariableKind + 'static) -> Self {
        Self { name, closure: Arc::new(closure) }
    }

    pub fn run(&self, arguments: Arguments) -> VariableKind {
        (self.closure)(arguments)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FunctionName(Arc<String>);

impl FunctionName {
    pub fn new(name: impl ToString) -> Self {
        Self(Arc::new(name.to_string()))
    }
}

#[derive(Debug)]
pub struct Argument {
    name: ArgumentName,
    kind: VariableKind,
    out: bool,
}

impl Argument {
    pub fn new(name: ArgumentName, kind: VariableKind, out: bool) -> Self {
        Self { name, kind, out }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ArgumentName(Arc<String>);

impl ArgumentName {
    pub fn new(name: impl ToString) -> Self {
        Self(Arc::new(name.to_string()))
    }
}

#[derive(Debug)]
pub struct Variable {
    name: VariableName,
    kind: VariableKind,
}

impl Variable {
    pub fn new(name: VariableName, kind: VariableKind) -> Self {
        Self { name, kind }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VariableName(Arc<String>);

impl VariableName {
    pub fn new(name: impl ToString) -> Self {
        Self(Arc::new(name.to_string()))
    }
}

#[derive(Debug)]
pub enum VariableKind {
    Link(VariableKindLink),
    List(VariableKindList),
    Map(VariableKindMap),
    Number(VariableKindNumber),
    Object(VariableKindObject),
    Text(VariableKindText),
}

#[derive(Debug)]
pub struct VariableKindLink {
    variable: Option<Box<Variable>>
}

impl VariableKindLink {
    pub fn new() -> Self {
        Self { variable: None }
    }
}

#[derive(Debug)]
pub struct VariableKindList {

}

#[derive(Debug)]
pub struct VariableKindMap {

}

#[derive(Debug)]
pub struct VariableKindNumber {
    number: f64
}

impl VariableKindNumber {
    pub fn new(number: f64) -> Self {
        Self { number }
    }
}

#[derive(Debug)]
pub struct VariableKindObject {
    variables: Variables
}

impl VariableKindObject {
    pub fn new(variables: Variables) -> Self {
        Self { variables }
    }
}

#[derive(Debug)]
pub struct VariableKindText {

}








