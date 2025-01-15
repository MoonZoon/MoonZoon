mod engine;
use engine::*;

mod element_helper;
use element_helper::*;

type ArgumentName = &'static str;

fn stream_one<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item))
}

pub async fn run(_program: &str) -> impl Element {

    let function_document_new = |arguments: Arguments, function_actor_id: ActorId| {
        stream_one(ObjectActor::new(
            "Document/new output object", 
            function_actor_id.push_child_id(32),
            stream_one(vec![
                VariableActor::new(
                    "Document/new output root_element", 
                    function_actor_id.push_child_id(33), 
                    "root_element",
                ),
                arguments.get_expected_variable_actor("root").actor_stream()
            ])
        ))
    };

    let function_element_stripe = |arguments: Arguments, function_actor_id: ActorId| {
        stream_one(ObjectActor::new(
            "Element/stripe output object", 
            function_actor_id.push_child_id(34),
            stream_one(vec![
                VariableActor::new(
                    "Element/stripe output type", 
                    function_actor_id.push_child_id(36), 
                    "type",
                    TagActor::new("Element/stripe output type tag", 37, stream_one("Stripe"))
                ),
                VariableActor::new(
                    "Element/stripe output settings", 
                    function_actor_id.push_child_id(35), 
                    "settings",
                    stream_one(vec![
                        arguments.get_expected_variable_actor("direction"),
                        arguments.get_expected_variable_actor("style"),
                        arguments.get_expected_variable_actor("items"),
                    ])
                ),
                VariableActor::new(
                    "Element/button output event", 
                    function_actor_id.push_child_id(44), 
                    "event",
                    arguments.get_expected_variable_actor("element").actor_stream().map(|actor| {
                        actor.expect_object_actor().get_variable_actor_or_unset("event")
                    })
                ),
            ])
        ))
    };

    let function_element_button = |arguments: Arguments, function_actor_id: ActorId| {
        stream_one(ObjectActor::new(
            "Element/button output object", 
            38,
            stream_one(vec![
                VariableActor::new(
                    "Element/button output type", 
                    function_actor_id.push_child_id(40), 
                    "type",
                    TagActor::new("Element/button output type tag", 41, stream_one("Button"))
                ),
                VariableActor::new(
                    "Element/button output settings", 
                    function_actor_id.push_child_id(39), 
                    "settings",
                    stream_one(ObjectActor::new(
                        "Element/button output settings object", 
                        function_actor_id.push_child_id(45),
                        stream_one(vec![
                            arguments.get_expected_variable_actor("style"),
                            arguments.get_expected_variable_actor("label"),
                        ])
                    ))
                ),
                VariableActor::new(
                    "Element/button output event", 
                    function_actor_id.push_child_id(46), 
                    "event",
                    arguments.get_expected_variable_actor("element").actor_stream().map(|actor| {
                        actor.expect_object_actor().get_variable_actor_or_unset("event")
                    })
                ),
            ])
        ))
    };

    let function_math_sum = |arguments: Arguments, function_actor_id: ActorId| {
        stream_one(NumberActor::new(
            "counter default number", 
            function_actor_id.push_child_id(43),
            arguments
                .get_expected_variable_actor("increment")
                .actor_stream()
                .flat_map(|actor| {
                    actor.expect_number_actor().number_stream()
                })
                .scan(0, |state, increment| {
                    *state += increment;
                    future::ready(Some(*state))
                })
        ))
    };

    let increment_button_event_press_to_counter_then_variable_reference_actor_16 = VariableReferenceActor::new("counter button press", 16, "increment_button.event.press");
    let counter_to_document_element_stripe_item_0_variable_reference_actor_47 =  VariableReferenceActor::new("document Element/stripe item 0", 47, "counter", 13);
    let increment_button_to_document_element_stripe_item_1_variable_reference_actor_48 = VariableReferenceActor::new("document Element/stripe item 1", 48, "increment_button", 20);

    let root_actor = ObjectActor::new(
        "root", 
        0,
        stream_one(vec![
            VariableActor::new(
                "document", 
                1, 
                "document",
                stream_one(FunctionActor::new(
                    "Document/new call", 
                    2, 
                    "Document/new", 
                    function_document_new,
                    vec![
                        VariableActor::new(
                            "Document/new root", 
                            3, 
                            "root",
                            stream_one(
                                FunctionActor::new(
                                    "Element/stripe call", 
                                    4, 
                                    "Element/stripe", 
                                    function_element_stripe,
                                    vec![
                                        VariableActor::new(
                                            "Element/stripe element", 
                                            5, 
                                            "element",
                                            stream_one(ObjectActor::new("Element/stripe element object", 6, stream_one(vec![])))
                                        ),
                                        VariableActor::new(
                                            "Element/stripe direction", 
                                            7, 
                                            "direction",
                                            stream_one(TagActor::new("Element/stripe direction tag", 8, stream_one("Column")))
                                        ),
                                        VariableActor::new(
                                            "Element/stripe style", 
                                            9, 
                                            "style",
                                            stream_one(ObjectActor::new("Element/stripe style object", 10, stream_one(vec![])))
                                        ),
                                        VariableActor::new(
                                            "Element/stripe items", 
                                            11, 
                                            "items",
                                            stream_one(ListActor::new(
                                                "Element/stripe items list", 
                                                12,
                                                stream_one(vec![
                                                    counter_to_document_element_stripe_item_0_variable_reference_actor_47.actor_stream(),
                                                    increment_button_to_document_element_stripe_item_1_variable_reference_actor_48.actor_stream(),
                                                ])
                                            ))
                                        )
                                    ]
                                ))
                        )
                    ]
                ))
            ),
            VariableActor::new(
                "counter", 
                13, 
                "counter",
                stream_one(LatestActor::new(
                    "counter latest", 
                    14,
                    vec![
                        NumberActor::new("counter default number", 15, stream_one(0.)),
                        FunctionActor::new(
                            "Math/sum call", 
                            19, 
                            "Math/sum", 
                            function_math_sum,
                            vec![
                                VariableActor::new(
                                    "Math/sum increment", 
                                    42, 
                                    "increment",
                                    stream_one(ThenActor::new(
                                        "counter button press then", 
                                        17,
                                        increment_button_event_press_to_counter_then_variable_reference_actor_16.actor_stream(),
                                        stream_one(NumberActor::new("counter after button press number", 18, stream_one(1.)),
                                    )))
                                )
                            ]
                        )
                    ]
                ))
            )
            .pass_as_reference_root(counter_to_document_element_stripe_item_0_variable_reference_actor_47),
            VariableActor::new(
                "increment_button", 
                20, 
                "increment_button",
                stream_one(FunctionActor::new(
                    "Element/button call", 
                    21, 
                    "Element/button", 
                    function_element_button,
                    vec![
                        VariableActor::new(
                            "Element/button argument element", 
                            22, 
                            "element",
                            stream_one(ObjectActor::new(
                                "Element/button element object", 
                                23,
                                stream_one(vec![
                                    VariableActor::new(
                                        "Element/button element event", 
                                        24, 
                                        "event",
                                        stream_one(
                                            ObjectActor::new(
                                                "Element/button element event object", 
                                                25,
                                                stream_one(vec![
                                                    VariableActor::new(
                                                        "Element/button element event press", 
                                                        26, 
                                                        "press",
                                                        stream_one(LinkActor::new("Element/button element event press link", 27))
                                                    )
                                                ])
                                            )
                                        )
                                    )
                                ])
                            ))
                        ),
                        VariableActor::new(
                            "Element/button style", 
                            28, 
                            "style",
                            stream_one(ObjectActor::new("Element/button style object", 29, stream_one(vec![])))
                        ),
                        VariableActor::new(
                            "Element/button label", 
                            30, 
                            "label",
                            stream_one(TextActor::new("Element/button label text", 31, stream_one("+")))
                        )
                    ]
                ))
            )
            .pass_as_reference_root(increment_button_event_press_to_counter_then_variable_reference_actor_16)
            .pass_as_reference_root(increment_button_to_document_element_stripe_item_1_variable_reference_actor_48)
        ])
    );

    root_actor_to_element(root_actor).await
}
