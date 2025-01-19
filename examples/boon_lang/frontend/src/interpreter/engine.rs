use std::pin::{Pin, pin};

use zoon::futures_channel::{oneshot, mpsc};
use zoon::futures_util::stream::{self, Stream, StreamExt, BoxStream};
use zoon::{Task, TaskHandle};
use zoon::future;
use zoon::eprintln;
use zoon::futures_util::select;

use pin_project::pin_project;

pub fn stream_one<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item))
}

// --- ConstructId ---

#[derive(Clone)]
pub struct ConstructId(Vec<u64>);

impl ConstructId {
    pub fn push_child_id(&self, child: u64) -> Self {
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
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    name: &'static str,
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
}

impl Variable {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        name: &'static str,
        value_stream: impl Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static,
    ) -> Self {
        let value_stream = value_stream.map(Into::into);
        let (loop_task, value_sender_sender) = Self::loop_task_and_value_sender_sender(value_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            name,
            loop_task,
            value_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = Value> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to send 'value_sender' through `value_sender_sender`: {error:#}");
        }
        value_receiver
    }

    fn loop_task_and_value_sender_sender(value_stream: impl Stream<Item = Value>) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>) {
        let (value_sender_sender, value_sender_receiver) = mpsc::unbounded();
        let loop_task = Task::start_droppable(async move {
            let mut value_senders = Vec::<mpsc::UnboundedSender<Value>>::new();
            let mut value = None::<Value>;
            let value_stream = pin!(value_stream.fuse());
            loop {
                select! {
                    new_value = value_stream.next() => {
                        let Some(new_value) = new_value else { break };
                        value_senders.retain(|value_sender| {
                            if let Err(error) = value_sender.unbounded_send(new_value.clone()) {
                                eprintln!("Failed to send new Variable value through `value_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        value = Some(new_value);
                    }
                    value_sender = value_sender_receiver.select_next_some() => {
                        if let Some(value) = value.cloned() {
                            if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                eprintln!("Failed to send Variable value through new `value_sender`: {error:#}");
                            } else {
                                value_senders.push(value_sender);
                            }
                        } else {
                            value_senders.push(value_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, value_sender_sender)
    }
}

impl Clone for Variable {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, value_sender_sender) = Self::loop_task_and_value_sender_sender(value_stream);
        Variable { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            name: self.name,
            loop_task,
            value_sender_sender 
        }
    }
}

impl Stream for Variable {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- VariableReference ---

#[pin_project]
pub struct VariableReference {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    alias: &'static str,
    #[pin]
    value_stream: BoxStream<'static, Value>,
}

impl VariableReference {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        alias: &'static str,
        variable_receiver: oneshot::Receiver<Variable>, 
    ) -> Self {
        Self {
            description,
            id: id.into(),
            alias,
            value_stream: stream::once(variable_receiver)
                .filter_map(|variable_result| {
                    match variable_result {
                        Ok(variable) => future::ready(Some(variable)),
                        Err(error) => {
                            eprintln!("Failed to get Variable in VariableReference: {error:#}");
                            future::ready(None)
                        }
                    }
                })
                .flatten()
                .boxed(),
        }
    }
}

impl Stream for VariableReference {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().value_stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.value_stream.size_hint()
    }
}

// --- Value ---

pub enum Value {
    ObjectValue(ObjectValue),
    TaggedObjectValue(TaggedObjectValue),
    NumberValue(NumberValue),
    TextValue(TextValue),
    TagValue(TagValue),
    LinkValue(LinkValue),
    ListValue(ListValue),
}

impl Value {
    pub fn expect_object_value(self) -> ObjectValue {
        if let Self::ObjectValue(object_value) = self {
            object_value
        } else {
            unreachable!("ObjectValue expected")
        }
    }

    // @TODO remove?
    #[allow(dead_code)]
    pub fn expect_tagged_object_value(self) -> TaggedObjectValue {
        if let Self::TaggedObjectValue(tagged_object_value) = self {
            tagged_object_value
        } else {
            unreachable!("TaggedObjectValue expected")
        }
    }

    pub fn expect_number_value(self) -> NumberValue {
        if let Self::NumberValue(number_value) = self {
            number_value
        } else {
            unreachable!("NumberValue expected")
        }
    }

    pub fn expect_text_value(self) -> TextValue {
        if let Self::TextValue(text_value) = self {
            text_value
        } else {
            unreachable!("TextValue expected")
        }
    }

    pub fn expect_tag_value(self) -> TagValue {
        if let Self::TagValue(tag_value) = self {
            tag_value
        } else {
            unreachable!("TagValue expected")
        }
    }

    pub fn expect_link_value(self) -> LinkValue {
        if let Self::LinkValue(link_value) = self {
            link_value
        } else {
            unreachable!("LinkValue expected")
        }
    }

    pub fn expect_list_value(self) -> ListValue {
        if let Self::ListValue(list_value) = self {
            list_value
        } else {
            unreachable!("ListValue expected")
        }
    }
}

impl From<ObjectValue> for Value {
    fn from(value: ObjectValue) -> Self {
        Self::ObjectValue(value)
    }
}

impl From<TaggedObjectValue> for Value {
    fn from(value: TaggedObjectValue) -> Self {
        Self::TaggedObjectValue(value)
    }
}

impl From<NumberValue> for Value {
    fn from(value: NumberValue) -> Self {
        Self::NumberValue(value)
    }
}

impl From<TextValue> for Value {
    fn from(value: TextValue) -> Self {
        Self::TextValue(value)
    }
}

impl From<TagValue> for Value {
    fn from(value: TagValue) -> Self {
        Self::TagValue(value)
    }
}

impl From<LinkValue> for Value {
    fn from(value: LinkValue) -> Self {
        Self::LinkValue(value)
    }
}

impl From<ListValue> for Value {
    fn from(value: ListValue) -> Self {
        Self::ListValue(value)
    }
}

// --- ObjectValue ---

pub struct ObjectValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    object_stream: BoxStream<'static, Object>,
}

impl ObjectValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        object_stream: impl Stream<Item = Object> + Send + 'static,
    ) -> Self {
        Self {
            description,
            id: id.into(),
            object_stream: object_stream.boxed(),
        }
    }

    pub fn object_stream(self) -> impl Stream<Item = Object> + Send + 'static {
        self.object_stream
    }
}

// --- TaggedObjectValue ---

pub struct TaggedObjectValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    tagged_object_stream: BoxStream<'static, (&'static str, Object)>,
}

impl TaggedObjectValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        tagged_object_stream: impl Stream<Item = (&'static str, Object)> + Send + 'static
    ) -> Self {
        Self {
            description,
            id: id.into(),
            tagged_object_stream: tagged_object_stream.boxed(),
        }
    }

    pub fn tagged_object_stream(self) -> impl Stream<Item = (&'static str, Object)> + Send + 'static {
        self.tagged_object_stream
    }
}

// --- NumberValue ---

pub struct NumberValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    number_stream: BoxStream<'static, f64>,
}

impl NumberValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        number_stream: impl Stream<Item = f64> + Send + 'static,
    ) -> Self {
        Self {
            description,
            id: id.into(),
            number_stream: number_stream.boxed(),
        }
    }

    pub fn number_stream(self) -> impl Stream<Item = f64> + Send + 'static {
        self.number_stream
    }
}

// --- TextValue ---

pub struct TextValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    text_stream: BoxStream<'static, String>,
}

impl TextValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        text_stream: impl Stream<Item = String> + Send + 'static,
    ) -> Self {
        Self {
            description,
            id: id.into(),
            text_stream: text_stream.boxed(),
        }
    }

    pub fn text_stream(self) -> impl Stream<Item = String> + Send + 'static {
        self.text_stream
    }
}

// --- ListValue ---

pub struct ListValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    list_stream: BoxStream<'static, List>,
}

impl ListValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        list_stream: impl Stream<Item = List> + Send + 'static,
    ) -> Self {
        Self {
            description,
            id: id.into(),
            list_stream: list_stream.boxed(),
        }
    }

    pub fn list_stream(self) -> impl Stream<Item = List> + Send + 'static {
        self.list_stream
    }
}

// --- LinkValue ---

#[pin_project]
pub struct LinkValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[pin]
    value_stream: BoxStream<'static, Value>,
    stream_sender: mpsc::UnboundedSender<BoxStream<'static, Value>>
}

impl LinkValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
    ) -> Self {
        let (stream_sender, stream_recevier) = mpsc::unbounded();
        Self {
            description,
            id: id.into(),
            value_stream: stream_recevier.flatten().boxed(),
            stream_sender
        }
    }

    pub fn send_new_value_stream(&self, value_stream: impl Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static) {
        let value_stream = value_stream.map(Into::into).boxed();
        if let Err(error) = self.stream_sender.unbounded_send(value_stream) {
            eprintln!("Failed to send new link value stream: {error:#}")
        }
    }
}

impl Stream for LinkValue {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().value_stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.value_stream.size_hint()
    }
}

// --- TagValue ---

pub struct TagValue {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    tag_stream: BoxStream<'static, String>,
}

impl TagValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        tag_stream: impl Stream<Item = String> + Send + 'static
    ) -> Self {
        Self {
            description,
            id: id.into(),
            tag_stream: tag_stream.boxed(),
        }
    }

    pub fn tag_stream(self) -> impl Stream<Item = String> + Send + 'static {
        self.tag_stream
    }
}

// --- FunctionCall ---

#[pin_project]
pub struct FunctionCall {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    name: &'static str,
    #[pin]
    value_stream: BoxStream<'static, Value>,
}

impl FunctionCall {
    pub fn new<FR: Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static>(
        description: &'static str, 
        id: impl Into<ConstructId>,
        name: &'static str,
        definition: impl Fn(Object, ConstructId) -> FR + 'static,
        arguments: Object
    ) -> Self {
        let id = id.into();
        Self {
            description,
            id: id.clone(),
            name,
            value_stream: definition(arguments, id).map(Into::into).boxed()
        }
    }
}

impl Stream for FunctionCall {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().value_stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.value_stream.size_hint()
    }
}

// --- Object ---

pub struct Object {
    variables: Vec<Variable>,
}

impl Object {
    pub fn new<const N: usize>(variables: [Variable; N]) -> Self {
        Self { 
            variables: Vec::from(variables)
        }
    }

    pub fn take_variable(&mut self, name: &str) -> Option<Variable> {
        let index = self.variables.iter().position(|variable| { 
            variable.name == name
        });
        if let Some(index) = index {
            Some(self.variables.swap_remove(index))
        } else {
            None
        }
    }

    pub fn take_expected_variable(&mut self, name: &str) -> Variable {
        self
            .take_variable(name)
            .expect("failed to get expect variable '{name}'")
    }
}

// --- FixedList ---

pub struct FixedList {
    items: Vec<BoxStream<'static, Value>>,
}

impl FixedList {
    pub fn new<const N: usize>(items: [BoxStream<'static, Value>; N]) -> Self {
        Self { 
            items: Vec::from(items)
        }
    }
}

impl IntoIterator for FixedList {
    type Item = BoxStream<'static, Value>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

// --- List ---

pub struct List {
    items: Vec<BoxStream<'static, Value>>,
    change_sender: mpsc::UnboundedSender<ListChange>,
    change_stream: BoxStream<'static, ListChange>,
}

impl List {
    pub fn new<const N: usize>(items: [BoxStream<'static, Value>; N]) -> Self {
        let (change_sender, change_receiver) = mpsc::unbounded();
        Self { 
            items: Vec::from(items),
            change_sender,
            change_stream: change_receiver.boxed(),
        }
    }

    pub fn change_stream(self) -> impl Stream<Item = ListChange> + Send + 'static {
        self.change_stream
    }
}

// @TODO remove
#[allow(dead_code)]
pub enum ListChange {
    Replace {
        values: Vec<BoxStream<'static, Value>>,
    },
    InsertAt {
        index: usize,
        value: BoxStream<'static, Value>,
    },
    UpdateAt {
        index: usize,
        value: BoxStream<'static, Value>,
    },
    RemoveAt {
        index: usize,
    },
    Move {
        old_index: usize,
        new_index: usize,
    },
    Push {
        value: BoxStream<'static, Value>,
    },
    Pop,
    Clear,
}

// --- LatestCombinator ---

#[pin_project]
pub struct LatestCombinator {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[pin]
    value_stream: BoxStream<'static, Value>,
}

impl LatestCombinator {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        value_streams: FixedList,
    ) -> Self {
        Self {
            description,
            id: id.into(),
            value_stream: stream::select_all(value_streams).boxed()
        }
    }
}

impl Stream for LatestCombinator {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().value_stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.value_stream.size_hint()
    }
}

// --- ThenCombinator ---

#[pin_project]
pub struct ThenCombinator {
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[pin]
    value_stream: BoxStream<'static, Value>,
}

impl ThenCombinator {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        observed_value_stream: impl Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static,
        input_value_stream: impl Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static,
    ) -> Self {
        Self {
            description,
            id: id.into(),
            value_stream: 
                // @TODO cache latest values (everywhere, just like lists have to work)
                observed_value_stream
                    .map(|_|())
                    .zip(input_value_stream.map(Into::into))
                    .map(|(_, input_value)| input_value)
                    .boxed(),
        }
    }
}

impl Stream for ThenCombinator {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        self.project().value_stream.poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.value_stream.size_hint()
    }
}
