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
            ),
            VariableActor::new(
                "counter", 
                13, 
                "counter",
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


    let document_new_function_actor_2 = FunctionActor::new("Document/new call", 2, "Document/new", function_document_new);

    let root_argument_actor_3 = ArgumentActor::new("Document/new root", 3, "root");

    let element_stripe_function_actor_4 = FunctionActor::new("Element/stripe call", 4, "Element/stripe", function_element_stripe);

    let element_stripe_element_argument_actor_5 = ArgumentActor::new("Element/stripe element", 5, "element");

    let element_stripe_element_object_actor_6 = ObjectActor::new("Element/stripe element object", 6);

    let element_stripe_direction_argument_actor_7 = ArgumentActor::new("Element/stripe direction", 7, "direction");

    let element_stripe_direction_tag_actor_8 = TagActor::new("Element/stripe direction tag", 8, stream_one("Column"));

    let element_stripe_style_argument_actor_9 = ArgumentActor::new("Element/stripe style", 9, "style");

    let element_stripe_style_object_actor_10 = ObjectActor::new("Element/stripe style object", 10);

    let element_stripe_items_argument_actor_11 = ArgumentActor::new("Element/stripe items", 11, "items");

    let element_stripe_items_list_actor_12 = ListActor::new("Element/stripe items list", 12);



    let counter_latest_actor_14 = LatestActor::new("counter latest", 14);

    let counter_default_number_actor_15 = NumberActor::new("counter default number", 15, stream_one(0.));

    let counter_button_press_reference_actor_16 = ReferenceActor::new("counter button press", 16, "increment_button.event.press");

    let counter_button_press_then_actor_17 = ThenActor::new("counter button press then", 17);

    let counter_after_button_press_number_actor_18 = NumberActor::new("counter after button press number", 18, stream_one(1.));

    let counter_math_sum_function_actor_19 = FunctionActor::new("Math/sum call", 19, "Math/sum", function_math_sum);

    let counter_math_sum_increment_argument_actor_42 = ArgumentActor::new("Math/sum increment", 42, "increment");



    El::new().child("3. attempt")
}
