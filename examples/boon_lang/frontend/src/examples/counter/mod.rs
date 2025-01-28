use crate::boon::platform::browser::preludes::for_generated_code::{*, println};

#[allow(dead_code)]
pub async fn run() -> Arc<Object> {
    let program = include_str!("counter.bn");
    println!("{program}");

    let (counter_variable_sender_0, counter_variable_receiver_0) = mpsc::unbounded();

    let (increment_button_sender_0, increment_button_receiver_0) = mpsc::unbounded();
    let (increment_button_sender_1, increment_button_receiver_1) = mpsc::unbounded();

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
                                            counter_variable_receiver_0,
                                        ),
                                        VariableReference::new_arc_value_actor(
                                            ConstructInfo::new(9, "increment_button reference"), 
                                            RunDuration::Nonstop, 
                                            "increment_button",
                                            increment_button_receiver_0,
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
                            VariableReference::new_arc_value_actor(
                                ConstructInfo::new(9, "increment_button.event.press reference"), 
                                RunDuration::Nonstop, 
                                "increment_button.event.press",
                                increment_button_receiver_1,
                            ).pipe_to(|piped| {
                                ThenCombinator::new_arc_value_actor(
                                    ConstructInfo::new(17, "THEN"),
                                    RunDuration::Nonstop,
                                    piped,
                                    || Number::new_constant(
                                        ConstructInfo::new(18, "Number 1"),
                                        1,
                                    )
                                )
                            })
                        ]
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
                );
                if let Err(error) = counter_variable_sender_0.unbounded_send(variable.clone()) {
                    panic!("Failed to send variable through `counter_variable_sender_0` channel: {error}");
                }
                variable
            },
            { 
                let variable = Variable::new_arc(
                    ConstructInfo::new(20, "increment_button"),
                    "increment_button",
                    FunctionCall::new_arc_value_actor(
                        ConstructInfo::new(21, "Element/button(..)"),
                        RunDuration::Nonstop,
                        function_element_button,
                        [
                            Object::new_arc_value_actor(
                                ConstructInfo::new(22, "Element/button(element)"), 
                                RunDuration::Nonstop, 
                                [
                                    Variable::new_arc(
                                        ConstructInfo::new(20, "Element/button(element: [event])"),
                                        "event",
                                        Object::new_arc_value_actor(
                                            ConstructInfo::new(23, "Element/button(element: [event: [..]])"), 
                                            RunDuration::Nonstop, 
                                            [
                                                Variable::new_link_arc(
                                                    ConstructInfo::new(20, "Element/button(element: [event: [press]])"),
                                                    RunDuration::Nonstop,
                                                    "press",
                                                )
                                            ]
                                        )
                                    )
                                ]
                            ),
                            Object::new_arc_value_actor(
                                ConstructInfo::new(6, "Element/button(style)"), 
                                RunDuration::Nonstop, 
                                []
                            ),
                            Text::new_arc_value_actor(
                                ConstructInfo::new(5, "Element/button(label)"),
                                RunDuration::Nonstop,
                                "+",
                            ),
                        ]
                    )
                );
                if let Err(error) = increment_button_sender_0.unbounded_send(variable.clone()) {
                    panic!("Failed to send variable through `increment_button_sender_0` channel: {error}");
                }
                if let Err(error) = increment_button_sender_1.unbounded_send(variable.clone()) {
                    panic!("Failed to send variable through `increment_button_sender_1` channel: {error}");
                }
                variable
            },
        ]
    )
}
