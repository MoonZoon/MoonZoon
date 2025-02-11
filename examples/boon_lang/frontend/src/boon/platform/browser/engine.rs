// @TODO remove
#![allow(dead_code)]

use std::borrow::Cow;
use std::pin::pin;
use std::sync::Arc;

use zoon::future;
use zoon::futures_channel::mpsc;
use zoon::futures_util::select;
use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::{eprintln, println};
use zoon::{Task, TaskHandle};

// @TODO [] -> impl Into<Vec.. everywhere?


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

// --- Run ---

#[derive(Clone, Copy)]
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
            description: self.description,
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
    VariableReference,
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
    name: &'static str,
    value_actor: Arc<ValueActor>,
    link_value_sender: Option<mpsc::UnboundedSender<Value>>,
}

impl Variable {
    pub fn new(
        construct_info: ConstructInfo,
        name: &'static str,
        value_actor: Arc<ValueActor>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Variable),
            name,
            value_actor,
            link_value_sender: None,
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        name: &'static str,
        value_actor: Arc<ValueActor>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, name, value_actor))
    }

    pub fn new_link_arc(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        name: &'static str,
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
            ValueActor::new_internal(actor_construct_info, run_duration, link_value_receiver, ());
        Arc::new(Self {
            construct_info: construct_info.complete(ConstructType::LinkVariable),
            name,
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

// --- VariableReference ---

pub struct VariableReference {}

impl VariableReference {
    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        alias: &'static str,
        root_variable_receiver: mpsc::UnboundedReceiver<Arc<Variable>>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::VariableReference);
        let variable_names = alias.split('.');
        let mut value_stream = root_variable_receiver
            .flat_map(|variable| variable.subscribe())
            .boxed_local();
        for variable_name in variable_names.skip(1) {
            value_stream = value_stream
                .flat_map(|value| match value {
                    Value::Object(object) => object.expect_variable(variable_name).subscribe(),
                    Value::TaggedObject(tagged_object) => {
                        tagged_object.expect_variable(variable_name).subscribe()
                    }
                    other => panic!(
                        "Failed to get Object or TaggedObject: The Value has a different type {}",
                        other.construct_info()
                    ),
                })
                .boxed_local();
        }
        Arc::new(ValueActor::new_internal(
            construct_info,
            run_duration,
            value_stream,
            (),
        ))
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
        Arc::new(ValueActor::new_internal(
            construct_info,
            run_duration,
            value_stream,
            arguments,
        ))
    }
}

// --- LatestCombinator ---

pub struct LatestCombinator {}

impl LatestCombinator {
    pub fn new_arc_value_actor<const IN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        inputs: [Arc<ValueActor>; IN],
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::LatestCombinator);
        let value_stream =
            stream::select_all(inputs.iter().map(|value_actor| value_actor.subscribe()));
        Arc::new(ValueActor::new_internal(
            construct_info,
            run_duration,
            value_stream,
            inputs,
        ))
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
        Arc::new(ValueActor::new_internal(
            construct_info,
            run_duration,
            value_stream,
            observed,
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
        run_duration: RunDuration,
        value_stream: impl Stream<Item = Value> + 'static,
    ) -> Self {
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
        let (value_sender_sender, mut value_sender_receiver) =
            mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
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

    pub fn new_arc(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        value_stream: impl Stream<Item = Value> + 'static,
    ) -> Arc<Self> {
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

    pub fn new_value<const VN: usize>(
        construct_info: ConstructInfo,
        variables: [Arc<Variable>; VN],
    ) -> Value {
        Value::Object(Self::new_arc(construct_info, variables))
    }

    pub fn new_constant<const VN: usize>(
        construct_info: ConstructInfo,
        variables: [Arc<Variable>; VN],
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, variables))
    }

    pub fn new_arc_value_actor<const VN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        variables: [Arc<Variable>; VN],
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: tagged_object_description,
        } = construct_info;
        let construct_info =
            ConstructInfo::new(actor_id.with_child_id(0), tagged_object_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant object wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, variables);
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            run_duration,
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

// --- Object ---

pub struct TaggedObject {
    construct_info: ConstructInfoComplete,
    tag: &'static str,
    variables: Box<[Arc<Variable>]>,
}

impl TaggedObject {
    pub fn new<const VN: usize>(
        construct_info: ConstructInfo,
        tag: &'static str,
        variables: [Arc<Variable>; VN],
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::TaggedObject),
            tag,
            variables: Box::new(variables),
        }
    }

    pub fn new_arc<const VN: usize>(
        construct_info: ConstructInfo,
        tag: &'static str,
        variables: [Arc<Variable>; VN],
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, tag, variables))
    }

    pub fn new_value<const VN: usize>(
        construct_info: ConstructInfo,
        tag: &'static str,
        variables: [Arc<Variable>; VN],
    ) -> Value {
        Value::TaggedObject(Self::new_arc(construct_info, tag, variables))
    }

    pub fn new_constant<const VN: usize>(
        construct_info: ConstructInfo,
        tag: &'static str,
        variables: [Arc<Variable>; VN],
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, tag, variables))
    }

    pub fn new_arc_value_actor<const VN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        tag: &'static str,
        variables: [Arc<Variable>; VN],
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: tagged_object_description,
        } = construct_info;
        let construct_info =
            ConstructInfo::new(actor_id.with_child_id(0), tagged_object_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Tagged object wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, tag, variables);
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            run_duration,
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
        run_duration: RunDuration,
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
            run_duration,
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
        run_duration: RunDuration,
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
            run_duration,
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
        run_duration: RunDuration,
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
            run_duration,
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
    pub fn new<const IN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        items: [Arc<ValueActor>; IN],
    ) -> Self {
        let change_stream = constant(ListChange::Replace {
            items: Vec::from(items),
        });
        Self::new_with_change_stream(construct_info, run_duration, change_stream, ())
    }

    pub fn new_with_change_stream<EOD: 'static>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        change_stream: impl Stream<Item = ListChange> + 'static,
        extra_owned_data: EOD,
    ) -> Self {
        let construct_info = Arc::new(construct_info.complete(ConstructType::List));
        let (change_sender_sender, mut change_sender_receiver) =
            mpsc::unbounded::<mpsc::UnboundedSender<ListChange>>();
        let loop_task = Task::start_droppable({
            let construct_info = construct_info.clone();
            async move {
                let mut change_stream = pin!(change_stream.fuse());
                let mut change_senders = Vec::<mpsc::UnboundedSender<ListChange>>::new();
                let mut list = None;
                loop {
                    select! {
                        change = change_stream.next() => {
                            let Some(change) = change else { break };
                            change_senders.retain(|change_sender| {
                                if let Err(error) = change_sender.unbounded_send(change.clone()) {
                                    eprintln!("Failed to send new {} change to subscriber: {error:#}", construct_info);
                                    false
                                } else {
                                    true
                                }
                            });
                            if let Some(list) = &mut list {
                                change.clone().apply_to_vec(list);
                            } else {
                                if let ListChange::Replace { items } = &change {
                                    list = Some(items.clone());
                                } else {
                                    panic!("Failed to initialize {}: The first change has to be 'ListChange::Replace'", construct_info)
                                }
                            }
                            if let RunDuration::UntilFirstValue = run_duration {
                                break
                            }
                        }
                        change_sender = change_sender_receiver.select_next_some() => {
                            if let Some(list) = list.as_ref() {
                                let first_change_to_send = ListChange::Replace { items: list.clone() };
                                if let Err(error) = change_sender.unbounded_send(first_change_to_send) {
                                    eprintln!("Failed to send {} change to subscriber: {error:#}", construct_info);
                                } else {
                                    change_senders.push(change_sender);
                                }
                            } else {
                                change_senders.push(change_sender);
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

    pub fn new_arc<const IN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        items: [Arc<ValueActor>; IN],
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, run_duration, items))
    }

    pub fn new_value<const IN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        items: [Arc<ValueActor>; IN],
    ) -> Value {
        Value::List(Self::new_arc(construct_info, run_duration, items))
    }

    pub fn new_constant<const IN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        items: [Arc<ValueActor>; IN],
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(construct_info, run_duration, items))
    }

    pub fn new_arc_value_actor<const IN: usize>(
        construct_info: ConstructInfo,
        run_duration: RunDuration,
        items: [Arc<ValueActor>; IN],
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            description: list_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(actor_id.with_child_id(0), list_description);
        let actor_construct_info = ConstructInfo::new(actor_id, "Constant list wrapper")
            .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(construct_info, run_duration, items);
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            run_duration,
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
