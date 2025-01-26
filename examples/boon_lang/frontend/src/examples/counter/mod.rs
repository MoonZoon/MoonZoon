use crate::boon::platform::browser::preludes::for_generated_code::{*, println};

#[allow(dead_code)]
pub async fn run() -> Arc<Object> {
    let program = include_str!("counter.bn");
    println!("{program}");

    let (counter_variable_sender, counter_variable_receiver) = mpsc::unbounded();

    Object::new_arc(
        ConstructInfo::new(0, "root"),
        [
            Variable::new_arc(
                ConstructInfo::new(1, "document"),
                "document",
                FunctionCall::new_arc_value_actor(
                    ConstructInfo::new(2, "Document/new(..)"),
                    RunDuration::Nonstop,
                    function_document_new,
                    [
                        FunctionCall::new_arc_value_actor(
                            ConstructInfo::new(3, "Element/stripe(..)"),
                            RunDuration::Nonstop,
                            function_element_stripe,
                            [
                                Object::new_arc_value_actor(
                                    ConstructInfo::new(4, "Element/stripe(element)"), 
                                    RunDuration::Nonstop, 
                                    []
                                ),
                                Tag::new_arc_value_actor(
                                    ConstructInfo::new(5, "Element/stripe(direction)"),
                                    RunDuration::Nonstop,
                                    "Column",
                                ),
                                Object::new_arc_value_actor(
                                    ConstructInfo::new(6, "Element/stripe(style)"), 
                                    RunDuration::Nonstop, 
                                    []
                                ),
                                List::new_arc_value_actor(
                                    ConstructInfo::new(7, "Element/stripe(items)"), 
                                    RunDuration::Nonstop, 
                                    [
                                        VariableReference::new_arc_value_actor(
                                            ConstructInfo::new(8, "counter reference"), 
                                            RunDuration::Nonstop, 
                                            "counter",
                                            counter_variable_receiver,
                                        ),
                                    ]
                                ),
                            ]
                        )
                    ]
                )
            ),
            { 
                let variable = Variable::new_arc(
                    ConstructInfo::new(10, "counter"),
                    "counter",
                    LatestCombinator::new_arc_value_actor(
                        ConstructInfo::new(11, "counter LATEST"), 
                        RunDuration::Nonstop,
                        [
                            Number::new_arc_value_actor(
                                ConstructInfo::new(12, "default counter number"),
                                RunDuration::Nonstop,
                                0
                            ),
                            TaggedObject::new_arc_value_actor(
                                ConstructInfo::new(13, "Duration[..]"),
                                RunDuration::Nonstop,
                                "Duration",
                                [
                                    Variable::new_arc(
                                        ConstructInfo::new(14, "Duration.seconds"), 
                                        "seconds", 
                                        Number::new_arc_value_actor(
                                            ConstructInfo::new(15, "Duration.seconds number"),
                                            RunDuration::Nonstop,
                                            1
                                        )
                                    )
                                ]
                            ).pipe_to(|piped| {
                                FunctionCall::new_arc_value_actor(
                                    ConstructInfo::new(16, "Timer/interval(..)"),
                                    RunDuration::Nonstop,
                                    function_timer_interval,
                                    [
                                        piped
                                    ]
                                ).pipe_to(|piped| {
                                    ThenCombinator::new_arc_value_actor(
                                        ConstructInfo::new(17, "THEN"),
                                        RunDuration::Nonstop,
                                        piped,
                                        || Number::new_constant(
                                            ConstructInfo::new(18, "Number 1"),
                                            1,
                                        )
                                    ).pipe_to(|piped| {
                                        FunctionCall::new_arc_value_actor(
                                            ConstructInfo::new(19, "Math/sum(..)"), 
                                            RunDuration::Nonstop,
                                            function_math_sum,
                                            [
                                                piped
                                            ]
                                        )
                                    })
                                })
                            })
                        ]
                    )
                );
                if let Err(error) = counter_variable_sender.unbounded_send(variable.clone()) {
                    panic!("Failed to send variable through `counter_variable_sender` channel:  {error}");
                }
                variable
            },
        ]
    )
}
