use std::pin::{Pin, pin};
use std::sync::Arc;

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

// --- ConstructId ---

#[derive(Clone, Debug)]
pub struct ConstructId(Vec<u64>);

impl ConstructId {
    pub fn new(id: u64) -> Self {
        Self(vec![id])
    }

    pub fn push_child_id(&self, child: u64) -> Self {
        let mut cloned = self.clone();
        cloned.0.push(child);
        cloned
    }
}

// --- Variable ---

pub struct Variable {
    description: &'static str,
    id: ConstructId,
    name: &'static str,
    value_actor: Arc<ValueActor>,
}

impl Variable {
    pub fn new(description: &'static str, id: ConstructId, name: &'static str, value_actor: Arc<ValueActor>) -> Self {
        Self {
            description,
            id,
            name,
            value_actor
        }
    }

    pub fn value_actor(&self) -> Arc<ValueActor> {
        self.value_actor.clone()
    }
}

impl Drop for Variable {
    fn drop(&mut self) {
        println!("Variable dropped. Id: '{:?}', Description: '{}'", self.id, self.description);
    }
}

// --- ValueActor ---

pub struct ValueActor {
    description: &'static str,
    id: ConstructId,
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
}

impl ValueActor {
    pub fn new(description: &'static str, id: ConstructId, value_stream: impl Stream<Item = Value> + 'static) -> Self {
        let (value_sender_sender, mut value_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
        let loop_task = Task::start_droppable({
            let id = id.clone();
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
                println!("ValueActor loop ended. Id: '{:?}', Description: '{}'", id, description);
            }
        });
        Self {
            description,
            id,
            loop_task,
            value_sender_sender,
        }
    }

    pub fn subscribe(&self) -> impl Stream<Item = Value> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to subscribe to ValueActor Id: '{:?}', Description: '{}': {error:#}", self.id, self.description);
        }
        value_receiver
    }
}

impl Drop for ValueActor {
    fn drop(&mut self) {
        println!("ValueActor dropped. Id: '{:?}', Description: '{}'", self.id, self.description);
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
    description: &'static str,
    id: ConstructId,
    variables: Vec<Arc<Variable>>,
}

impl Object {
    pub fn new(description: &'static str, id: ConstructId, variables: Vec<Arc<Variable>>) -> Self {
        Self {
            description,
            id,
            variables
        }
    }

    pub fn expect_variable(&self, name: &str) -> Arc<Variable> {
        let Some(index) = self.variables.iter().position(|variable| { 
            variable.name == name
        }) else {
            panic!("Failed to get expected Variable '{name}' from Object Id: '{:?}', Description: '{}'", self.id, self.description);
        };
        self.variables[index].clone()
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Object dropped. Id: '{:?}', Description: '{}'", self.id, self.description);
    }
}

// --- Text ---

pub struct Text {
    description: &'static str,
    id: ConstructId,
    text: String,
}

impl Text {
    pub fn new(description: &'static str, id: ConstructId, text: String) -> Self {
        Self {
            description,
            id,
            text
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        println!("Text dropped. Id: '{:?}', Description: '{}'", self.id, self.description);
    }
}
