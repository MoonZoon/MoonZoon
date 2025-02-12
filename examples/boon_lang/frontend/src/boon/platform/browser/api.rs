use std::future;
use std::sync::Arc;

use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::Timer;

use super::engine::*;

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
    arguments: &[Arc<ValueActor>],
    function_call_id: ConstructId,
) -> impl Stream<Item = Value> {
    let [argument_root] = arguments else { panic!("Unexpected argument count") };
    Object::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            "Document/new(..) -> [..]",
        ),
        [Variable::new_arc(
            ConstructInfo::new(
                function_call_id.with_child_id(1),
                "Document/new(..) -> [root_element]",
            ),
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
    arguments: &[Arc<ValueActor>],
    function_call_id: ConstructId,
) -> impl Stream<Item = Value> {
    let [_argument_element, argument_style, argument_child] = arguments else { panic!("Unexpected argument count") };
    TaggedObject::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            "Element/container(..) -> ElementContainer[..]",
        ),
        "ElementContainer",
        [Variable::new_arc(
            ConstructInfo::new(
                function_call_id.with_child_id(1),
                "Element/container(..) -> ElementContainer[settings]",
            ),
            "settings",
            Object::new_arc_value_actor(
                ConstructInfo::new(
                    function_call_id.with_child_id(2),
                    "Element/container(..) -> ElementContainer[settings: [..]]",
                ),
                RunDuration::Nonstop,
                [
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(3),
                            "Element/container(..) -> ElementContainer[settings: [style]]",
                        ),
                        "style",
                        argument_style.clone(),
                    ),
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(4),
                            "Element/container(..) -> ElementContainer[settings: [child]]",
                        ),
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
    arguments: &[Arc<ValueActor>],
    function_call_id: ConstructId,
) -> impl Stream<Item = Value> {
    let [_argument_element, argument_direction, argument_style, argument_items] = arguments else { panic!("Unexpected argument count") };
    TaggedObject::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            "Element/stripe(..) -> ElementStripe[..]",
        ),
        "ElementStripe",
        [Variable::new_arc(
            ConstructInfo::new(
                function_call_id.with_child_id(1),
                "Element/stripe(..) -> ElementStripe[settings]",
            ),
            "settings",
            Object::new_arc_value_actor(
                ConstructInfo::new(
                    function_call_id.with_child_id(2),
                    "Element/stripe(..) -> ElementStripe[settings: [..]]",
                ),
                RunDuration::Nonstop,
                [
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(3),
                            "Element/stripe(..) -> ElementStripe[settings: [direction]]",
                        ),
                        "direction",
                        argument_direction.clone(),
                    ),
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(4),
                            "Element/stripe(..) -> ElementStripe[settings: [style]]",
                        ),
                        "style",
                        argument_style.clone(),
                    ),
                    Variable::new_arc(
                        ConstructInfo::new(
                            function_call_id.with_child_id(5),
                            "Element/stripe(..) -> ElementStripe[settings: [items]]",
                        ),
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
    arguments: &[Arc<ValueActor>],
    function_call_id: ConstructId,
) -> impl Stream<Item = Value> {
    let [argument_element, argument_style, argument_label] = arguments else { panic!("Unexpected argument count") };
    TaggedObject::new_constant(
        ConstructInfo::new(
            function_call_id.with_child_id(0),
            "Element/stripe(..) -> ElementButton[..]",
        ),
        "ElementButton",
        [
            Variable::new_arc(
                ConstructInfo::new(
                    function_call_id.with_child_id(1),
                    "Element/stripe(..) -> ElementButton[event]",
                ),
                "event",
                ValueActor::new_arc(
                    ConstructInfo::new(
                        function_call_id.with_child_id(2),
                        "Element/stripe(..) -> ElementButton[event: [..]]",
                    ),
                    RunDuration::Nonstop,
                    argument_element
                        .subscribe()
                        .filter_map(|value| future::ready(value.expect_object().variable("event")))
                        .flat_map(|variable| variable.subscribe()),
                ),
            ),
            Variable::new_arc(
                ConstructInfo::new(
                    function_call_id.with_child_id(1),
                    "Element/stripe(..) -> ElementButton[settings]",
                ),
                "settings",
                Object::new_arc_value_actor(
                    ConstructInfo::new(
                        function_call_id.with_child_id(2),
                        "Element/stripe(..) -> ElementButton[settings: [..]]",
                    ),
                    RunDuration::Nonstop,
                    [
                        Variable::new_arc(
                            ConstructInfo::new(
                                function_call_id.with_child_id(1),
                                "Element/stripe(..) -> ElementButton[settings: [style]]",
                            ),
                            "style",
                            argument_style.clone(),
                        ),
                        Variable::new_arc(
                            ConstructInfo::new(
                                function_call_id.with_child_id(1),
                                "Element/stripe(..) -> ElementButton[settings: [label]]",
                            ),
                            "label",
                            argument_label.clone(),
                        ),
                    ],
                ),
            ),
        ],
    )
}

/// ```
/// Math/sum(increment<Number>) -> Number
/// ``````
pub fn function_math_sum(
    arguments: &[Arc<ValueActor>],
    function_call_id: ConstructId,
) -> impl Stream<Item = Value> {
    let [argument_increment] = arguments else { panic!("Unexpected argument count") };
    argument_increment
        .subscribe()
        .map(|value| value.expect_number().number())
        .scan(0., |sum, number| {
            *sum += number;
            future::ready(Some(*sum))
        })
        .map(move |sum| {
            Number::new_value(
                ConstructInfo::new(function_call_id.with_child_id(0), "Math/sum(..) -> Number"),
                sum,
            )
        })
}

/// ```
/// Timer/interval(duration<Duration[seconds<Number> | milliseconds<Number>]>) -> []
/// ```
pub fn function_timer_interval(
    arguments: &[Arc<ValueActor>],
    function_call_id: ConstructId,
) -> impl Stream<Item = Value> {
    let [argument_duration] = arguments else { panic!("Unexpected argument count") };
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
            stream::unfold(function_call_id, move |function_call_id| {
                async move {
                    Timer::sleep(milliseconds.round() as u32).await;
                    let output_value = Object::new_value(
                        ConstructInfo::new(function_call_id.with_child_id(0), "Timer/interval(.. ) -> [..]"),
                        []
                    );
                    Some((output_value, function_call_id))
                }
            })
        })
}
