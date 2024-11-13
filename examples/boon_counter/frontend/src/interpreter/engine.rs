use indexmap::IndexMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use zoon::futures_channel::oneshot;

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

#[derive(Debug, Clone)]
pub struct Argument {
    name: ArgumentName,
    in_out: ArgumentInOut
}

#[derive(Debug, Clone)]
pub enum ArgumentInOut {
    In(ArgumentIn),
    Out(ArgumentOut),
}

#[derive(Debug, Clone)]
pub struct ArgumentIn {
    kind: VariableKind,
}

impl ArgumentIn {
    pub fn kind(&self) -> VariableKind {
        self.kind.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentOut {
    kind_sender: Arc<Mutex<Option<oneshot::Sender<VariableKind>>>>,
}

impl ArgumentOut {
    pub fn send_kind(&self, kind: VariableKind) {
        self
            .kind_sender
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .send(kind)
            .unwrap()
    }
}

impl Argument {
    pub fn new_in(name: ArgumentName, kind: VariableKind) -> Self {
        Self { name, in_out: ArgumentInOut::In(ArgumentIn { kind }) }
    }

    pub fn new_out(name: ArgumentName) -> (Self, oneshot::Receiver<VariableKind>) {
        let (kind_sender, kind_receiver) = oneshot::channel();
        let this = Self { 
            name, 
            in_out: ArgumentInOut::Out(ArgumentOut { 
                kind_sender: Arc::new(Mutex::new(Some(kind_sender))) 
            })
        };
        (this, kind_receiver)
    }

    pub fn argument_in(&self) -> Option<&ArgumentIn> {
        match &self.in_out {
            ArgumentInOut::In(argument_in) => Some(argument_in),
            _ => None
        }
    }

    pub fn argument_out(&self) -> Option<&ArgumentOut> {
        match &self.in_out {
            ArgumentInOut::Out(argument_out) => Some(argument_out),
            _ => None
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct ArgumentName(Arc<String>);

impl ArgumentName {
    pub fn new(name: impl ToString) -> Self {
        Self(Arc::new(name.to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    name: VariableName,
    kind: VariableKind,
}

impl Variable {
    pub fn new(name: VariableName, kind: VariableKind) -> Self {
        Self { name, kind }
    }

    pub fn kind(&self) -> VariableKind {
        self.kind.clone()
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VariableName(Arc<String>);

impl VariableName {
    pub fn new(name: impl ToString) -> Self {
        Self(Arc::new(name.to_string()))
    }
}

#[derive(Debug, Clone)]
pub enum VariableKind {
    Link(VariableKindLink),
    List(VariableKindList),
    Map(VariableKindMap),
    Number(VariableKindNumber),
    Object(VariableKindObject),
    TaggedObject(VariableKindTaggedObject),
    Tag(VariableKindTag),
    Text(VariableKindText),
}

// --- VariableKindLink ---

#[derive(Debug, Clone)]
pub struct VariableKindLink {
    variable: Option<Arc<Variable>>
}

impl VariableKindLink {
    pub fn new() -> Self {
        Self { variable: None }
    }
}

// --- VariableKindList ---

#[derive(Debug, Clone)]
pub struct VariableKindList {
    list: Vec<VariableKind>
}

impl VariableKindList {
    pub fn new(list: Vec<VariableKind>) -> Self {
        Self { list }
    }
}

// --- VariableKindMap ---

#[derive(Debug, Clone)]
pub struct VariableKindMap {

}

// --- VariableKindNumber ---

#[derive(Debug, Clone)]
pub struct VariableKindNumber {
    number: f64
}

impl VariableKindNumber {
    pub fn new(number: f64) -> Self {
        Self { number }
    }
}

// --- VariableKindObject ---

#[derive(Debug, Clone)]
pub struct VariableKindObject {
    variables: Variables
}

impl VariableKindObject {
    pub fn new(variables: Variables) -> Self {
        Self { variables }
    }
}

// --- VariableKindTaggedObject ---

#[derive(Debug, Clone)]
pub struct VariableKindTaggedObject {
    tag: String,
    variables: Variables
}

impl VariableKindTaggedObject {
    pub fn new(tag: impl ToString, variables: Variables) -> Self {
        Self { tag: tag.to_string(), variables }
    }
}

// --- VariableKindTag ---

#[derive(Debug, Clone)]
pub struct VariableKindTag {
    tag: String
}

impl VariableKindTag {
    pub fn new(tag: impl ToString) -> Self {
        Self { tag: tag.to_string() }
    }
}

// --- VariableKindText ---

#[derive(Debug, Clone)]
pub struct VariableKindText {
    text: String
}

impl VariableKindText {
    pub fn new(text: impl ToString) -> Self {
        Self { text: text.to_string() }
    }
}
