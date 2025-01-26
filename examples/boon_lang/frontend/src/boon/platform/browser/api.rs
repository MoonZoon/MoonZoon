use std::sync::Arc;
use std::future;

use zoon::futures_util::stream::{self, Stream, StreamExt};
use zoon::Timer;

use super::engine::*;

/// `Document/new(root<INTO_ELEMENT>) -> [root_element<INTO_ELEMENT>]`
/// 
/// `INTO_ELEMENT: <ELEMENT | Text | Number>`
/// 
/// ```
/// ELEMENT: <
///     ElementContainer[
///         settings<[
///             style<[]>
///             child<INTO_ELEMENT>
///         ]>
///     ]
///     | ElementStripe [
///         settings<[
///             direction<Column | Row>
///             style<[]>
///             items<List<INTO_ELEMENT>>
///         ]>
///     ]
///     | ElementButton[
///         event?<[press?<LINK<[]>>]>
///         settings<[
///             style<[]>
///             label<Text>
///         ]>
///     ]
/// >
/// ```
pub fn function_document_new(arguments: [Arc<ValueActor>; 1], function_call_id: ConstructId) -> impl Stream<Item = Value> {
    let [argument_root] = arguments;
    Object::new_constant(
        ConstructInfo::new(function_call_id.with_child_id(0), "Document/new(..) -> [..]"),
        [
            Variable::new_arc(
                ConstructInfo::new(function_call_id.with_child_id(1), "Document/new(..) -> [root_element]"),
                "root_element",
                argument_root
            )
        ]
    )
}

/// `Math/sum(increment<Number>) -> Number`
pub fn function_math_sum(arguments: [Arc<ValueActor>; 1], function_call_id: ConstructId) -> impl Stream<Item = Value> {
    let [argument_increment] = arguments;
    argument_increment
        .subscribe()
        .map(|value| value.expect_number().number())
        .scan(0., |sum, number| {
            *sum += number;
            future::ready(Some(*sum))
        })
        .map(move |sum| {
            Number::new_value(
                ConstructInfo::new(
                    function_call_id.with_child_id(0), 
                    "Math/sum(..) -> Number"
                ),
                sum
            )
        })
}

/// `Timer/interval(duration<Duration[seconds<Number> | milliseconds<Number>]>) -> []`
pub fn function_timer_interval(arguments: [Arc<ValueActor>; 1], function_call_id: ConstructId) -> impl Stream<Item = Value> {
    let [argument_duration] = arguments;
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
