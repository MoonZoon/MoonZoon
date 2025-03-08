use std::future;
use std::sync::Arc;

use zoon::Timer;
use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::{Deserialize, Serialize, serde};

use super::engine::*;

use crate::boon::parser::PersistenceId;

// @TODO make sure Values are deduplicated everywhere it makes sense

/// ```
/// Document/new(root<INTO_ELEMENT>) -> [root_element<INTO_ELEMENT>]
/// INTO_ELEMENT: <ELEMENT | Text | Number>
/// ELEMENT: <
///     | ELEMENT_CONTAINER
///     | ELEMENT_STRIPE
///     | ELEMENT_BUTTON
/// >
/// ELEMENT_CONTAINER: ElementContainer[
///     settings<[
///         style<[]>
///         child<INTO_ELEMENT>
///     ]>
/// ]
/// ELEMENT_STRIPE: ElementStripe[
///     settings<[
///         direction<Column | Row>
///         style<[]>
///         items<List<INTO_ELEMENT>>
///     ]>
/// ]
/// ELEMENT_BUTTON: ElementButton[
///     event?<[
///         press?<LINK<[]>>
///     ]>
///     settings<[
///         style<[]>
///         label<INTO_ELEMENT>
///     ]>
/// ]
/// >
/// ```
pub fn function_document_new(
    arguments: Arc<Vec<Arc<ValueActor>>>,
    function_call_id: ConstructId,
    function_call_persistence_id: PersistenceId,
    construct_context: ConstructContext,
    actor_context: ActorContext,
) -> impl Stream<Item = Value> {
    let [argument_root] = arguments.as_slice() else {
        panic!("Unexpected argument count")
    };
    Object::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            None,
            "Document/new(..) -> [..]",
        ),
        construct_context.clone(),
        ValueIdempotencyKey::new(),
        [Variable::new_arc(
            ConstructInfo::new(
                function_call_id.with_child_id(1),
                None,
                "Document/new(..) -> [root_element]",
            ),
            construct_context,
            "root_element",
            argument_root.clone(),
        )],
    )
}

// @TODO remove
#[allow(dead_code)]
/// ```
/// Element/container(
///     element<[]>
///     style<[]>
///     child<INTO_ELEMENT>
/// ) -> ELEMENT_CONTAINER
/// ```
pub fn function_element_container(
    arguments: Arc<Vec<Arc<ValueActor>>>,
    function_call_id: ConstructId,
    function_call_persistence_id: PersistenceId,
    construct_context: ConstructContext,
    actor_context: ActorContext,
) -> impl Stream<Item = Value> {
    let [_argument_element, argument_style, argument_child] = arguments.as_slice() else {
        panic!("Unexpected argument count")
    };
    TaggedObject::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            None,
            "Element/container(..) -> ElementContainer[..]",
        ),
        construct_context.clone(),
        ValueIdempotencyKey::new(),
        "ElementContainer",
        [Variable::new_arc(
            ConstructInfo::new(
                function_call_id.with_child_id(1),
                None,
                "Element/container(..) -> ElementContainer[settings]",
            ),
            construct_context.clone(),
            "settings",
            Object::new_arc_value_actor(
                ConstructInfo::new(
                    function_call_id.with_child_id(2),
                    None,
                    "Element/container(..) -> ElementContainer[settings: [..]]",
                ),
                construct_context.clone(),
                ValueIdempotencyKey::new(),
                actor_context,
                [
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(3),
                            None,
                            "Element/container(..) -> ElementContainer[settings: [style]]",
                        ),
                        construct_context.clone(),
                        "style",
                        argument_style.clone(),
                    ),
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(4),
                            None,
                            "Element/container(..) -> ElementContainer[settings: [child]]",
                        ),
                        construct_context,
                        "child",
                        argument_child.clone(),
                    ),
                ],
            ),
        )],
    )
}

/// ```
/// Element/stripe(
///     element<[]>
///     direction<Column | Row>
///     style<[]>
///     items<List<INTO_ELEMENT>>
/// ) -> ELEMENT_STRIPE
/// ```
pub fn function_element_stripe(
    arguments: Arc<Vec<Arc<ValueActor>>>,
    function_call_id: ConstructId,
    function_call_persistence_id: PersistenceId,
    construct_context: ConstructContext,
    actor_context: ActorContext,
) -> impl Stream<Item = Value> {
    let [
        _argument_element,
        argument_direction,
        argument_style,
        argument_items,
    ] = arguments.as_slice()
    else {
        panic!("Unexpected argument count")
    };
    TaggedObject::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            None,
            "Element/stripe(..) -> ElementStripe[..]",
        ),
        construct_context.clone(),
        ValueIdempotencyKey::new(),
        "ElementStripe",
        [Variable::new_arc(
            ConstructInfo::new(
                function_call_id.with_child_id(1),
                None,
                "Element/stripe(..) -> ElementStripe[settings]",
            ),
            construct_context.clone(),
            "settings",
            Object::new_arc_value_actor(
                ConstructInfo::new(
                    function_call_id.with_child_id(2),
                    None,
                    "Element/stripe(..) -> ElementStripe[settings: [..]]",
                ),
                construct_context.clone(),
                ValueIdempotencyKey::new(),
                actor_context,
                [
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(3),
                            None,
                            "Element/stripe(..) -> ElementStripe[settings: [direction]]",
                        ),
                        construct_context.clone(),
                        "direction",
                        argument_direction.clone(),
                    ),
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(4),
                            None,
                            "Element/stripe(..) -> ElementStripe[settings: [style]]",
                        ),
                        construct_context.clone(),
                        "style",
                        argument_style.clone(),
                    ),
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(5),
                            None,
                            "Element/stripe(..) -> ElementStripe[settings: [items]]",
                        ),
                        construct_context,
                        "items",
                        argument_items.clone(),
                    ),
                ],
            ),
        )],
    )
}

/// ```
/// Element/button(
///     element<[
///         event?<[
///             press?<LINK<[]>>
///         ]>
///     ]>
///     style<[]>
///     label<INTO_ELEMENT>
/// ) -> ELEMENT_BUTTON
/// ```
pub fn function_element_button(
    arguments: Arc<Vec<Arc<ValueActor>>>,
    function_call_id: ConstructId,
    function_call_persistence_id: PersistenceId,
    construct_context: ConstructContext,
    actor_context: ActorContext,
) -> impl Stream<Item = Value> {
    let [argument_element, argument_style, argument_label] = arguments.as_slice() else {
        panic!("Unexpected argument count")
    };
    TaggedObject::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            None,
            "Element/stripe(..) -> ElementButton[..]",
        ),
        construct_context.clone(),
        ValueIdempotencyKey::new(),
        "ElementButton",
        [
            Variable::new_arc(
                ConstructInfo::new(
                    function_call_id.with_child_id(1),
                    None,
                    "Element/stripe(..) -> ElementButton[event]",
                ),
                construct_context.clone(),
                "event",
                ValueActor::new_arc(
                    ConstructInfo::new(
                        function_call_id.with_child_id(2),
                        None,
                        "Element/stripe(..) -> ElementButton[event: [..]]",
                    ),
                    actor_context.clone(),
                    argument_element
                        .subscribe()
                        .filter_map(|value| future::ready(value.expect_object().variable("event")))
                        .flat_map(|variable| variable.subscribe()),
                ),
            ),
            Variable::new_arc(
                ConstructInfo::new(
                    function_call_id.with_child_id(3),
                    None,
                    "Element/stripe(..) -> ElementButton[settings]",
                ),
                construct_context.clone(),
                "settings",
                Object::new_arc_value_actor(
                    ConstructInfo::new(
                        function_call_id.with_child_id(4),
                        None,
                        "Element/stripe(..) -> ElementButton[settings: [..]]",
                    ),
                    construct_context.clone(),
                    ValueIdempotencyKey::new(),
                    actor_context,
                    [
                        Variable::new_arc(
                            ConstructInfo::new(
                                function_call_id.with_child_id(5),
                                None,
                                "Element/stripe(..) -> ElementButton[settings: [style]]",
                            ),
                            construct_context.clone(),
                            "style",
                            argument_style.clone(),
                        ),
                        Variable::new_arc(
                            ConstructInfo::new(
                                function_call_id.with_child_id(6),
                                None,
                                "Element/stripe(..) -> ElementButton[settings: [label]]",
                            ),
                            construct_context,
                            "label",
                            argument_label.clone(),
                        ),
                    ],
                ),
            ),
        ],
    )
}

// @TODO refactor
/// ```
/// Math/sum(increment<Number>) -> Number
/// ``````
pub fn function_math_sum(
    arguments: Arc<Vec<Arc<ValueActor>>>,
    function_call_id: ConstructId,
    function_call_persistence_id: PersistenceId,
    construct_context: ConstructContext,
    actor_context: ActorContext,
) -> impl Stream<Item = Value> {
    #[derive(Default, Clone, Copy, Serialize, Deserialize)]
    #[serde(crate = "serde")]
    struct State {
        input_value_idempotency_key: Option<ValueIdempotencyKey>,
        sum: f64,
        output_value_idempotency_key: Option<ValueIdempotencyKey>,
    }

    let [argument_increment] = arguments.as_slice() else {
        panic!("Unexpected argument count")
    };
    let storage = construct_context.construct_storage.clone();
    stream::once(storage.clone().load_state(function_call_persistence_id))
        .filter_map(future::ready)
        .chain(argument_increment.subscribe().map(|value| State {
            input_value_idempotency_key: Some(value.idempotency_key()),
            sum: value.expect_number().number(),
            output_value_idempotency_key: None,
        }))
        // @TODO refactor with async closure once possible?
        .scan(State::default(), {
            move |state,
                  State {
                      input_value_idempotency_key,
                      sum: number,
                      output_value_idempotency_key,
                  }| {
                let storage = storage.clone();
                let skip_value = state.input_value_idempotency_key == input_value_idempotency_key;
                if !skip_value {
                    state.input_value_idempotency_key = input_value_idempotency_key;
                    state.sum += number;
                    state.output_value_idempotency_key = if output_value_idempotency_key.is_some() {
                        output_value_idempotency_key
                    } else {
                        Some(ValueIdempotencyKey::new())
                    };
                }
                let state = *state;
                async move {
                    if skip_value {
                        Some(None)
                    } else {
                        storage
                            .save_state(function_call_persistence_id, &state)
                            .await;
                        Some(Some((
                            state.sum,
                            state.output_value_idempotency_key.unwrap(),
                        )))
                    }
                }
            }
        })
        .filter_map(future::ready)
        .map({
            let mut result_version = 0u64;
            move |(sum, idempotency_key)| {
                let value = Number::new_value(
                    ConstructInfo::new(
                        function_call_id
                            .with_child_id(format!("Math/sum result v.{result_version}")),
                        None,
                        "Math/sum(..) -> Number",
                    ),
                    construct_context.clone(),
                    idempotency_key,
                    sum,
                );
                result_version += 1;
                value
            }
        })
}

// @TODO remember configuration?
/// ```
/// Timer/interval(duration<Duration[seconds<Number> | milliseconds<Number>]>) -> []
/// ```
pub fn function_timer_interval(
    arguments: Arc<Vec<Arc<ValueActor>>>,
    function_call_id: ConstructId,
    function_call_persistence_id: PersistenceId,
    construct_context: ConstructContext,
    actor_context: ActorContext,
) -> impl Stream<Item = Value> {
    let [argument_duration] = arguments.as_slice() else {
        panic!("Unexpected argument count")
    };
    argument_duration
        .subscribe()
        .flat_map(|value| {
            let duration_object = value.expect_tagged_object("Duration");
            if let Some(seconds) = duration_object.variable("seconds") {
                seconds.subscribe().map(|value| value.expect_number().number() * 1000.).left_stream()
            } else if let Some(milliseconds) = duration_object.variable("milliseconds") {
                milliseconds.subscribe().map(|value| value.expect_number().number()).right_stream()
            } else {
                panic!("Failed to get property 'seconds' or 'milliseconds' from tagged object 'Duration'");
            }
        })
        .flat_map(move |milliseconds| {
            let function_call_id = function_call_id.clone();
            stream::unfold((function_call_id, 0u64), {
                let construct_context = construct_context.clone();
                move |(function_call_id, result_version)| {
                    let construct_context = construct_context.clone();
                    async move {
                        // @TODO How to properly resolve resuming? Only if it's a longer interval?
                        Timer::sleep(milliseconds.round() as u32).await;
                        let output_value = Object::new_value(
                            ConstructInfo::new(function_call_id.with_child_id("Timer/interval result v.{result_version}"), None, "Timer/interval(.. ) -> [..]"),
                            construct_context.clone(),
                            ValueIdempotencyKey::new(),
                            []
                        );
                        Some((output_value, (function_call_id, result_version + 1)))
                    }
                }
            })
        })
}
