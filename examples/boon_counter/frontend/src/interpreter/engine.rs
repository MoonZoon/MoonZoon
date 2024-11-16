use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

use indexmap::IndexMap;

use zoon::futures_channel::{oneshot, mpsc};
use zoon::futures_util::StreamExt;
use zoon::futures_util::future::join_all;
use zoon::{Task, TaskHandle};

pub type Functions = IndexMap<FunctionName, Function>;
pub type Arguments = IndexMap<ArgumentName, Argument>;
pub type Variables = IndexMap<VariableName, Variable>;

pub trait AsyncDebugFormat {
    async fn async_debug_format(&self) -> String {
        self.async_debug_format_with_formatter(Formatter::default()).await
    }

    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String;
}

#[derive(Debug, Clone, Copy)]
pub struct Formatter {
    indent_spaces: u32,
    indent_level: u32,
}

impl Default for Formatter {
    fn default() -> Self {
        Self {
            indent_spaces: 4,
            indent_level: 0,
        }
    }
}

impl Formatter {
    pub fn increase_level(self) -> Formatter {
        let mut updated = self.clone();
        updated.indent_level += 1;
        updated
    }

    pub fn indent(&self, text: &str) -> String {
        let indentation = (self.indent_spaces * self.indent_level) as usize;
        format!("{:indentation$}{text}", "")
    }
}

#[derive(Debug, Default)]
pub struct Engine {
    pub functions: Functions,
    pub variables: Variables,
}

impl Engine {
    // @TODO `address` should work for the scope, not only for the root
    pub async fn set_link_value(&self, address: &str, actor: VariableActor) {
        let address_parts = address.split(".").collect::<Vec<_>>();

        if address_parts.len() == 1 {
            let link_actor = self
                .variables
                .get(&VariableName::new(address_parts[0]))
                .unwrap()
                .actor();
            link_actor.set_value(VariableValue::Link(VariableValueLink { 
                actor: Some(Arc::new(actor))
            }));
        } else {
            let root = self
                .variables
                .get(&VariableName::new(address_parts[0]))
                .unwrap()
                .actor();

            let mut parent_or_link_actor = root;
            for address_part in address_parts.into_iter().skip(1) {
                parent_or_link_actor = match parent_or_link_actor.get_value().await.unwrap() {
                    VariableValue::Object(VariableValueObject { variables }) => {
                        variables
                            .get(&VariableName::new(address_part))
                            .unwrap()
                            .actor()
                    }
                    _ => unreachable!("Link path parts have to be 'VariableValue::Object'")
                }
            }
            parent_or_link_actor.set_value(VariableValue::Link(VariableValueLink { 
                actor: Some(Arc::new(actor))
            }));
        }
    }
}

impl AsyncDebugFormat for Engine {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        let mut output = String::new();
        output.push_str("--- ENGINE ---\n");

        let functions = {
            let formatter = formatter.increase_level();
            self
                .functions
                .keys()
                .map(|FunctionName(name)| formatter.indent(name))
                .collect::<Vec<_>>()
                .join("\n")
        };
        output.push_str(&format!("functions: LIST {{\n{functions}\n}}\n"));

        let variables = {
            let formatter = formatter.increase_level();
            self
                .variables
                .values()
                .map(move |variable| async move {
                    let variable = variable.async_debug_format_with_formatter(formatter).await;
                    formatter.indent(&format!("{variable},"))
                })
        };
        let variables = join_all(variables)
            .await
            .join("\n");
        output.push_str(&format!("variables: [\n{variables}\n]\n"));

        output.push_str("--------------\n");
        output
    }
}

pub struct Function {
    name: FunctionName,
    closure: Arc<dyn Fn(Arguments) -> Pin<Box<dyn Future<Output = VariableActor>>>>,
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
    pub fn new<FUT: Future<Output = VariableActor> + 'static>(name: FunctionName, closure: impl Fn(Arguments) -> FUT + 'static) -> Self {
        let closure = Arc::new(move |arguments: Arguments| { 
            Box::pin(closure(arguments)) as Pin<Box<dyn Future<Output = VariableActor>>>
        });
        Self { name, closure }
    }

    pub async fn run(&self, arguments: Arguments) -> VariableActor {
        (self.closure)(arguments).await
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

impl AsyncDebugFormat for Variable {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        let VariableName(name) = &self.name;
        let value = self.actor.async_debug_format_with_formatter(formatter).await;
        format!("{name}: {value}")
    }
}

enum VariableActorMessage {
    GetValue { value_sender: oneshot::Sender<Option<VariableValue>> },
    SetValue { new_value: VariableValue }
}

// @TODO Don't clone - only weak references
#[derive(Debug, Clone)]
pub struct VariableActor {
    task_handle: Arc<TaskHandle>,
    message_sender: mpsc::UnboundedSender<VariableActorMessage>,
}

impl VariableActor {
    pub fn new(default_value: Option<VariableValue>) -> Self {
        let mut value = default_value;
        let (message_sender, mut message_receiver) = mpsc::unbounded::<VariableActorMessage>();

        let task_handle = Task::start_droppable(async move {
            while let Some(message) = message_receiver.next().await {
                match message {
                    VariableActorMessage::GetValue { value_sender } => {
                        value_sender.send(value.clone()).unwrap();
                    }
                    VariableActorMessage::SetValue { new_value } => {
                        value = Some(new_value);
                    }
                }
            }
        });
        Self {
            task_handle: Arc::new(task_handle),
            message_sender
        }
    }

    pub async fn get_value(&self) -> Option<VariableValue> {
        let (value_sender, value_receiver) = oneshot::channel();
        let message = VariableActorMessage::GetValue {
            value_sender
        };
        self.message_sender.unbounded_send(message).unwrap();
        value_receiver.await.unwrap()
    }

    pub fn set_value(&self, new_value: VariableValue) {
        let message = VariableActorMessage::SetValue {
            new_value
        };
        self.message_sender.unbounded_send(message).unwrap()
    }
}

impl AsyncDebugFormat for VariableActor {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        if let Some(value) = self.get_value().await {
            value.async_debug_format_with_formatter(formatter).await
        } else {
            "UNSET".to_owned()
        }
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

impl AsyncDebugFormat for VariableValue {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        match self {
            Self::Link(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::List(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::Map(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::Number(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::Object(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::TaggedObject(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::Tag(value) => value.async_debug_format_with_formatter(formatter).await,
            Self::Text(value) => value.async_debug_format_with_formatter(formatter).await,
        }
    }
}

// --- VariableValueLink ---

#[derive(Debug, Clone)]
pub struct VariableValueLink {
    actor: Option<Arc<VariableActor>>
}

impl VariableValueLink {
    pub fn new() -> Self {
        Self { actor: None }
    }
}

impl AsyncDebugFormat for VariableValueLink {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        if let Some(actor) = &self.actor {
            let actor = Box::pin(actor.async_debug_format_with_formatter(formatter)).await;
            format!("LINK {{ {actor} }}")
        } else {
            "LINK { UNSET }".to_owned()
        }
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

impl AsyncDebugFormat for VariableValueList {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        if self.list.is_empty() {
            return String::from("LIST {}")
        }
        let mut output = String::new();
        output.push_str("LIST {\n");

        let values = {
            let formatter = formatter.increase_level();
            self.list.iter().map(move |value| async move {
                formatter.indent(&value.async_debug_format_with_formatter(formatter).await)
            })
        };
        output.push_str(&join_all(values).await.join("\n"));
        
        output.push_str("\n");
        output.push_str(&formatter.indent("}"));
        output
    }
}

// --- VariableValueMap ---

#[derive(Debug, Clone)]
pub struct VariableValueMap {

}

impl AsyncDebugFormat for VariableValueMap {
    async fn async_debug_format_with_formatter(&self, _: Formatter) -> String {
        String::from("MAP { @TODO }")
    }
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

impl AsyncDebugFormat for VariableValueNumber {
    async fn async_debug_format_with_formatter(&self, _: Formatter) -> String {
        self.number.to_string()
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

impl AsyncDebugFormat for VariableValueObject {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        if self.variables.is_empty() {
            return String::from("[]")
        }
        let mut output = String::new();
        output.push_str("[\n");

        let variables = { 
            let formatter = formatter.increase_level();
            self.variables.values().map(move |variable| async move {
                formatter.indent(&variable.async_debug_format_with_formatter(formatter).await)
            })
        };
        output.push_str(&join_all(variables).await.join("\n"));
        output.push_str("\n");
        output.push_str(&formatter.indent("]"));
        output
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

impl AsyncDebugFormat for VariableValueTaggedObject {
    async fn async_debug_format_with_formatter(&self, formatter: Formatter) -> String {
        let tag = &self.tag;

        if self.variables.is_empty() {
            return format!("{tag}[]")
        }

        let mut output = String::new();
        output.push_str(&format!("{tag}[\n"));

        let variables = { 
            let formatter = formatter.increase_level();
            self.variables.values().map(move |variable| async move {
                let variable = variable.async_debug_format_with_formatter(formatter).await;
                formatter.indent(&variable)
            })
        };
        output.push_str(&join_all(variables).await.join("\n"));
        output.push_str("\n");
        output.push_str(&formatter.indent("]"));
        output
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

impl AsyncDebugFormat for VariableValueTag {
    async fn async_debug_format_with_formatter(&self, _: Formatter) -> String {
        self.tag.clone()
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

impl AsyncDebugFormat for VariableValueText {
    async fn async_debug_format_with_formatter(&self, _: Formatter) -> String {
        let text = &self.text;
        format!("'{text}'")
    }
}
