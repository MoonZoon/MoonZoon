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
                    Number::new_arc_value_actor(
                        ConstructInfo::new(11, "Number 62"), 
                        RunDuration::Nonstop, 
                        62
                    ),
                );
                if let Err(error) = counter_variable_sender.unbounded_send(variable.clone()) {
                    panic!("Failed to send variable through `counter_variable_sender` channel:  {error}");
                }
                variable
            },
        ]
    )
}
