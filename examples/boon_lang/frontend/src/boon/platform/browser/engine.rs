// @TODO remove
#![allow(dead_code)]

use std::pin::pin;
use std::sync::Arc;
use std::borrow::Cow;

use zoon::futures_channel::mpsc;
use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::{Task, TaskHandle};
use zoon::future;
use zoon::{println, eprintln};
use zoon::futures_util::select;

// --- PipeTo ---

pub trait PipeTo {
    fn pipe_to<FR>(self, f: impl FnOnce(Self) -> FR) -> FR where Self: Sized {
        f(self)
    }
}

impl<T> PipeTo for T {}

// --- constant ---

pub fn constant<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item)).chain(stream::once(future::pending()))
}

// --- Run ---

pub enum RunDuration {
    Nonstop,
    UntilFirstValue,
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
    pub fn id(&self) -> ConstructId {
        self.id.clone()
    }
}

impl std::fmt::Display for ConstructInfoComplete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} {:?} '{}')", self.r#type, self.id.ids, self.description)
    }
}

// --- ConstructType ---

#[derive(Debug, Clone, Copy)]
pub enum ConstructType {
    Variable,
    VariableReference,
    FunctionCall,
    ThenCombinator,
    ValueActor,
    Object,
    TaggedObject,
    Text,
    Tag,
    Number,
    List,
}

// --- ConstructId ---

#[derive(Clone, Debug)]
pub struct ConstructId {
    ids: Arc<Vec<u64>>
}

impl ConstructId {
    pub fn new(id: u64) -> Self {
        Self { ids: Arc::new(vec![id]) }
    }

    pub fn with_child_id(&self, child: u64) -> Self {
        let mut ids = Vec::clone(&self.ids);
        ids.push(child);
        Self { ids: Arc::new(ids) }
    }
}

impl From<u64> for ConstructId {
    fn from(id: u64) -> Self {
        ConstructId::new(id)
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

    pub fn value_actor(&self) -> Arc<ValueActor> {
        self.value_actor.clone()
    }
}

impl Drop for Variable {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- VariableReference ---

pub struct VariableReference {}

impl VariableReference {
    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        alias: &'static str,
        variable_receiver: mpsc::UnboundedReceiver<Arc<Variable>>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::VariableReference);
        let value_stream = variable_receiver
            .flat_map(|variable| variable.subscribe());
        Arc::new(ValueActor::new_internal(construct_info, run_duration, value_stream, ()))
    }
}

// --- FunctionCall ---

pub struct FunctionCall {}

impl FunctionCall {
    pub fn new_arc_value_actor<const AN: usize, FR: Stream<Item = Value> + 'static>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        definition: impl Fn([Arc<ValueActor>; AN], ConstructId) -> FR + 'static,
        arguments: [Arc<ValueActor>; AN],
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::FunctionCall);
        let value_stream = definition(arguments.clone(), construct_info.id());
        Arc::new(ValueActor::new_internal(construct_info, run_duration, value_stream, arguments))
    }
}

// --- ThenCombinator ---

pub struct ThenCombinator {}

impl ThenCombinator {
    pub fn new_arc_value_actor<FR: Stream<Item = Value> + 'static>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        observed: Arc<ValueActor>,
        stream_on_change: impl Fn() -> FR + 'static,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::ThenCombinator);
        let stream_on_change = Arc::new(stream_on_change);
        let value_stream = observed.subscribe().filter_map(move |_| {
            let stream_on_change = stream_on_change.clone(); 
            async move { pin!(stream_on_change()).next().await }
        });
        Arc::new(ValueActor::new_internal(construct_info, run_duration, value_stream, observed))
    }
}

// --- ValueActor ---

pub struct ValueActor {
    construct_info: Arc<ConstructInfoComplete>,
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
}

impl ValueActor {
    pub fn new(construct_info: ConstructInfo, run_duration: RunDuration, value_stream: impl Stream<Item = Value> + 'static) -> Self {
        let construct_info = construct_info.complete(ConstructType::ValueActor);
        Self::new_internal(construct_info, run_duration, value_stream, ())
    }

    fn new_internal<EOD: 'static>(
        construct_info: ConstructInfoComplete,
        run_duration: RunDuration,
        value_stream: impl Stream<Item = Value> + 'static,
        extra_owned_data: EOD,
    ) -> Self {
        let construct_info = Arc::new(construct_info);
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
                                    eprintln!("Failed to send new {} value to subscriber: {error:#}", construct_info);
                                    false
                                } else {
                                    true
                                }
                            });
                            value = Some(new_value);
                            if let RunDuration::UntilFirstValue = run_duration {
                                break
                            }
                        }
                        value_sender = value_sender_receiver.select_next_some() => {
                            if let Some(value) = value.as_ref() {
                                if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                    eprintln!("Failed to send {} value to subscriber: {error:#}", construct_info);
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
                drop(extra_owned_data);
            }
        });
        Self {
            construct_info,
            loop_task,
            value_sender_sender,
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, run_duration: RunDuration, value_stream: impl Stream<Item = Value> + 'static) -> Arc<Self> {
        Arc::new(Self::new(construct_info, run_duration, value_stream))
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
    TaggedObject(Arc<TaggedObject>),
    Text(Arc<Text>),
    Tag(Arc<Tag>),
    Number(Arc<Number>),
    List(Arc<List>),
}

impl Value {
    pub fn expect_object(self) -> Arc<Object> {
        let Self::Object(object) = self else {
            panic!("Failed to get expected Object: The Value has a different type")
        };
        object
    }

    pub fn expect_tagged_object(self, tag: &str) -> Arc<TaggedObject> {
        let Self::TaggedObject(tagged_object) = self else {
            panic!("Failed to get expected TaggedObject: The Value has a different type")
        };
        let found_tag = tagged_object.tag;
        if found_tag != tag {
            panic!("Failed to get expected TaggedObject: Expected tag: '{tag}', found tag: '{found_tag}'")
        }
        tagged_object
    }

    pub fn expect_text(self) -> Arc<Text> {
        let Self::Text(text) = self else {
            panic!("Failed to get expected Text: The Value has a different type")
        };
        text
    }

    pub fn expect_tag(self) -> Arc<Tag> {
        let Self::Tag(tag) = self else {
            panic!("Failed to get expected Tag: The Value has a different type")
        };
        tag
    }

    pub fn expect_number(self) -> Arc<Number> {
        let Self::Number(number) = self else {
            panic!("Failed to get expected Number: The Value has a different type")
        };
        number
    }

    pub fn expect_list(self) -> Arc<List> {
        let Self::List(list) = self else {
            panic!("Failed to get expected List: The Value has a different type")
        };
        list
    }
}

// --- Object ---

pub struct Object {
    construct_info: ConstructInfoComplete,
    variables: Box<[Arc<Variable>]>,
}

impl Object {
    pub fn new<const VN: usize>(construct_info: ConstructInfo, variables: [Arc<Variable>; VN]) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Object),
            variables: Box::new(variables)
        }
    }

    pub fn new_arc<const VN: usize>(construct_info: ConstructInfo, variables: [Arc<Variable>; VN]) -> Arc<Self> {
        Arc::new(Self::new(construct_info, variables))
    }

    pub fn new_value<const VN: usize>(construct_info: ConstructInfo, variables: [Arc<Variable>; VN]) -> Value {
        Value::Object(Self::new_arc(construct_info, variables))
    }

    pub fn new_constant<const VN: usize>(construct_info: ConstructInfo, variables: [Arc<Variable>; VN]) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, variables))
    }

    pub fn new_arc_value_actor<const VN: usize>(construct_info: ConstructInfo, run_duration: RunDuration, variables: [Arc<Variable>; VN]) -> Arc<ValueActor> {
        let ConstructInfo { id: actor_id, description: tagged_object_description } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), tagged_object_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant object wrapper").complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, variables);
        Arc::new(ValueActor::new_internal(actor_construct_info, run_duration, value_stream, ()))
    }

    pub fn variable(&self, name: &str) -> Option<Arc<Variable>> {
        self
            .variables
            .iter()
            .position(|variable| variable.name == name)
            .map(|index| self.variables[index].clone())
    }

    pub fn expect_variable(&self, name: &str) -> Arc<Variable> {
        self.variable(name).unwrap_or_else(|| {
            panic!("Failed to get expected Variable '{name}' from {}", self.construct_info)
        })
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- Object ---

pub struct TaggedObject {
    construct_info: ConstructInfoComplete,
    tag: &'static str,
    variables: Box<[Arc<Variable>]>,
}

impl TaggedObject {
    pub fn new<const VN: usize>(construct_info: ConstructInfo, tag: &'static str, variables: [Arc<Variable>; VN]) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::TaggedObject),
            tag,
            variables: Box::new(variables)
        }
    }

    pub fn new_arc<const VN: usize>(construct_info: ConstructInfo, tag: &'static str, variables: [Arc<Variable>; VN]) -> Arc<Self> {
        Arc::new(Self::new(construct_info, tag, variables))
    }

    pub fn new_value<const VN: usize>(construct_info: ConstructInfo, tag: &'static str, variables: [Arc<Variable>; VN]) -> Value {
        Value::TaggedObject(Self::new_arc(construct_info, tag, variables))
    }

    pub fn new_constant<const VN: usize>(construct_info: ConstructInfo, tag: &'static str, variables: [Arc<Variable>; VN]) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, tag, variables))
    }

    pub fn new_arc_value_actor<const VN: usize>(construct_info: ConstructInfo, run_duration: RunDuration, tag: &'static str, variables: [Arc<Variable>; VN]) -> Arc<ValueActor> {
        let ConstructInfo { id: actor_id, description: tagged_object_description } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), tagged_object_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Tagged object wrapper").complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, tag, variables);
        Arc::new(ValueActor::new_internal(actor_construct_info, run_duration, value_stream, ()))
    }

    pub fn variable(&self, name: &str) -> Option<Arc<Variable>> {
        self
            .variables
            .iter()
            .position(|variable| variable.name == name)
            .map(|index| self.variables[index].clone())
    }

    pub fn expect_variable(&self, name: &str) -> Arc<Variable> {
        self.variable(name).unwrap_or_else(|| {
            panic!("Failed to get expected Variable '{name}' from {}", self.construct_info)
        })
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}

impl Drop for TaggedObject {
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

    pub fn new_value(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> Value {
        Value::Text(Self::new_arc(construct_info, text))
    }

    pub fn new_constant(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, text))
    }

    pub fn new_arc_value_actor(construct_info: ConstructInfo, run_duration: RunDuration, text: impl Into<Cow<'static, str>>) -> Arc<ValueActor> {
        let ConstructInfo { id: actor_id, description: text_description } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), text_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant text wrapper").complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, text.into());
        Arc::new(ValueActor::new_internal(actor_construct_info, run_duration, value_stream, ()))
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

// --- Tag ---

pub struct Tag {
    construct_info: ConstructInfoComplete,
    tag: Cow<'static, str>,
}

impl Tag {
    pub fn new(construct_info: ConstructInfo, tag: impl Into<Cow<'static, str>>) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Tag),
            tag: tag.into()
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, tag: impl Into<Cow<'static, str>>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, tag))
    }

    pub fn new_value(construct_info: ConstructInfo, tag: impl Into<Cow<'static, str>>) -> Value {
        Value::Tag(Self::new_arc(construct_info, tag))
    }

    pub fn new_constant(construct_info: ConstructInfo, tag: impl Into<Cow<'static, str>>) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, tag))
    }

    pub fn new_arc_value_actor(construct_info: ConstructInfo, run_duration: RunDuration, tag: impl Into<Cow<'static, str>>) -> Arc<ValueActor> {
        let ConstructInfo { id: actor_id, description: tag_description } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), tag_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant tag wrapper").complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, tag.into());
        Arc::new(ValueActor::new_internal(actor_construct_info, run_duration, value_stream, ()))
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}

impl Drop for Tag {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- Number ---

pub struct Number {
    construct_info: ConstructInfoComplete,
    number: f64,
}

impl Number {
    pub fn new(construct_info: ConstructInfo, number: impl Into<f64>) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Number),
            number: number.into()
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, number: impl Into<f64>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, number))
    }

    pub fn new_value(construct_info: ConstructInfo, number: impl Into<f64>) -> Value {
        Value::Number(Self::new_arc(construct_info, number))
    }

    pub fn new_constant(construct_info: ConstructInfo, number: impl Into<f64>) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, number))
    }

    pub fn new_arc_value_actor(construct_info: ConstructInfo, run_duration: RunDuration, number: impl Into<f64>) -> Arc<ValueActor> {
        let ConstructInfo { id: actor_id, description: number_description } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), number_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant number wrapper)").complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, number.into());
        Arc::new(ValueActor::new_internal(actor_construct_info, run_duration, value_stream, ()))
    }

    pub fn number(&self) -> f64 {
        self.number
    }
}

impl Drop for Number {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- List ---

// @TODO change streams?
pub struct List {
    construct_info: ConstructInfoComplete,
    items: Vec<Arc<ValueActor>>,
}

impl List {
    pub fn new<const IN: usize>(construct_info: ConstructInfo, items: [Arc<ValueActor>; IN]) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::List),
            items: Vec::from(items),
        }
    }

    pub fn new_arc<const IN: usize>(construct_info: ConstructInfo, items: [Arc<ValueActor>; IN]) -> Arc<Self> {
        Arc::new(Self::new(construct_info, items))
    }

    pub fn new_value<const IN: usize>(construct_info: ConstructInfo, items: [Arc<ValueActor>; IN]) -> Value {
        Value::List(Self::new_arc(construct_info, items))
    }

    pub fn new_constant<const IN: usize>(construct_info: ConstructInfo, items: [Arc<ValueActor>; IN]) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, items))
    }

    pub fn new_arc_value_actor<const IN: usize>(construct_info: ConstructInfo, run_duration: RunDuration, items: [Arc<ValueActor>; IN]) -> Arc<ValueActor> {
        let ConstructInfo { id: actor_id, description: list_description } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), list_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant list wrapper").complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, items);
        Arc::new(ValueActor::new_internal(actor_construct_info, run_duration, value_stream, ()))
    }

    pub fn items(&self) -> &[Arc<ValueActor>] {
        &self.items
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}
