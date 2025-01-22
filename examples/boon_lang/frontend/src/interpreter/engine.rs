use std::pin::{Pin, pin};
use std::sync::Arc;
use std::borrow::Cow;

use zoon::futures_channel::{oneshot, mpsc};
use zoon::futures_util::stream::{self, Stream, StreamExt, BoxStream};
use zoon::{Task, TaskHandle};
use zoon::future;
use zoon::{println, eprintln};
use zoon::futures_util::select;

use pin_project::pin_project;

pub fn constant<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item)).chain(stream::once(future::pending()))
}

// --- ConstructInfo ---

pub struct ConstructInfo {
    id: ConstructId,
    description: &'static str,
}

impl ConstructInfo {
    pub fn new(id: impl Into<ConstructId>, description: &'static str) -> Self {
        Self {
            id: id.into(),
            description,
        }
    }

    pub fn complete(self, r#type: ConstructType) -> ConstructInfoComplete {
        ConstructInfoComplete {
            r#type,
            id: self.id,
            description: self.description
        }
    }
}

// --- ConstructInfoComplete ---

pub struct ConstructInfoComplete {
    r#type: ConstructType,
    id: ConstructId,
    description: &'static str,
}

impl ConstructInfoComplete {
    pub fn id(&self) -> &ConstructId {
        &self.id
    }
}

impl std::fmt::Display for ConstructInfoComplete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} {:?} '{}')", self.r#type, self.id.0, self.description)
    }
}

// --- ConstructType ---

#[derive(Debug)]
pub enum ConstructType {
    Variable,
    ValueActor,
    Object,
    Text,
}

// --- ConstructId ---

#[derive(Clone, Debug)]
pub struct ConstructId(Vec<u64>);

impl ConstructId {
    pub fn new(id: u64) -> Self {
        Self(vec![id])
    }

    pub fn with_child_id(&self, child: u64) -> Self {
        let mut cloned = self.clone();
        cloned.0.push(child);
        cloned
    }
}

impl From<u64> for ConstructId {
    fn from(value: u64) -> Self {
        ConstructId(vec![value])
    }
}

// --- Variable ---

pub struct Variable {
    construct_info: ConstructInfoComplete,
    name: &'static str,
    value_actor: Arc<ValueActor>,
}

impl Variable {
    pub fn new(construct_info: ConstructInfo, name: &'static str, value_actor: Arc<ValueActor>) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Variable),
            name,
            value_actor
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, name: &'static str, value_actor: Arc<ValueActor>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, name, value_actor))
    }

    pub fn subscribe(&self) -> impl Stream<Item = Value> {
        self.value_actor.subscribe()
    }
}

impl Drop for Variable {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- ValueActor ---

pub struct ValueActor {
    construct_info: Arc<ConstructInfoComplete>,
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
}

impl ValueActor {
    pub fn new(construct_info: ConstructInfo, value_stream: impl Stream<Item = Value> + 'static) -> Self {
        let construct_info = Arc::new(construct_info.complete(ConstructType::ValueActor));
        let (value_sender_sender, mut value_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
        let loop_task = Task::start_droppable({
            let construct_info = construct_info.clone();
            async move {
                let mut value_stream = pin!(value_stream.fuse());
                let mut value = None;
                let mut value_senders = Vec::<mpsc::UnboundedSender<Value>>::new();
                loop {
                    select! {
                        new_value = value_stream.next() => {
                            let Some(new_value) = new_value else { break };
                            value_senders.retain(|value_sender| {
                                if let Err(error) = value_sender.unbounded_send(new_value.clone()) {
                                    eprintln!("Failed to send new ValueActor value to subscriber: {error:#}");
                                    false
                                } else {
                                    true
                                }
                            });
                            value = Some(new_value);
                        }
                        value_sender = value_sender_receiver.select_next_some() => {
                            if let Some(value) = value.as_ref() {
                                if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                    eprintln!("Failed to send ValueActor value to subscriber: {error:#}");
                                } else {
                                    value_senders.push(value_sender);
                                }
                            } else {
                                value_senders.push(value_sender);
                            }
                        }
                    }
                }
                println!("Loop ended {construct_info}");
            }
        });
        Self {
            construct_info,
            loop_task,
            value_sender_sender,
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, value_stream: impl Stream<Item = Value> + 'static) -> Arc<Self> {
        Arc::new(Self::new(construct_info, value_stream))
    }

    pub fn subscribe(&self) -> impl Stream<Item = Value> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to subscribe to {}: {error:#}", self.construct_info);
        }
        value_receiver
    }
}

impl Drop for ValueActor {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- Value ---

#[derive(Clone)]
pub enum Value {
    Object(Arc<Object>),
    Text(Arc<Text>),
}

impl Value {
    pub fn expect_object(self) -> Arc<Object> {
        let Self::Object(object) = self else {
            panic!("Failed to get expected Object: The Value has a different type")
        };
        object
    }

    pub fn expect_text(self) -> Arc<Text> {
        let Self::Text(text) = self else {
            panic!("Failed to get expected Text: The Value has a different type")
        };
        text
    }
}

// --- Object ---

pub struct Object {
    construct_info: ConstructInfoComplete,
    variables: Vec<Arc<Variable>>,
}

impl Object {
    pub fn new(construct_info: ConstructInfo, variables: Vec<Arc<Variable>>) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Object),
            variables
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, variables: Vec<Arc<Variable>>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, variables))
    }

    pub fn new_constant(construct_info: ConstructInfo, variables: Vec<Arc<Variable>>) -> impl Stream<Item = Value> {
        constant(Value::Object(Self::new_arc(construct_info, variables)))
    }

    pub fn expect_variable(&self, name: &str) -> Arc<Variable> {
        let Some(index) = self.variables.iter().position(|variable| { 
            variable.name == name
        }) else {
            panic!("Failed to get expected Variable '{name}' from {}", self.construct_info);
        };
        self.variables[index].clone()
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- Text ---

pub struct Text {
    construct_info: ConstructInfoComplete,
    text: Cow<'static, str>,
}

impl Text {
    pub fn new(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Text),
            text: text.into()
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, text))
    }

    pub fn new_constant(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> impl Stream<Item = Value> {
        constant(Value::Text(Self::new_arc(construct_info, text)))
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}
