use zoon::{*, println};
use zoon::futures_util::{stream, future};
use std::sync::Arc;

mod engine;
use engine::*;

type ArgumentName = &'static str;

fn stream_one<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item))
}

pub async fn run(_program: &str) -> impl Element {

    let function_document_new = |arguments: Arguments| {
        stream_one(ObjectActor::new(
            "Document/new output object", 
            32,
            stream_one(vec![
                VariableActor::new(
                    "Document/new output root_element", 
                    33, 
                    "root_element",
                ),
                arguments.get_expected_variable_actor("root").into_actor_stream()
            ])
        ))
    };

    let function_element_stripe = |arguments: Arguments| {
        stream_one(ObjectActor::new(
            "Element/stripe output object", 
            34,
            stream_one(vec![
                VariableActor::new(
                    "Element/stripe output type", 
                    36, 
                    "type",
                    TagActor::new("Element/stripe output type tag", 37, stream_one("Stripe"))
                ),
                VariableActor::new(
                    "Element/stripe output settings", 
                    35, 
                    "settings",
                    stream_one(vec![
                        arguments.get_expected_variable_actor("direction"),
                        arguments.get_expected_variable_actor("style"),
                        arguments.get_expected_variable_actor("items"),
                    ])
                ),
                VariableActor::new(
                    "Element/button output event", 
                    44, 
                    "event",
                    arguments.get_expected_variable_actor("element").actor_stream().map(|actor| {
                        actor.expect_object_actor().get_variable_actor_or_unset("event")
                    })
                ),
            ])
        ))
    };

    let function_element_button = |arguments: Arguments| {
        stream_one(ObjectActor::new(
            "Element/button output object", 
            38,
            stream_one(vec![
                VariableActor::new(
                    "Element/button output type", 
                    40, 
                    "type",
                    TagActor::new("Element/button output type tag", 41, stream_one("Button"))
                ),
                VariableActor::new(
                    "Element/button output settings", 
                    39, 
                    "settings",
                    stream_one(ObjectActor::new(
                        "Element/button output settings object", 
                        45,
                        stream_one(vec![
                            arguments.get_expected_variable_actor("style"),
                            arguments.get_expected_variable_actor("label"),
                        ])
                    ))
                ),
                VariableActor::new(
                    "Element/button output event", 
                    46, 
                    "event",
                    arguments.get_expected_variable_actor("element").actor_stream().map(|actor| {
                        actor.expect_object_actor().get_variable_actor_or_unset("event")
                    })
                ),
            ])
        ))
    };

    let function_math_sum = |arguments: Arguments| {
        stream_one(NumberActor::new(
            "counter default number", 
            43,
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
                                                    stream_one(ReferenceActor::new("document Element/stripe item 0", 47, "counter", 13)),
                                                    stream_one(ReferenceActor::new("document Element/stripe item 1", 48, "increment_button", 20))
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
                                    // @TODO think through
                                    stream_one(ThenActor::new(
                                        "counter button press then", 
                                        17,
                                        stream_one(ReferenceActor::new("counter button press", 16, "increment_button.event.press", 20)),
                                        stream_one(NumberActor::new("counter after button press number", 18, stream_one(1.)),
                                    )))
                                )
                            ]
                        )
                    ]
                ))
            ),
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
        ])
    );

    El::new().child("3. attempt")
}
