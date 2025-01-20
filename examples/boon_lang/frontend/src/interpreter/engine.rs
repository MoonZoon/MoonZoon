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

#[derive(Clone, Debug)]
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
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    name: &'static str,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
    output_value_stream: Option<CloneableValueStream>,
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
            output_value_stream: None,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = Value> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to send Variable 'value_sender' through `value_sender_sender`: {error:#}");
        }
        value_receiver
    }

    fn loop_task_and_value_sender_sender(value_stream: impl Stream<Item = Value> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>) {
        let (value_sender_sender, mut value_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
        let loop_task = Task::start_droppable(async move {
            let mut value_senders = Vec::<mpsc::UnboundedSender<Value>>::new();
            let mut value = None::<Value>;
            let mut value_stream = pin!(value_stream.fuse());
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
                        if let Some(value) = value.clone() {
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
            value_sender_sender,
            output_value_stream: None, 
        }
    }
}

impl Stream for Variable {
    type Item = Value;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        if self.output_value_stream.is_none() {
            let stream = CloneableValueStream::new(self.subscribe());
            self.output_value_stream = Some(stream);

        }
        pin!(self.output_value_stream.as_mut().unwrap()).poll_next(cx)
    }
}

impl Drop for Variable {
    fn drop(&mut self) {
        zoon::println!("Variable dropped!");
        zoon::println!("Id: {:?}", self.id);
        zoon::println!("Description: {}", self.description);
        zoon::println!("Name: {}", self.name);
        zoon::println!("Clone: {:?}", self.is_clone);
        zoon::println!("______");
    }
}

// --- CloneableValueStream ---

pub struct CloneableValueStream {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
}

impl CloneableValueStream {
    pub fn new(
        value_stream: impl Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static,
    ) -> Self {
        let value_stream = value_stream.map(Into::into);
        let (loop_task, value_sender_sender) = Self::loop_task_and_value_sender_sender(value_stream);
        Self {
            is_clone: false,
            loop_task,
            value_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = Value> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to send CloneableValueStream 'value_sender' through `value_sender_sender`: {error:#}");
        }
        value_receiver
    }

    fn loop_task_and_value_sender_sender(value_stream: impl Stream<Item = Value> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>) {
        let (value_sender_sender, mut value_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
        let loop_task = Task::start_droppable(async move {
            let mut value_senders = Vec::<mpsc::UnboundedSender<Value>>::new();
            let mut value = None::<Value>;
            let mut value_stream = pin!(value_stream.fuse());
            loop {
                select! {
                    new_value = value_stream.next() => {
                        let Some(new_value) = new_value else { break };
                        value_senders.retain(|value_sender| {
                            if let Err(error) = value_sender.unbounded_send(new_value.clone()) {
                                eprintln!("Failed to send new CloneableValueStream value through `value_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        value = Some(new_value);
                    }
                    value_sender = value_sender_receiver.select_next_some() => {
                        if let Some(value) = value.clone() {
                            if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                eprintln!("Failed to send CloneableValueStream value through new `value_sender`: {error:#}");
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

impl Clone for CloneableValueStream {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, value_sender_sender) = Self::loop_task_and_value_sender_sender(value_stream);
        CloneableValueStream { 
            is_clone: true,
            loop_task,
            value_sender_sender 
        }
    }
}

impl Stream for CloneableValueStream {
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

#[derive(Clone)]
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
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    object_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Object>>,
}

impl ObjectValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        object_stream: impl Stream<Item = Object> + Send + 'static,
    ) -> Self {
        let (loop_task, object_sender_sender) = Self::loop_task_and_object_sender_sender(object_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            object_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = Object> {
        let (object_sender, object_receiver) = mpsc::unbounded();
        if let Err(error) = self.object_sender_sender.unbounded_send(object_sender) {
            eprintln!("Failed to send 'object_sender' through `object_sender_sender`: {error:#}");
        }
        object_receiver
    }

    fn loop_task_and_object_sender_sender(object_stream: impl Stream<Item = Object> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<Object>>) {
        let (object_sender_sender, mut object_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<Object>>();
        let loop_task = Task::start_droppable(async move {
            let mut object_senders = Vec::<mpsc::UnboundedSender<Object>>::new();
            let mut object = None::<Object>;
            let mut object_stream = pin!(object_stream.fuse());
            loop {
                select! {
                    new_object = object_stream.next() => {
                        let Some(new_object) = new_object else { break };
                        object_senders.retain(|object_sender| {
                            if let Err(error) = object_sender.unbounded_send(new_object.clone()) {
                                eprintln!("Failed to send new ObjectValue object through `object_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        object = Some(new_object);
                    }
                    object_sender = object_sender_receiver.select_next_some() => {
                        if let Some(object) = object.clone() {
                            if let Err(error) = object_sender.unbounded_send(object.clone()) {
                                eprintln!("Failed to send ObjectValue object through new `object_sender`: {error:#}");
                            } else {
                                object_senders.push(object_sender);
                            }
                        } else {
                            object_senders.push(object_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, object_sender_sender)
    }
}

impl Clone for ObjectValue {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, object_sender_sender) = Self::loop_task_and_object_sender_sender(value_stream);
        ObjectValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            object_sender_sender 
        }
    }
}

impl Stream for ObjectValue {
    type Item = Object;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- TaggedObjectValue ---

pub struct TaggedObjectValue {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    tagged_object_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<(&'static str, Object)>>,
}

impl TaggedObjectValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        tagged_object_stream: impl Stream<Item = (&'static str, Object)> + Send + 'static
    ) -> Self {
        let (loop_task, tagged_object_sender_sender) = Self::loop_task_and_tagged_object_sender_sender(tagged_object_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            tagged_object_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = (&'static str, Object)> {
        let (tagged_object_sender, tagged_object_receiver) = mpsc::unbounded();
        if let Err(error) = self.tagged_object_sender_sender.unbounded_send(tagged_object_sender) {
            eprintln!("Failed to send 'tagged_object_sender' through `tagged_object_sender_sender`: {error:#}");
        }
        tagged_object_receiver
    }

    fn loop_task_and_tagged_object_sender_sender(tagged_object_stream: impl Stream<Item = (&'static str, Object)> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<(&'static str, Object)>>) {
        let (tagged_object_sender_sender, mut tagged_object_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<(&'static str, Object)>>();
        let loop_task = Task::start_droppable(async move {
            let mut tagged_object_senders = Vec::<mpsc::UnboundedSender<(&'static str, Object)>>::new();
            let mut tagged_object = None::<(&'static str, Object)>;
            let mut tagged_object_stream = pin!(tagged_object_stream.fuse());
            loop {
                select! {
                    new_tagged_object = tagged_object_stream.next() => {
                        let Some(new_tagged_object) = new_tagged_object else { break };
                        tagged_object_senders.retain(|tagged_object_sender| {
                            if let Err(error) = tagged_object_sender.unbounded_send(new_tagged_object.clone()) {
                                eprintln!("Failed to send new TaggedObjectValue tagged_object through `tagged_object_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        tagged_object = Some(new_tagged_object);
                    }
                    tagged_object_sender = tagged_object_sender_receiver.select_next_some() => {
                        if let Some(tagged_object) = tagged_object.clone() {
                            if let Err(error) = tagged_object_sender.unbounded_send(tagged_object.clone()) {
                                eprintln!("Failed to send TaggedObjectValue tagged_object through new `tagged_object_sender`: {error:#}");
                            } else {
                                tagged_object_senders.push(tagged_object_sender);
                            }
                        } else {
                            tagged_object_senders.push(tagged_object_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, tagged_object_sender_sender)
    }
}

impl Clone for TaggedObjectValue {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, tagged_object_sender_sender) = Self::loop_task_and_tagged_object_sender_sender(value_stream);
        TaggedObjectValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            tagged_object_sender_sender 
        }
    }
}

impl Stream for TaggedObjectValue {
    type Item = (&'static str, Object);

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- NumberValue ---

pub struct NumberValue {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    number_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<f64>>,
}

impl NumberValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        number_stream: impl Stream<Item = f64> + Send + 'static,
    ) -> Self {
        let (loop_task, number_sender_sender) = Self::loop_task_and_number_sender_sender(number_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            number_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = f64> {
        let (number_sender, number_receiver) = mpsc::unbounded();
        if let Err(error) = self.number_sender_sender.unbounded_send(number_sender) {
            eprintln!("Failed to send 'number_sender' through `number_sender_sender`: {error:#}");
        }
        number_receiver
    }

    fn loop_task_and_number_sender_sender(number_stream: impl Stream<Item = f64> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<f64>>) {
        let (number_sender_sender, mut number_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<f64>>();
        let loop_task = Task::start_droppable(async move {
            let mut number_senders = Vec::<mpsc::UnboundedSender<f64>>::new();
            let mut number = None::<f64>;
            let mut number_stream = pin!(number_stream.fuse());
            loop {
                select! {
                    new_number = number_stream.next() => {
                        let Some(new_number) = new_number else { break };
                        number_senders.retain(|number_sender| {
                            if let Err(error) = number_sender.unbounded_send(new_number.clone()) {
                                eprintln!("Failed to send new NumberValue number through `number_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        number = Some(new_number);
                    }
                    number_sender = number_sender_receiver.select_next_some() => {
                        if let Some(number) = number.clone() {
                            if let Err(error) = number_sender.unbounded_send(number.clone()) {
                                eprintln!("Failed to send NumberValue number through new `number_sender`: {error:#}");
                            } else {
                                number_senders.push(number_sender);
                            }
                        } else {
                            number_senders.push(number_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, number_sender_sender)
    }
}

impl Clone for NumberValue {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, number_sender_sender) = Self::loop_task_and_number_sender_sender(value_stream);
        NumberValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            number_sender_sender 
        }
    }
}

impl Stream for NumberValue {
    type Item = f64;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- TextValue ---

pub struct TextValue {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    text_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<String>>,
}

impl TextValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        text_stream: impl Stream<Item = String> + Send + 'static,
    ) -> Self {
        let (loop_task, text_sender_sender) = Self::loop_task_and_text_sender_sender(text_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            text_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = String> {
        let (text_sender, text_receiver) = mpsc::unbounded();
        if let Err(error) = self.text_sender_sender.unbounded_send(text_sender) {
            eprintln!("Failed to send 'text_sender' through `text_sender_sender`: {error:#}");
        }
        text_receiver
    }

    fn loop_task_and_text_sender_sender(text_stream: impl Stream<Item = String> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<String>>) {
        let (text_sender_sender, mut text_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<String>>();
        let loop_task = Task::start_droppable(async move {
            let mut text_senders = Vec::<mpsc::UnboundedSender<String>>::new();
            let mut text = None::<String>;
            let mut text_stream = pin!(text_stream.fuse());
            loop {
                select! {
                    new_text = text_stream.next() => {
                        let Some(new_text) = new_text else { break };
                        text_senders.retain(|text_sender| {
                            if let Err(error) = text_sender.unbounded_send(new_text.clone()) {
                                eprintln!("Failed to send new TextValue text through `text_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        text = Some(new_text);
                    }
                    text_sender = text_sender_receiver.select_next_some() => {
                        if let Some(text) = text.clone() {
                            if let Err(error) = text_sender.unbounded_send(text.clone()) {
                                eprintln!("Failed to send TextValue text through new `text_sender`: {error:#}");
                            } else {
                                text_senders.push(text_sender);
                            }
                        } else {
                            text_senders.push(text_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, text_sender_sender)
    }
}

impl Clone for TextValue {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, text_sender_sender) = Self::loop_task_and_text_sender_sender(value_stream);
        TextValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            text_sender_sender 
        }
    }
}

impl Stream for TextValue {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- ListValue ---

pub struct ListValue {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    list_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<List>>,
}

impl ListValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        list_stream: impl Stream<Item = List> + Send + 'static,
    ) -> Self {
        let (loop_task, list_sender_sender) = Self::loop_task_and_list_sender_sender(list_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            list_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = List> {
        let (list_sender, list_receiver) = mpsc::unbounded();
        if let Err(error) = self.list_sender_sender.unbounded_send(list_sender) {
            eprintln!("Failed to send 'list_sender' through `list_sender_sender`: {error:#}");
        }
        list_receiver
    }

    fn loop_task_and_list_sender_sender(list_stream: impl Stream<Item = List> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<List>>) {
        let (list_sender_sender, mut list_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<List>>();
        let loop_task = Task::start_droppable(async move {
            let mut list_senders = Vec::<mpsc::UnboundedSender<List>>::new();
            let mut list = None::<List>;
            let mut list_stream = pin!(list_stream.fuse());
            loop {
                select! {
                    new_list = list_stream.next() => {
                        let Some(new_list) = new_list else { break };
                        list_senders.retain(|list_sender| {
                            if let Err(error) = list_sender.unbounded_send(new_list.clone()) {
                                eprintln!("Failed to send new ListValue list through `list_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        list = Some(new_list);
                    }
                    list_sender = list_sender_receiver.select_next_some() => {
                        if let Some(list) = list.clone() {
                            if let Err(error) = list_sender.unbounded_send(list.clone()) {
                                eprintln!("Failed to send ListValue list through new `list_sender`: {error:#}");
                            } else {
                                list_senders.push(list_sender);
                            }
                        } else {
                            list_senders.push(list_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, list_sender_sender)
    }
}

impl Clone for ListValue {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, list_sender_sender) = Self::loop_task_and_list_sender_sender(value_stream);
        ListValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            list_sender_sender 
        }
    }
}

impl Stream for ListValue {
    type Item = List;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- LinkValue ---

#[pin_project]
pub struct LinkValue {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    value_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>,
    value_stream_sender: mpsc::UnboundedSender<BoxStream<'static, Value>>,
}

impl LinkValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
    ) -> Self {
        let (value_stream_sender, value_stream_receiver) = mpsc::unbounded::<BoxStream<'static, Value>>();
        let value_stream = value_stream_receiver.flatten();
        let (loop_task, value_sender_sender) = Self::loop_task_and_value_sender_sender(value_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            value_sender_sender,
            value_stream_sender
        }
    }

    pub fn send_new_value_stream(&self, value_stream: impl Stream<Item = impl Into<Value> + Send + 'static> + Send + 'static) {
        let value_stream = value_stream.map(Into::into).boxed();
        if let Err(error) = self.value_stream_sender.unbounded_send(value_stream) {
            eprintln!("Failed to send new link value stream: {error:#}")
        }
    }

    fn subscribe(&self) -> impl Stream<Item = Value> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to send LinkValue 'value_sender' through `value_sender_sender`: {error:#}");
        }
        value_receiver
    }

    fn loop_task_and_value_sender_sender(value_stream: impl Stream<Item = Value> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<Value>>) {
        let (value_sender_sender, mut value_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<Value>>();
        let loop_task = Task::start_droppable(async move {
            let mut value_senders = Vec::<mpsc::UnboundedSender<Value>>::new();
            let mut value = None::<Value>;
            let mut value_stream = pin!(value_stream.fuse());
            loop {
                select! {
                    new_value = value_stream.next() => {
                        let Some(new_value) = new_value else { break };
                        value_senders.retain(|value_sender| {
                            if let Err(error) = value_sender.unbounded_send(new_value.clone()) {
                                eprintln!("Failed to send new LinkValue value through `value_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        value = Some(new_value);
                    }
                    value_sender = value_sender_receiver.select_next_some() => {
                        if let Some(value) = value.clone() {
                            if let Err(error) = value_sender.unbounded_send(value.clone()) {
                                eprintln!("Failed to send LinkValue value through new `value_sender`: {error:#}");
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

impl Clone for LinkValue {
    fn clone(&self) -> Self {
        let (value_stream_sender, value_stream_receiver) = mpsc::unbounded::<BoxStream<'static, Value>>();
        let value_stream = value_stream_receiver.flatten();
        value_stream_sender.unbounded_send(self.subscribe().boxed()).unwrap();
        let (loop_task, value_sender_sender) = Self::loop_task_and_value_sender_sender(value_stream);
        LinkValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            value_sender_sender ,
            value_stream_sender
        }
    }
}

impl Stream for LinkValue {
    type Item = Value;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// --- TagValue ---

pub struct TagValue {
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    // @TODO remove
    #[allow(dead_code)]
    description: &'static str,
    // @TODO remove
    #[allow(dead_code)]
    id: ConstructId,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    tag_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<String>>,
}

impl TagValue {
    pub fn new(
        description: &'static str, 
        id: impl Into<ConstructId>,
        tag_stream: impl Stream<Item = String> + Send + 'static
    ) -> Self {
        let (loop_task, tag_sender_sender) = Self::loop_task_and_tag_sender_sender(tag_stream);
        Self {
            is_clone: false,
            description,
            id: id.into(),
            loop_task,
            tag_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = String> {
        let (tag_sender, tag_receiver) = mpsc::unbounded();
        if let Err(error) = self.tag_sender_sender.unbounded_send(tag_sender) {
            eprintln!("Failed to send 'tag_sender' through `tag_sender_sender`: {error:#}");
        }
        tag_receiver
    }

    fn loop_task_and_tag_sender_sender(tag_stream: impl Stream<Item = String> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<String>>) {
        let (tag_sender_sender, mut tag_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<String>>();
        let loop_task = Task::start_droppable(async move {
            let mut tag_senders = Vec::<mpsc::UnboundedSender<String>>::new();
            let mut tag = None::<String>;
            let mut tag_stream = pin!(tag_stream.fuse());
            loop {
                select! {
                    new_tag = tag_stream.next() => {
                        let Some(new_tag) = new_tag else { break };
                        tag_senders.retain(|tag_sender| {
                            if let Err(error) = tag_sender.unbounded_send(new_tag.clone()) {
                                eprintln!("Failed to send new TagValue tag through `tag_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        tag = Some(new_tag);
                    }
                    tag_sender = tag_sender_receiver.select_next_some() => {
                        if let Some(tag) = tag.clone() {
                            if let Err(error) = tag_sender.unbounded_send(tag.clone()) {
                                eprintln!("Failed to send TagValue tag through new `tag_sender`: {error:#}");
                            } else {
                                tag_senders.push(tag_sender);
                            }
                        } else {
                            tag_senders.push(tag_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, tag_sender_sender)
    }
}

impl Clone for TagValue {
    fn clone(&self) -> Self {
        let value_stream = self.subscribe();
        let (loop_task, tag_sender_sender) = Self::loop_task_and_tag_sender_sender(value_stream);
        TagValue { 
            is_clone: true,
            description: self.description,
            id: self.id.clone(),
            loop_task,
            tag_sender_sender 
        }
    }
}

impl Stream for TagValue {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
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

#[derive(Clone)]
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
    // @TODO remove
    #[allow(dead_code)]
    is_clone: bool,
    #[allow(dead_code)]
    loop_task: TaskHandle,
    change_sender_sender: mpsc::UnboundedSender<mpsc::UnboundedSender<ListChange>>,
}

impl List {
    pub fn new<const N: usize>(items: [CloneableValueStream; N]) -> Self {
        let change_stream = stream_one(ListChange::Replace { items: Vec::from(items) });
        let (loop_task, change_sender_sender) = Self::loop_task_and_change_sender_sender(change_stream);
        Self {
            is_clone: false,
            loop_task,
            change_sender_sender,
        }
    }

    fn subscribe(&self) -> impl Stream<Item = ListChange> {
        let (change_sender, change_receiver) = mpsc::unbounded();
        if let Err(error) = self.change_sender_sender.unbounded_send(change_sender) {
            eprintln!("Failed to send 'change_sender' through `change_sender_sender`: {error:#}");
        }
        change_receiver
    }

    fn loop_task_and_change_sender_sender(change_stream: impl Stream<Item = ListChange> + 'static) -> (TaskHandle, mpsc::UnboundedSender<mpsc::UnboundedSender<ListChange>>) {
        let (change_sender_sender, mut change_sender_receiver) = mpsc::unbounded::<mpsc::UnboundedSender<ListChange>>();
        let loop_task = Task::start_droppable(async move {
            let mut change_senders = Vec::<mpsc::UnboundedSender<ListChange>>::new();
            let mut change = None::<ListChange>;
            let mut change_stream = pin!(change_stream.fuse());
            loop {
                select! {
                    new_change = change_stream.next() => {
                        let Some(new_change) = new_change else { break };
                        change_senders.retain(|change_sender| {
                            if let Err(error) = change_sender.unbounded_send(new_change.clone()) {
                                eprintln!("Failed to send new List through `change_sender`: {error:#}");
                                false
                            } else {
                                true
                            }
                        });
                        change = Some(new_change);
                    }
                    change_sender = change_sender_receiver.select_next_some() => {
                        if let Some(change) = change.clone() {
                            if let Err(error) = change_sender.unbounded_send(change.clone()) {
                                eprintln!("Failed to send List through new `change_sender`: {error:#}");
                            } else {
                                change_senders.push(change_sender);
                            }
                        } else {
                            change_senders.push(change_sender);
                        }
                    }
                    complete => break
                }
            }
        });
        (loop_task, change_sender_sender)
    }
}

impl Clone for List {
    fn clone(&self) -> Self {
        let change_stream = self.subscribe();
        let (loop_task, change_sender_sender) = Self::loop_task_and_change_sender_sender(change_stream);
        List { 
            is_clone: true,
            loop_task,
            change_sender_sender 
        }
    }
}

impl Stream for List {
    type Item = ListChange;

    fn poll_next(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Self::Item>> {
        pin!(self.subscribe()).poll_next(cx)
    }
}

// @TODO remove
#[allow(dead_code)]
#[derive(Clone)]
pub enum ListChange {
    Replace {
        items: Vec<CloneableValueStream>,
    },
    InsertAt {
        index: usize,
        item: CloneableValueStream,
    },
    UpdateAt {
        index: usize,
        item: CloneableValueStream,
    },
    RemoveAt {
        index: usize,
    },
    Move {
        old_index: usize,
        new_index: usize,
    },
    Push {
        item: CloneableValueStream,
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
