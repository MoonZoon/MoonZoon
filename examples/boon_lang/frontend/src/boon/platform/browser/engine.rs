// @TODO remove
#![allow(dead_code)]

use std::borrow::Cow;
use std::pin::pin;
use std::sync::Arc;

use crate::boon::parser;

use zoon::future;
use zoon::futures_channel::mpsc;
use zoon::futures_util::select;
use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::{eprintln, println};
use zoon::{Task, TaskHandle};

// @TODO Replace `[]` with `impl Into<Vec..` and `&'static str` with `Cow<'static, str>` everywhere?


// --- PipeTo ---

pub trait PipeTo {
    fn pipe_to<FR>(self, f: impl FnOnce(Self) -> FR) -> FR
    where
        Self: Sized,
    {
        f(self)
    }
}

impl<T> PipeTo for T {}

// --- constant ---

pub fn constant<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item)).chain(stream::once(future::pending()))
}

// --- ActorContext ---

#[derive(Default, Clone)]
pub struct ActorContext {
    pub output_valve_signal: Option<Arc<ActorOutputValveSignal>>,
}

// --- ActorOutputValveSignal ---

pub struct ActorOutputValveSignal {
    impulse_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<()>>,
    loop_task: TaskHandle,
}

impl ActorOutputValveSignal {
    pub fn new(impulse_stream: impl Stream<Item = ()> + 'static) -> Self {
        let (impulse_sender_sender, mut impulse_sender_receiver) =
            mpsc::unbounded::<mpsc::UnboundedSender<()>>();
        Self {
            impulse_sender_sender,
            loop_task: Task::start_droppable(async move {
                let mut impulse_stream = pin!(impulse_stream.fuse());
                let mut impulse_senders = Vec::<mpsc::UnboundedSender<()>>::new();
                loop {
                    select! {
                        impulse = impulse_stream.next() => {
                            if impulse.is_none() { break };
                            impulse_senders.retain(|impulse_sender| {
                                if let Err(error) = impulse_sender.unbounded_send(()) {
                                    false
                                } else {
                                    true
                                }
                            });
                        }
                        impulse_sender = impulse_sender_receiver.select_next_some() => {
                            impulse_senders.push(impulse_sender);
                        }
                    }
                }
            })
        }
    }

    pub fn subscribe(&self) -> impl Stream<Item = ()> {
        let (impulse_sender, impulse_receiver) = mpsc::unbounded();
        if let Err(error) = self.impulse_sender_sender.unbounded_send(impulse_sender) {
            eprintln!("Failed to subscribe to actor output valve signal: {error:#}");
        }
        impulse_receiver
    }
}

// --- ConstructInfo ---

pub struct ConstructInfo {
    id: ConstructId,
    description: Cow<'static, str>,
}

impl ConstructInfo {
    pub fn new(id: impl Into<ConstructId>, description: impl Into<Cow<'static, str>>) -> Self {
        Self {
            id: id.into(),
            description: description.into(),
        }
    }

    pub fn complete(self, r#type: ConstructType) -> ConstructInfoComplete {
        ConstructInfoComplete {
            r#type,
            id: self.id,
            description: self.description,
        }
    }
}

// --- ConstructInfoComplete ---

#[derive(Clone)]
pub struct ConstructInfoComplete {
    r#type: ConstructType,
    id: ConstructId,
    description: Cow<'static, str>,
}

impl ConstructInfoComplete {
    pub fn id(&self) -> ConstructId {
        self.id.clone()
    }
}

impl std::fmt::Display for ConstructInfoComplete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({:?} {:?} '{}')",
            self.r#type, self.id.ids, self.description
        )
    }
}

// --- ConstructType ---

#[derive(Debug, Clone, Copy)]
pub enum ConstructType {
    Variable,
    LinkVariable,
    VariableOrArgumentReference,
    FunctionCall,
    LatestCombinator,
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
    ids: Arc<Vec<u64>>,
}

impl ConstructId {
    pub fn new(id: u64) -> Self {
        Self {
            ids: Arc::new(vec![id]),
        }
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
    name: Cow<'static, str>,
    value_actor: Arc<ValueActor>,
    link_value_sender: Option<mpsc::UnboundedSender<Value>>,
}

impl Variable {
    pub fn new(
        construct_info: ConstructInfo,
        name: impl Into<Cow<'static, str>>,
        value_actor: Arc<ValueActor>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Variable),
            name: name.into(),
            value_actor,
            link_value_sender: None,
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        name: impl Into<Cow<'static, str>>,
        value_actor: Arc<ValueActor>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, name, value_actor))
    }

    pub fn new_link_arc(
        construct_info: ConstructInfo,
        name: impl Into<Cow<'static, str>>,
        actor_context: ActorContext,
    ) -> Arc<Self> {
        let ConstructInfo {
            id: actor_id,
            description: variable_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), variable_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Link variable value actor")
            .complete(ConstructType::ValueActor);
        let (link_value_sender, link_value_receiver) = mpsc::unbounded();
        let value_actor =
            ValueActor::new_internal(actor_construct_info, actor_context, link_value_receiver, ());
        Arc::new(Self {
            construct_info: construct_info.complete(ConstructType::LinkVariable),
            name: name.into(),
            value_actor: Arc::new(value_actor),
            link_value_sender: Some(link_value_sender),
        })
    }

    pub fn subscribe(&self) -> impl Stream<Item = Value> {
        self.value_actor.subscribe()
    }

    pub fn value_actor(&self) -> Arc<ValueActor> {
        self.value_actor.clone()
    }

    pub fn link_value_sender(&self) -> Option<mpsc::UnboundedSender<Value>> {
        self.link_value_sender.clone()
    }

    pub fn expect_link_value_sender(&self) -> mpsc::UnboundedSender<Value> {
        if let Some(link_value_sender) = self.link_value_sender.clone() {
            link_value_sender
        } else {
            panic!(
                "Failed to get expected link value sender from {}",
                self.construct_info
            );
        }
    }
}

impl Drop for Variable {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- VariableOrArgumentReference ---

pub struct VariableOrArgumentReference {}

impl VariableOrArgumentReference {
    pub fn new_arc_value_actor<'code>(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        alias: parser::Alias<'code>,
        root_value_actor_receiver: mpsc::UnboundedReceiver<Arc<ValueActor>>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::VariableOrArgumentReference);
        let mut skip_alias_parts = 0;
        let alias_parts = match alias {
            parser::Alias::WithoutPassed { parts, referenceables: _ } => {
                skip_alias_parts = 1;
                parts
            }
            parser::Alias::WithPassed { extra_parts } => extra_parts,
        };
        let mut value_stream = root_value_actor_receiver
            .flat_map(|actor| actor.subscribe())
            .boxed_local();
        for alias_part in alias_parts.into_iter().skip(skip_alias_parts) {
            let alias_part = alias_part.to_owned();
            value_stream = value_stream
                .flat_map(move |value| match value {
                    Value::Object(object) => object.expect_variable(&alias_part).subscribe(),
                    Value::TaggedObject(tagged_object) => {
                        tagged_object.expect_variable(&alias_part).subscribe()
                    }
                    other => panic!(
                        "Failed to get Object or TaggedObject to create VariableOrArgumentReference: The Value has a different type {}",
                        other.construct_info()
                    ),
                })
                .boxed_local();
        }
        Arc::new(ValueActor::new_internal(
            construct_info,
            actor_context,
            value_stream,
            (),
        ))
    }
}

// --- FunctionCall ---

pub struct FunctionCall {}

impl FunctionCall {
    pub fn new_arc_value_actor<FR: Stream<Item = Value> + 'static>(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        definition: impl Fn(Arc<Vec<Arc<ValueActor>>>, ConstructId, ActorContext) -> FR + 'static,
        arguments: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::FunctionCall);
        let arguments = Arc::new(arguments.into());
        let value_stream = definition(arguments.clone(), construct_info.id(), actor_context.clone());
        Arc::new(ValueActor::new_internal(
            construct_info,
            actor_context,
            value_stream,
            arguments,
        ))
    }
}

// --- LatestCombinator ---

pub struct LatestCombinator {}

impl LatestCombinator {
    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        inputs: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::LatestCombinator);
        let inputs = inputs.into();
        let value_stream =
            stream::select_all(inputs.iter().map(|value_actor| value_actor.subscribe()));
        Arc::new(ValueActor::new_internal(
            construct_info,
            actor_context,
            value_stream,
            inputs,
        ))
    }
}

// --- ThenCombinator ---

pub struct ThenCombinator {}

impl ThenCombinator {
    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        observed: Arc<ValueActor>,
        impulse_sender: mpsc::UnboundedSender<()>,
        body: Arc<ValueActor>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::ThenCombinator);
        let send_impulse_task = Task::start_droppable(observed
            .subscribe()
            .for_each({
                let construct_info = construct_info.clone();
                move |_| { 
                    if let Err(error) = impulse_sender.unbounded_send(()) {
                        eprintln!("Failed to send impulse in {construct_info}: {error:#}")
                    }
                    future::ready(())
                }
            })
        );
        let value_stream = body.subscribe();
        Arc::new(ValueActor::new_internal(
            construct_info,
            actor_context,
            value_stream,
            (observed, send_impulse_task, body),
        ))
    }
}

// --- ValueActor ---

pub struct ValueActor {
    construct_info: Arc<ConstructInfoComplete>,
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
}

impl ValueActor {
    pub fn new(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        value_stream: impl Stream<Item = Value> + 'static,
    ) -> Self {
        let construct_info = construct_info.complete(ConstructType::ValueActor);
        Self::new_internal(construct_info, actor_context, value_stream, ())
    }

    fn new_internal<EOD: 'static>(
        construct_info: ConstructInfoComplete,
        actor_context: ActorContext,
        value_stream: impl Stream<Item = Value> + 'static,
        extra_owned_data: EOD,
    ) -> Self {
        let construct_info = Arc::new(construct_info);
        let (value_sender_sender, mut value_sender_receiver) =
            mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
        let loop_task = Task::start_droppable({
            let construct_info = construct_info.clone();
            let output_valve_signal = actor_context.output_valve_signal;
            async move {
                let output_valve_signal = output_valve_signal;
                let mut output_valve_impulse_stream = if let Some(output_valve_signal) = &output_valve_signal {
                    output_valve_signal.subscribe().left_stream()
                } else {
                    stream::pending().right_stream()
                }.fuse();
                let mut value_stream = pin!(value_stream.fuse());
                let mut value = None;
                let mut value_senders = Vec::<mpsc::UnboundedSender<Value>>::new();
                loop {
                    select! {
                        new_value = value_stream.next() => {
                            let Some(new_value) = new_value else { break };
                            if output_valve_signal.is_none() {
                                value_senders.retain(|value_sender| {
                                    if let Err(error) = value_sender.unbounded_send(new_value.clone()) {
                                        eprintln!("Failed to send new {construct_info} value to subscriber: {error:#}");
                                        false
                                    } else {
                                        true
                                    }
                                });
                            }
                            value = Some(new_value);
                        }
                        value_sender = value_sender_receiver.select_next_some() => {
                            if output_valve_signal.is_none() {
                                if let Some(value) = value.as_ref() {
                                    if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                        eprintln!("Failed to send {construct_info} value to subscriber: {error:#}");
                                    } else {
                                        value_senders.push(value_sender);
                                    }
                                } else {
                                    value_senders.push(value_sender);
                                }
                            } else {
                                value_senders.push(value_sender);
                            }
                        }
                        impulse = output_valve_impulse_stream.next() => {
                            if impulse.is_none() { 
                                break 
                            }
                            if let Some(value) = value.as_ref() {
                                value_senders.retain(|value_sender| {
                                    if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                        eprintln!("Failed to send {construct_info} value to subscriber on impulse: {error:#}");
                                        false
                                    } else {
                                        true
                                    }
                                });
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

    pub fn new_arc(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        value_stream: impl Stream<Item = Value> + 'static,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, actor_context, value_stream))
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
    pub fn construct_info(&self) -> &ConstructInfoComplete {
        match &self {
            Self::Object(object) => &object.construct_info,
            Self::TaggedObject(tagged_object) => &tagged_object.construct_info,
            Self::Text(text) => &text.construct_info,
            Self::Tag(tag) => &tag.construct_info,
            Self::Number(number) => &number.construct_info,
            Self::List(list) => &list.construct_info,
        }
    }

    pub fn expect_object(self) -> Arc<Object> {
        let Self::Object(object) = self else {
            panic!(
                "Failed to get expected Object: The Value has a different type {}",
                self.construct_info()
            )
        };
        object
    }

    pub fn expect_tagged_object(self, tag: &str) -> Arc<TaggedObject> {
        let Self::TaggedObject(tagged_object) = self else {
            panic!("Failed to get expected TaggedObject: The Value has a different type")
        };
        let found_tag = &tagged_object.tag;
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
    variables: Vec<Arc<Variable>>,
}

impl Object {
    pub fn new(
        construct_info: ConstructInfo,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Object),
            variables: variables.into(),
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, variables))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Value {
        Value::Object(Self::new_arc(construct_info, variables))
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, variables))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: object_description,
        } = construct_info;
        let construct_info =
            ConstructInfo::new(actor_id.with_child_id(0), object_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant object wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, variables.into());
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
    }

    pub fn variable(&self, name: &str) -> Option<Arc<Variable>> {
        self.variables
            .iter()
            .position(|variable| variable.name == name)
            .map(|index| self.variables[index].clone())
    }

    pub fn expect_variable(&self, name: &str) -> Arc<Variable> {
        self.variable(name).unwrap_or_else(|| {
            panic!(
                "Failed to get expected Variable '{name}' from {}",
                self.construct_info
            )
        })
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

// --- TaggedObject ---

pub struct TaggedObject {
    construct_info: ConstructInfoComplete,
    tag: Cow<'static, str>,
    variables: Vec<Arc<Variable>>,
}

impl TaggedObject {
    pub fn new(
        construct_info: ConstructInfo,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::TaggedObject),
            tag: tag.into(),
            variables: variables.into(),
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, tag, variables))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Value {
        Value::TaggedObject(Self::new_arc(construct_info, tag, variables))
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, tag, variables))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: tagged_object_description,
        } = construct_info;
        let construct_info =
            ConstructInfo::new(actor_id.with_child_id(0), tagged_object_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Tagged object wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, tag.into(), variables.into());
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
    }

    pub fn variable(&self, name: &str) -> Option<Arc<Variable>> {
        self.variables
            .iter()
            .position(|variable| variable.name == name)
            .map(|index| self.variables[index].clone())
    }

    pub fn expect_variable(&self, name: &str) -> Arc<Variable> {
        self.variable(name).unwrap_or_else(|| {
            panic!(
                "Failed to get expected Variable '{name}' from {}",
                self.construct_info
            )
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
            text: text.into(),
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, text))
    }

    pub fn new_value(construct_info: ConstructInfo, text: impl Into<Cow<'static, str>>) -> Value {
        Value::Text(Self::new_arc(construct_info, text))
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        text: impl Into<Cow<'static, str>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, text))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        text: impl Into<Cow<'static, str>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: text_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), text_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant text wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, text.into());
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
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
            tag: tag.into(),
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, tag: impl Into<Cow<'static, str>>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, tag))
    }

    pub fn new_value(construct_info: ConstructInfo, tag: impl Into<Cow<'static, str>>) -> Value {
        Value::Tag(Self::new_arc(construct_info, tag))
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        tag: impl Into<Cow<'static, str>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, tag))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        tag: impl Into<Cow<'static, str>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: tag_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), tag_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant tag wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, tag.into());
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
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
            number: number.into(),
        }
    }

    pub fn new_arc(construct_info: ConstructInfo, number: impl Into<f64>) -> Arc<Self> {
        Arc::new(Self::new(construct_info, number))
    }

    pub fn new_value(construct_info: ConstructInfo, number: impl Into<f64>) -> Value {
        Value::Number(Self::new_arc(construct_info, number))
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        number: impl Into<f64>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, number))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        number: impl Into<f64>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: number_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), number_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant number wrapper)")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, number.into());
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
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

pub struct List {
    construct_info: Arc<ConstructInfoComplete>,
    loop_task: TaskHandle,
    change_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<ListChange>>,
}

impl List {
    pub fn new(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Self {
        let change_stream = constant(ListChange::Replace {
            items: items.into()
        });
        Self::new_with_change_stream(construct_info, actor_context, change_stream, ())
    }

    pub fn new_with_change_stream<EOD: 'static>(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        change_stream: impl Stream<Item = ListChange> + 'static,
        extra_owned_data: EOD,
    ) -> Self {
        let construct_info = Arc::new(construct_info.complete(ConstructType::List));
        let (change_sender_sender, mut change_sender_receiver) =
            mpsc::unbounded::<mpsc::UnboundedSender<ListChange>>();
        let loop_task = Task::start_droppable({
            let construct_info = construct_info.clone();
            let output_valve_signal = actor_context.output_valve_signal;
            async move {
                let output_valve_signal = output_valve_signal;
                let mut output_valve_impulse_stream = if let Some(output_valve_signal) = &output_valve_signal {
                    output_valve_signal.subscribe().left_stream()
                } else {
                    stream::pending().right_stream()
                }.fuse();
                let mut change_stream = pin!(change_stream.fuse());
                let mut change_senders = Vec::<mpsc::UnboundedSender<ListChange>>::new();
                let mut list = None;
                loop {
                    select! {
                        change = change_stream.next() => {
                            let Some(change) = change else { break };
                            if output_valve_signal.is_none() {
                                change_senders.retain(|change_sender| {
                                    if let Err(error) = change_sender.unbounded_send(change.clone()) {
                                        eprintln!("Failed to send new {construct_info} change to subscriber: {error:#}");
                                        false
                                    } else {
                                        true
                                    }
                                });
                            }
                            if let Some(list) = &mut list {
                                change.clone().apply_to_vec(list);
                            } else {
                                if let ListChange::Replace { items } = &change {
                                    list = Some(items.clone());
                                } else {
                                    panic!("Failed to initialize {construct_info}: The first change has to be 'ListChange::Replace'")
                                }
                            }
                        }
                        change_sender = change_sender_receiver.select_next_some() => {
                            if output_valve_signal.is_none() {
                                if let Some(list) = list.as_ref() {
                                    let first_change_to_send = ListChange::Replace { items: list.clone() };
                                    if let Err(error) = change_sender.unbounded_send(first_change_to_send) {
                                        eprintln!("Failed to send {construct_info} change to subscriber: {error:#}");
                                    } else {
                                        change_senders.push(change_sender);
                                    }
                                } else {
                                    change_senders.push(change_sender);
                                }
                            } else {
                                change_senders.push(change_sender);
                            }
                        }
                        impulse = output_valve_impulse_stream.next() => {
                            if impulse.is_none() { 
                                break 
                            }
                            if let Some(list) = list.as_ref() {
                                change_senders.retain(|change_sender| {
                                    let change_to_send = ListChange::Replace { items: list.clone() };
                                    if let Err(error) = change_sender.unbounded_send(change_to_send) {
                                        eprintln!("Failed to send {construct_info} change to subscriber on impulse: {error:#}");
                                        false
                                    } else {
                                        true
                                    }
                                });
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
            change_sender_sender,
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, actor_context, items))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Value {
        Value::List(Self::new_arc(construct_info, actor_context, items))
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, actor_context, items))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: list_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), list_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant list wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, actor_context.clone(), items.into());
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
    }

    pub fn subscribe(&self) -> impl Stream<Item = ListChange> {
        let (change_sender, change_receiver) = mpsc::unbounded();
        if let Err(error) = self.change_sender_sender.unbounded_send(change_sender) {
            eprintln!("Failed to subscribe to {}: {error:#}", self.construct_info);
        }
        change_receiver
    }
}

impl Drop for List {
    fn drop(&mut self) {
        println!("Dropped: {}", self.construct_info);
    }
}

#[derive(Clone)]
pub enum ListChange {
    Replace { items: Vec<Arc<ValueActor>> },
    InsertAt { index: usize, item: Arc<ValueActor> },
    UpdateAt { index: usize, item: Arc<ValueActor> },
    RemoveAt { index: usize },
    Move { old_index: usize, new_index: usize },
    Push { item: Arc<ValueActor> },
    Pop,
    Clear,
}

impl ListChange {
    pub fn apply_to_vec(self, vec: &mut Vec<Arc<ValueActor>>) {
        match self {
            Self::Replace { items } => {
                *vec = items;
            }
            Self::InsertAt { index, item } => {
                vec.insert(index, item);
            }
            Self::UpdateAt { index, item } => {
                vec[index] = item;
            }
            Self::Push { item } => {
                vec.push(item);
            }
            Self::RemoveAt { index } => {
                vec.remove(index);
            }
            Self::Move {
                old_index,
                new_index,
            } => {
                let item = vec.remove(old_index);
                vec.insert(new_index, item);
            }
            Self::Pop => {
                vec.pop().unwrap();
            }
            Self::Clear => {
                vec.clear();
            }
        }
    }
}
