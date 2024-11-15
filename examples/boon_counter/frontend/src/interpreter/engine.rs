use std::fmt;
use std::sync::{Arc, Mutex};

use indexmap::IndexMap;

use zoon::futures_channel::{oneshot, mpsc};
use zoon::futures_util::StreamExt;
use zoon::{Task, TaskHandle};
use zoon::println;

pub type Functions = IndexMap<FunctionName, Function>;
pub type Arguments = IndexMap<ArgumentName, Argument>;
pub type Variables = IndexMap<VariableName, Variable>;

#[derive(Debug, Default)]
pub struct Engine {
    pub functions: Functions,
    pub variables: Variables,
}

impl Engine {
    pub fn print_functions(&self) {
        println!("FUNCTIONS: {:#?}", self.functions.values());
    }

    pub fn print_variables(&self) {
        println!("VARIABLES: {:#?}", self.variables.values());
    }
}

pub struct Function {
    name: FunctionName,
    closure: Arc<dyn Fn(Arguments) -> VariableActor>,
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
    pub fn new(name: FunctionName, closure: impl Fn(Arguments) -> VariableActor + 'static) -> Self {
        Self { name, closure: Arc::new(closure) }
    }

    pub fn run(&self, arguments: Arguments) -> VariableActor {
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
    actor: VariableActor,
}

impl ArgumentIn {
    pub fn actor(&self) -> VariableActor {
        self.actor.clone()
    }
}

#[derive(Debug, Clone)]
pub struct ArgumentOut {
    actor_sender: Arc<Mutex<Option<oneshot::Sender<VariableActor>>>>,
}

impl ArgumentOut {
    pub fn send_actor(&self, actor: VariableActor) {
        self
            .actor_sender
            .lock()
            .unwrap()
            .take()
            .unwrap()
            .send(actor)
            .unwrap()
    }
}

impl Argument {
    pub fn new_in(name: ArgumentName, actor: VariableActor) -> Self {
        Self { name, in_out: ArgumentInOut::In(ArgumentIn { actor }) }
    }

    pub fn new_out(name: ArgumentName) -> (Self, oneshot::Receiver<VariableActor>) {
        let (actor_sender, actor_receiver) = oneshot::channel();
        let this = Self { 
            name, 
            in_out: ArgumentInOut::Out(ArgumentOut { 
                actor_sender: Arc::new(Mutex::new(Some(actor_sender))) 
            })
        };
        (this, actor_receiver)
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
    actor: VariableActor,
}

impl Variable {
    pub fn new(name: VariableName, actor: VariableActor) -> Self {
        Self { name, actor }
    }

    pub fn actor(&self) -> VariableActor {
        self.actor.clone()
    }
}

#[derive(Debug, Clone)]
pub struct VariableActor {
    task_handle: Arc<TaskHandle>,
    value_sender_sender: mpsc::UnboundedSender<oneshot::Sender<Option<VariableValue>>>,
}

impl VariableActor {
    pub fn new(default_value: Option<VariableValue>) -> Self {
        let value = default_value;
        let (value_sender_sender, mut value_sender_receiver) = mpsc::unbounded::<oneshot::Sender<Option<VariableValue>>>();
        let task_handle = Task::start_droppable(async move {
            while let Some(value_sender) = value_sender_receiver.next().await {
                value_sender.send(value.clone()).unwrap();
            }
        });
        Self {
            task_handle: Arc::new(task_handle),
            value_sender_sender
        }
    }

    pub async fn get_value(&self) -> Option<VariableValue> {
        let (value_sender, value_receiver) = oneshot::channel();
        self.value_sender_sender.unbounded_send(value_sender).unwrap();
        value_receiver.await.unwrap()
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
pub enum VariableValue {
    Link(VariableValueLink),
    List(VariableValueList),
    Map(VariableValueMap),
    Number(VariableValueNumber),
    Object(VariableValueObject),
    TaggedObject(VariableValueTaggedObject),
    Tag(VariableValueTag),
    Text(VariableValueText),
}

// --- VariableValueLink ---

#[derive(Debug, Clone)]
pub struct VariableValueLink {
    variable: Option<Arc<Variable>>
}

impl VariableValueLink {
    pub fn new() -> Self {
        Self { variable: None }
    }
}

// --- VariableValueList ---

#[derive(Debug, Clone)]
pub struct VariableValueList {
    list: Vec<VariableActor>
}

impl VariableValueList {
    pub fn new(list: Vec<VariableActor>) -> Self {
        Self { list }
    }
}

// --- VariableValueMap ---

#[derive(Debug, Clone)]
pub struct VariableValueMap {

}

// --- VariableValueNumber ---

#[derive(Debug, Clone)]
pub struct VariableValueNumber {
    number: f64
}

impl VariableValueNumber {
    pub fn new(number: f64) -> Self {
        Self { number }
    }
}

// --- VariableValueObject ---

#[derive(Debug, Clone)]
pub struct VariableValueObject {
    variables: Variables
}

impl VariableValueObject {
    pub fn new(variables: Variables) -> Self {
        Self { variables }
    }
}

// --- VariableValueTaggedObject ---

#[derive(Debug, Clone)]
pub struct VariableValueTaggedObject {
    tag: String,
    variables: Variables
}

impl VariableValueTaggedObject {
    pub fn new(tag: impl ToString, variables: Variables) -> Self {
        Self { tag: tag.to_string(), variables }
    }
}

// --- VariableValueTag ---

#[derive(Debug, Clone)]
pub struct VariableValueTag {
    tag: String
}

impl VariableValueTag {
    pub fn new(tag: impl ToString) -> Self {
        Self { tag: tag.to_string() }
    }
}

// --- VariableValueText ---

#[derive(Debug, Clone)]
pub struct VariableValueText {
    text: String
}

impl VariableValueText {
    pub fn new(text: impl ToString) -> Self {
        Self { text: text.to_string() }
    }
}
