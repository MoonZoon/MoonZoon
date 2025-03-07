// @TODO remove
#![allow(dead_code)]

use std::borrow::Cow;
use std::collections::{BTreeMap, HashMap};
use std::future::Future;
use std::pin::pin;
use std::sync::Arc;

use crate::boon::parser;

use ulid::Ulid;

use zoon::IntoCowStr;
use zoon::future;
use zoon::futures_channel::{mpsc, oneshot};
use zoon::futures_util::select;
use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::{Deserialize, DeserializeOwned, Serialize, serde, serde_json};
use zoon::{Task, TaskHandle};
use zoon::{WebStorage, local_storage};
use zoon::{eprintln, println};

const LOG_DROPS_AND_LOOP_ENDS: bool = false;

// --- constant ---

pub fn constant<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item)).chain(stream::once(future::pending()))
}

// --- ConstructContext ---

#[derive(Clone)]
pub struct ConstructContext {
    pub construct_storage: Arc<ConstructStorage>,
}

// --- ConstructStorage ---

pub struct ConstructStorage {
    state_inserter_sender: mpsc::UnboundedSender<(
        parser::PersistenceId,
        serde_json::Value,
        oneshot::Sender<()>,
    )>,
    state_getter_sender: mpsc::UnboundedSender<(
        parser::PersistenceId,
        oneshot::Sender<Option<serde_json::Value>>,
    )>,
    loop_task: TaskHandle,
}

// @TODO Replace LocalStorage with IndexedDB
// - https://crates.io/crates/indexed_db
// - https://developer.mozilla.org/en-US/docs/Web/API/IndexedDB_API
// - https://blog.openreplay.com/the-ultimate-guide-to-browser-side-storage/
impl ConstructStorage {
    pub fn new(states_local_storage_key: impl Into<Cow<'static, str>>) -> Self {
        let states_local_storage_key = states_local_storage_key.into();
        let (state_inserter_sender, mut state_inserter_receiver) = mpsc::unbounded();
        let (state_getter_sender, mut state_getter_receiver) = mpsc::unbounded();
        Self {
            state_inserter_sender,
            state_getter_sender,
            loop_task: Task::start_droppable(async move {
                let mut states = match local_storage().get(&states_local_storage_key) {
                    None => BTreeMap::<String, serde_json::Value>::new(),
                    Some(Ok(states)) => states,
                    Some(Err(error)) => panic!("Failed to deserialize states: {error:#}"),
                };
                loop {
                    select! {
                        (persistence_id, json_value, confirmation_sender) = state_inserter_receiver.select_next_some() => {
                            // @TODO remove `.to_string()` call when LocalStorage is replaced with IndexedDB (?)
                            states.insert(persistence_id.to_string(), json_value);
                            if let Err(error) = local_storage().insert(&states_local_storage_key, &states) {
                                eprintln!("Failed to save states: {error:#}");
                            }
                            if confirmation_sender.send(()).is_err() {
                                eprintln!("Failed to send save confirmation from construct storage");
                            }
                        },
                        (persistence_id, state_sender) = state_getter_receiver.select_next_some() => {
                            // @TODO Cheaper cloning? Replace get with remove?
                            let state = states.get(&persistence_id.to_string()).cloned();
                            if state_sender.send(state).is_err() {
                                eprintln!("Failed to send state from construct storage");
                            }
                        }
                    }
                }
            }),
        }
    }

    pub async fn save_state<T: Serialize>(&self, persistence_id: parser::PersistenceId, state: &T) {
        let json_value = match serde_json::to_value(state) {
            Ok(json_value) => json_value,
            Err(error) => {
                eprintln!("Failed to save state: {error:#}");
                return;
            }
        };
        let (confirmation_sender, confirmation_receiver) = oneshot::channel::<()>();
        if let Err(error) = self.state_inserter_sender.unbounded_send((
            persistence_id,
            json_value,
            confirmation_sender,
        )) {
            eprintln!("Failed to save state: {error:#}")
        }
        confirmation_receiver
            .await
            .expect("Failed to get confirmation from ConstructStorage")
    }

    // @TODO is &self enough?
    pub async fn load_state<T: DeserializeOwned>(
        self: Arc<Self>,
        persistence_id: parser::PersistenceId,
    ) -> Option<T> {
        let (state_sender, state_receiver) = oneshot::channel::<Option<serde_json::Value>>();
        if let Err(error) = self
            .state_getter_sender
            .unbounded_send((persistence_id, state_sender))
        {
            eprintln!("Failed to load state: {error:#}")
        }
        let json_value = state_receiver
            .await
            .expect("Failed to get state from ConstructStorage")?;
        match serde_json::from_value(json_value) {
            Ok(state) => Some(state),
            Err(error) => {
                panic!("Failed to load state: {error:#}");
            }
        }
    }
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
            }),
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
    // @TODO remove Option in the future once Persistence is created also inside API functions?
    persistence: Option<parser::Persistence>,
    description: Cow<'static, str>,
}

impl ConstructInfo {
    pub fn new(
        id: impl Into<ConstructId>,
        persistence: Option<parser::Persistence>,
        description: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            id: id.into(),
            persistence,
            description: description.into(),
        }
    }

    pub fn complete(self, r#type: ConstructType) -> ConstructInfoComplete {
        ConstructInfoComplete {
            r#type,
            id: self.id,
            persistence: self.persistence,
            description: self.description,
        }
    }
}

// --- ConstructInfoComplete ---

#[derive(Clone)]
pub struct ConstructInfoComplete {
    r#type: ConstructType,
    id: ConstructId,
    persistence: Option<parser::Persistence>,
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct ConstructId {
    ids: Arc<Vec<Cow<'static, str>>>,
}

impl ConstructId {
    pub fn new(id: impl IntoCowStr<'static>) -> Self {
        Self {
            ids: Arc::new(vec![id.into_cow_str()]),
        }
    }

    pub fn with_child_id(&self, child: impl IntoCowStr<'static>) -> Self {
        let mut ids = Vec::clone(&self.ids);
        ids.push(child.into_cow_str());
        Self { ids: Arc::new(ids) }
    }
}

impl<T: IntoCowStr<'static>> From<T> for ConstructId {
    fn from(id: T) -> Self {
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
        construct_context: ConstructContext,
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
        construct_context: ConstructContext,
        name: impl Into<Cow<'static, str>>,
        value_actor: Arc<ValueActor>,
    ) -> Arc<Self> {
        Arc::new(Self::new(
            construct_info,
            construct_context,
            name,
            value_actor,
        ))
    }

    pub fn new_link_arc(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        name: impl Into<Cow<'static, str>>,
        actor_context: ActorContext,
    ) -> Arc<Self> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: variable_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped Variable"),
            persistence,
            variable_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Link variable value actor")
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

    pub fn subscribe(&self) -> impl Stream<Item = Value> + use<> {
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
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
    }
}

// --- VariableOrArgumentReference ---

pub struct VariableOrArgumentReference {}

impl VariableOrArgumentReference {
    pub fn new_arc_value_actor<'code>(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        actor_context: ActorContext,
        alias: parser::Alias<'code>,
        root_value_actor: impl Future<Output = Arc<ValueActor>> + 'static,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::VariableOrArgumentReference);
        let mut skip_alias_parts = 0;
        let alias_parts = match alias {
            parser::Alias::WithoutPassed {
                parts,
                referenceables: _,
            } => {
                skip_alias_parts = 1;
                parts
            }
            parser::Alias::WithPassed { extra_parts } => extra_parts,
        };
        let mut value_stream = stream::once(root_value_actor)
            .flat_map(|actor| actor.subscribe())
            .boxed_local();
        for alias_part in alias_parts.into_iter().skip(skip_alias_parts) {
            let alias_part = alias_part.to_owned();
            value_stream = value_stream
                .flat_map(move |value| match value {
                    Value::Object(object, _) => object.expect_variable(&alias_part).subscribe(),
                    Value::TaggedObject(tagged_object, _) => {
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

// --- ReferenceConnector ---

pub struct ReferenceConnector {
    referenceable_inserter_sender: mpsc::UnboundedSender<(parser::Span, Arc<ValueActor>)>,
    referenceable_getter_sender:
        mpsc::UnboundedSender<(parser::Span, oneshot::Sender<Arc<ValueActor>>)>,
    loop_task: TaskHandle,
}

impl ReferenceConnector {
    pub fn new() -> Self {
        let (referenceable_inserter_sender, mut referenceable_inserter_receiver) =
            mpsc::unbounded();
        let (referenceable_getter_sender, mut referenceable_getter_receiver) = mpsc::unbounded();
        Self {
            referenceable_inserter_sender,
            referenceable_getter_sender,
            loop_task: Task::start_droppable(async move {
                let mut referenceables = HashMap::<parser::Span, Arc<ValueActor>>::new();
                let mut referenceable_senders =
                    HashMap::<parser::Span, Vec<oneshot::Sender<Arc<ValueActor>>>>::new();
                loop {
                    select! {
                        (span, actor) = referenceable_inserter_receiver.select_next_some() => {
                            if let Some(senders) = referenceable_senders.remove(&span) {
                                for sender in senders {
                                    if sender.send(actor.clone()).is_err() {
                                        eprintln!("Failed to send referenceable actor from reference connector");
                                    }
                                }
                            }
                            referenceables.insert(span, actor);
                        },
                        (span, referenceable_sender) = referenceable_getter_receiver.select_next_some() => {
                            if let Some(actor) = referenceables.get(&span) {
                                if referenceable_sender.send(actor.clone()).is_err() {
                                    eprintln!("Failed to send referenceable actor from reference connector");
                                }
                            } else {
                                referenceable_senders.entry(span).or_default().push(referenceable_sender);
                            }
                        }
                    }
                }
            }),
        }
    }

    pub fn register_referenceable(&self, span: parser::Span, actor: Arc<ValueActor>) {
        if let Err(error) = self
            .referenceable_inserter_sender
            .unbounded_send((span, actor))
        {
            eprintln!("Failed to register referenceable: {error:#}")
        }
    }

    // @TODO is &self enough?
    pub async fn referenceable(self: Arc<Self>, span: parser::Span) -> Arc<ValueActor> {
        let (referenceable_sender, referenceable_receiver) = oneshot::channel();
        if let Err(error) = self
            .referenceable_getter_sender
            .unbounded_send((span, referenceable_sender))
        {
            eprintln!("Failed to register referenceable: {error:#}")
        }
        referenceable_receiver
            .await
            .expect("Failed to get referenceable from ReferenceConnector")
    }
}

// --- FunctionCall ---

pub struct FunctionCall {}

impl FunctionCall {
    pub fn new_arc_value_actor<FR: Stream<Item = Value> + 'static>(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        actor_context: ActorContext,
        definition: impl Fn(
            Arc<Vec<Arc<ValueActor>>>,
            ConstructId,
            parser::PersistenceId,
            ConstructContext,
            ActorContext,
        ) -> FR
        + 'static,
        arguments: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<ValueActor> {
        let construct_info = construct_info.complete(ConstructType::FunctionCall);
        let arguments = Arc::new(arguments.into());
        let value_stream = definition(
            arguments.clone(),
            construct_info.id(),
            construct_info
                .persistence
                .expect("Failed to get FunctionCall Persistence")
                .id,
            construct_context,
            actor_context.clone(),
        );
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
        construct_context: ConstructContext,
        actor_context: ActorContext,
        inputs: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<ValueActor> {
        #[derive(Default, Clone, Serialize, Deserialize)]
        #[serde(crate = "serde")]
        struct State {
            input_idempotency_keys: BTreeMap<usize, ValueIdempotencyKey>,
        }

        let construct_info = construct_info.complete(ConstructType::LatestCombinator);
        let inputs: Vec<Arc<ValueActor>> = inputs.into();
        let persistent_id = construct_info
            .persistence
            .expect("Failed to get Persistence in LatestCombinator")
            .id;
        let storage = construct_context.construct_storage.clone();

        let value_stream =
            stream::select_all(inputs.iter().enumerate().map(|(index, value_actor)| {
                value_actor.subscribe().map(move |value| (index, value))
            }))
            .scan(true, {
                let storage = storage.clone();
                move |first_run, (index, value)| {
                    let storage = storage.clone();
                    let previous_first_run = *first_run;
                    *first_run = false;
                    async move {
                        if previous_first_run {
                            Some((
                                storage.clone().load_state::<State>(persistent_id).await,
                                index,
                                value,
                            ))
                        } else {
                            Some((None, index, value))
                        }
                    }
                }
            })
            .scan(State::default(), move |state, (new_state, index, value)| {
                if let Some(new_state) = new_state {
                    *state = new_state;
                }
                let idempotency_key = value.idempotency_key();
                let skip_value = state.input_idempotency_keys.get(&index).is_some_and(
                    |previous_idempotency_key| *previous_idempotency_key == idempotency_key,
                );
                if !skip_value {
                    state.input_idempotency_keys.insert(index, idempotency_key);
                }
                // @TODO Refactor to get rid of the `clone` call. Use async closure?
                let state = state.clone();
                let storage = storage.clone();
                async move {
                    if skip_value {
                        Some(None)
                    } else {
                        storage.save_state(persistent_id, &state).await;
                        Some(Some(value))
                    }
                }
            })
            .filter_map(future::ready);

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
        construct_context: ConstructContext,
        actor_context: ActorContext,
        observed: Arc<ValueActor>,
        impulse_sender: mpsc::UnboundedSender<()>,
        body: Arc<ValueActor>,
    ) -> Arc<ValueActor> {
        #[derive(Default, Copy, Clone, Serialize, Deserialize)]
        #[serde(crate = "serde")]
        struct State {
            observed_idempotency_key: Option<ValueIdempotencyKey>,
        }

        let construct_info = construct_info.complete(ConstructType::ThenCombinator);
        let persistent_id = construct_info
            .persistence
            .expect("Failed to get Persistence in ThenCombinator")
            .id;
        let storage = construct_context.construct_storage.clone();

        let send_impulse_task = Task::start_droppable(
            observed
                .subscribe()
                .scan(true, {
                    let storage = storage.clone();
                    move |first_run, value| {
                        let storage = storage.clone();
                        let previous_first_run = *first_run;
                        *first_run = false;
                        async move {
                            if previous_first_run {
                                Some((
                                    storage.clone().load_state::<State>(persistent_id).await,
                                    value,
                                ))
                            } else {
                                Some((None, value))
                            }
                        }
                    }
                })
                .scan(State::default(), move |state, (new_state, value)| {
                    if let Some(new_state) = new_state {
                        *state = new_state;
                    }
                    let idempotency_key = value.idempotency_key();
                    let skip_value = state
                        .observed_idempotency_key
                        .is_some_and(|key| key == idempotency_key);
                    if !skip_value {
                        state.observed_idempotency_key = Some(idempotency_key);
                    }
                    let state = *state;
                    let storage = storage.clone();
                    async move {
                        if skip_value {
                            Some(None)
                        } else {
                            storage.save_state(persistent_id, &state).await;
                            Some(Some(value))
                        }
                    }
                })
                .filter_map(future::ready)
                .for_each({
                    let construct_info = construct_info.clone();
                    move |_| {
                        if let Err(error) = impulse_sender.unbounded_send(()) {
                            eprintln!("Failed to send impulse in {construct_info}: {error:#}")
                        }
                        future::ready(())
                    }
                }),
        );
        let value_stream = body.subscribe().map(|mut value| {
            value.set_idempotency_key(ValueIdempotencyKey::new());
            value
        });
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
                let mut output_valve_impulse_stream =
                    if let Some(output_valve_signal) = &output_valve_signal {
                        output_valve_signal.subscribe().left_stream()
                    } else {
                        stream::pending().right_stream()
                    }
                    .fuse();
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
                if LOG_DROPS_AND_LOOP_ENDS {
                    println!("Loop ended {construct_info}");
                }
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

    pub fn subscribe(&self) -> impl Stream<Item = Value> + use<> {
        let (value_sender, value_receiver) = mpsc::unbounded();
        if let Err(error) = self.value_sender_sender.unbounded_send(value_sender) {
            eprintln!("Failed to subscribe to {}: {error:#}", self.construct_info);
        }
        value_receiver
    }
}

impl Drop for ValueActor {
    fn drop(&mut self) {
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
    }
}

// --- ValueIdempotencyKey ---

pub type ValueIdempotencyKey = Ulid;

// --- ValueMetadata ---

#[derive(Clone, Copy)]
pub struct ValueMetadata {
    pub idempotency_key: ValueIdempotencyKey,
}

// --- Value ---

#[derive(Clone)]
pub enum Value {
    Object(Arc<Object>, ValueMetadata),
    TaggedObject(Arc<TaggedObject>, ValueMetadata),
    Text(Arc<Text>, ValueMetadata),
    Tag(Arc<Tag>, ValueMetadata),
    Number(Arc<Number>, ValueMetadata),
    List(Arc<List>, ValueMetadata),
}

impl Value {
    pub fn construct_info(&self) -> &ConstructInfoComplete {
        match self {
            Self::Object(object, _) => &object.construct_info,
            Self::TaggedObject(tagged_object, _) => &tagged_object.construct_info,
            Self::Text(text, _) => &text.construct_info,
            Self::Tag(tag, _) => &tag.construct_info,
            Self::Number(number, _) => &number.construct_info,
            Self::List(list, _) => &list.construct_info,
        }
    }

    pub fn metadata(&self) -> ValueMetadata {
        match self {
            Self::Object(_, metadata) => *metadata,
            Self::TaggedObject(_, metadata) => *metadata,
            Self::Text(_, metadata) => *metadata,
            Self::Tag(_, metadata) => *metadata,
            Self::Number(_, metadata) => *metadata,
            Self::List(_, metadata) => *metadata,
        }
    }
    pub fn metadata_mut(&mut self) -> &mut ValueMetadata {
        match self {
            Self::Object(_, metadata) => metadata,
            Self::TaggedObject(_, metadata) => metadata,
            Self::Text(_, metadata) => metadata,
            Self::Tag(_, metadata) => metadata,
            Self::Number(_, metadata) => metadata,
            Self::List(_, metadata) => metadata,
        }
    }

    pub fn idempotency_key(&self) -> ValueIdempotencyKey {
        self.metadata().idempotency_key
    }

    pub fn set_idempotency_key(&mut self, key: ValueIdempotencyKey) {
        self.metadata_mut().idempotency_key = key;
    }

    pub fn expect_object(self) -> Arc<Object> {
        let Self::Object(object, _) = self else {
            panic!(
                "Failed to get expected Object: The Value has a different type {}",
                self.construct_info()
            )
        };
        object
    }

    pub fn expect_tagged_object(self, tag: &str) -> Arc<TaggedObject> {
        let Self::TaggedObject(tagged_object, _) = self else {
            panic!("Failed to get expected TaggedObject: The Value has a different type")
        };
        let found_tag = &tagged_object.tag;
        if found_tag != tag {
            panic!(
                "Failed to get expected TaggedObject: Expected tag: '{tag}', found tag: '{found_tag}'"
            )
        }
        tagged_object
    }

    pub fn expect_text(self) -> Arc<Text> {
        let Self::Text(text, _) = self else {
            panic!("Failed to get expected Text: The Value has a different type")
        };
        text
    }

    pub fn expect_tag(self) -> Arc<Tag> {
        let Self::Tag(tag, _) = self else {
            panic!("Failed to get expected Tag: The Value has a different type")
        };
        tag
    }

    pub fn expect_number(self) -> Arc<Number> {
        let Self::Number(number, _) = self else {
            panic!("Failed to get expected Number: The Value has a different type")
        };
        number
    }

    pub fn expect_list(self) -> Arc<List> {
        let Self::List(list, _) = self else {
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
        construct_context: ConstructContext,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Object),
            variables: variables.into(),
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, construct_context, variables))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Value {
        Value::Object(
            Self::new_arc(construct_info, construct_context, variables),
            ValueMetadata { idempotency_key },
        )
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(
            construct_info,
            construct_context,
            idempotency_key,
            variables,
        ))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: object_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped Object"),
            persistence,
            object_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Constant object wrapper")
                .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(
            construct_info,
            construct_context,
            idempotency_key,
            variables.into(),
        );
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
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
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
        construct_context: ConstructContext,
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
        construct_context: ConstructContext,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, construct_context, tag, variables))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Value {
        Value::TaggedObject(
            Self::new_arc(construct_info, construct_context, tag, variables),
            ValueMetadata { idempotency_key },
        )
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(
            construct_info,
            construct_context,
            idempotency_key,
            tag,
            variables,
        ))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        tag: impl Into<Cow<'static, str>>,
        variables: impl Into<Vec<Arc<Variable>>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: tagged_object_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped TaggedObject"),
            persistence,
            tagged_object_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Tagged object wrapper")
                .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(
            construct_info,
            construct_context,
            idempotency_key,
            tag.into(),
            variables.into(),
        );
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
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
    }
}

// --- Text ---

pub struct Text {
    construct_info: ConstructInfoComplete,
    text: Cow<'static, str>,
}

impl Text {
    pub fn new(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        text: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Text),
            text: text.into(),
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        text: impl Into<Cow<'static, str>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, construct_context, text))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        text: impl Into<Cow<'static, str>>,
    ) -> Value {
        Value::Text(
            Self::new_arc(construct_info, construct_context, text),
            ValueMetadata { idempotency_key },
        )
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        text: impl Into<Cow<'static, str>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(
            construct_info,
            construct_context,
            idempotency_key,
            text,
        ))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        text: impl Into<Cow<'static, str>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: text_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped Text"),
            persistence,
            text_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Constant text wrapper")
                .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(
            construct_info,
            construct_context,
            idempotency_key,
            text.into(),
        );
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
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
    }
}

// --- Tag ---

pub struct Tag {
    construct_info: ConstructInfoComplete,
    tag: Cow<'static, str>,
}

impl Tag {
    pub fn new(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        tag: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Tag),
            tag: tag.into(),
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        tag: impl Into<Cow<'static, str>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, construct_context, tag))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        tag: impl Into<Cow<'static, str>>,
    ) -> Value {
        Value::Tag(
            Self::new_arc(construct_info, construct_context, tag),
            ValueMetadata { idempotency_key },
        )
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        tag: impl Into<Cow<'static, str>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(
            construct_info,
            construct_context,
            idempotency_key,
            tag,
        ))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        tag: impl Into<Cow<'static, str>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: tag_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped Tag"),
            persistence,
            tag_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Constant tag wrapper")
                .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(
            construct_info,
            construct_context,
            idempotency_key,
            tag.into(),
        );
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
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
    }
}

// --- Number ---

pub struct Number {
    construct_info: ConstructInfoComplete,
    number: f64,
}

impl Number {
    pub fn new(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        number: impl Into<f64>,
    ) -> Self {
        Self {
            construct_info: construct_info.complete(ConstructType::Number),
            number: number.into(),
        }
    }

    pub fn new_arc(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        number: impl Into<f64>,
    ) -> Arc<Self> {
        Arc::new(Self::new(construct_info, construct_context, number))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        number: impl Into<f64>,
    ) -> Value {
        Value::Number(
            Self::new_arc(construct_info, construct_context, number),
            ValueMetadata { idempotency_key },
        )
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        number: impl Into<f64>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(
            construct_info,
            construct_context,
            idempotency_key,
            number,
        ))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        number: impl Into<f64>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: number_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped Number"),
            persistence,
            number_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Constant number wrapper)")
                .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(
            construct_info,
            construct_context,
            idempotency_key,
            number.into(),
        );
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
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
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
        construct_context: ConstructContext,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Self {
        let change_stream = constant(ListChange::Replace {
            items: items.into(),
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
                let mut output_valve_impulse_stream =
                    if let Some(output_valve_signal) = &output_valve_signal {
                        output_valve_signal.subscribe().left_stream()
                    } else {
                        stream::pending().right_stream()
                    }
                    .fuse();
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
                if LOG_DROPS_AND_LOOP_ENDS {
                    println!("Loop ended {construct_info}");
                }
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
        construct_context: ConstructContext,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<Self> {
        Arc::new(Self::new(
            construct_info,
            construct_context,
            actor_context,
            items,
        ))
    }

    pub fn new_value(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Value {
        Value::List(
            Self::new_arc(construct_info, construct_context, actor_context, items),
            ValueMetadata { idempotency_key },
        )
    }

    pub fn new_constant(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> impl Stream<Item = Value> {
        constant(Self::new_value(
            construct_info,
            construct_context,
            idempotency_key,
            actor_context,
            items,
        ))
    }

    pub fn new_arc_value_actor(
        construct_info: ConstructInfo,
        construct_context: ConstructContext,
        idempotency_key: ValueIdempotencyKey,
        actor_context: ActorContext,
        items: impl Into<Vec<Arc<ValueActor>>>,
    ) -> Arc<ValueActor> {
        let ConstructInfo {
            id: actor_id,
            persistence,
            description: list_description,
        } = construct_info;
        let construct_info = ConstructInfo::new(
            actor_id.with_child_id("wrapped List"),
            persistence,
            list_description,
        );
        let actor_construct_info =
            ConstructInfo::new(actor_id, persistence, "Constant list wrapper")
                .complete(ConstructType::ValueActor);
        let value_stream = Self::new_constant(
            construct_info,
            construct_context,
            idempotency_key,
            actor_context.clone(),
            items.into(),
        );
        Arc::new(ValueActor::new_internal(
            actor_construct_info,
            actor_context,
            value_stream,
            (),
        ))
    }

    pub fn subscribe(&self) -> impl Stream<Item = ListChange> + use<> {
        let (change_sender, change_receiver) = mpsc::unbounded();
        if let Err(error) = self.change_sender_sender.unbounded_send(change_sender) {
            eprintln!("Failed to subscribe to {}: {error:#}", self.construct_info);
        }
        change_receiver
    }
}

impl Drop for List {
    fn drop(&mut self) {
        if LOG_DROPS_AND_LOOP_ENDS {
            println!("Dropped: {}", self.construct_info);
        }
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
