use std::future;
use std::sync::Arc;

use zoon::{println, eprintln};
use zoon::futures_channel::oneshot;
use zoon::futures_util::stream::{self, StreamExt};
use zoon::Timer;
use zoon::{El, Element, HookableLifecycle};

use crate::runtime::{element_helper::*, engine::*};

pub async fn run() -> impl Element {
     let program = include_str!("examples/interval.bn");

    let function_document_new = |arguments: [Arc<ValueActor>; 1], function_call_id: ConstructId| {
        let [argument_root] = arguments;
        Object::new_constant(
            ConstructInfo::new(function_call_id.with_child_id(0), "Document/new output object"),
            [
                Variable::new_arc(
                    ConstructInfo::new(function_call_id.with_child_id(1), "Document/new output root_element"),
                    "root_element",
                    argument_root
                )
            ]
        )
    };

    let function_math_sum = |arguments: [Arc<ValueActor>; 1], function_call_id: ConstructId| {
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
                        "sum"
                    ),
                    sum
                )
            })
    };

    let function_timer_interval = |arguments: [Arc<ValueActor>; 1], function_call_id: ConstructId| {
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
                            ConstructInfo::new(function_call_id.with_child_id(0), "Timer/interval output object"),
                            []
                        );
                        Some((output_value, function_call_id))
                    }
                })
            })
    };

    let root_object = Object::new_arc(
        ConstructInfo::new(0, "root"),
        [
            Variable::new_arc(
                ConstructInfo::new(1, "document"),
                "document",
                TaggedObject::new_arc_value_actor(
                    ConstructInfo::new(6, "Duration[..]"),
                    RunDuration::Nonstop,
                    "Duration",
                    [
                        Variable::new_arc(
                            ConstructInfo::new(7, "Duration.seconds"), 
                            "seconds", 
                            Number::new_arc_value_actor(
                                ConstructInfo::new(8, "Duration.seconds number"),
                                RunDuration::Nonstop,
                                1
                            )
                        )
                    ]
                ).pipe_to(|piped| {
                    FunctionCall::new_arc_value_actor(
                        ConstructInfo::new(5, "Timer/interval(..)"),
                        RunDuration::Nonstop,
                        function_timer_interval,
                        [
                            piped
                        ]
                    ).pipe_to(|piped| {
                        ThenCombinator::new_arc_value_actor(
                            ConstructInfo::new(4, "THEN"),
                            RunDuration::Nonstop,
                            piped,
                            || Number::new_constant(
                                ConstructInfo::new(9, "number 1"),
                                1,
                            )
                        ).pipe_to(|piped| {
                            FunctionCall::new_arc_value_actor(
                                ConstructInfo::new(3, "Math/sum(..)"), 
                                RunDuration::Nonstop,
                                function_math_sum,
                                [
                                    piped
                                ]
                            ).pipe_to(|piped| {
                                FunctionCall::new_arc_value_actor(
                                    ConstructInfo::new(2, "Document/new(..)"),
                                    RunDuration::Nonstop,
                                    function_document_new,
                                    [
                                        piped
                                    ]
                                )
                            })
                        })
                    })
                })
            )
    ]);

    El::new()
        .child_signal(root_object_to_element_signal(root_object.clone()))
        .after_remove(move |_| drop(root_object))
}
