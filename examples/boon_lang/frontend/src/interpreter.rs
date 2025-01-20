use std::future;

use zoon::{*, eprintln};
use zoon::futures_channel::oneshot;

mod engine;
use engine::*;

mod element_helper;
use element_helper::*;

pub async fn run(_program: &str) -> impl Element {
    let function_document_new = |mut arguments: Object, function_call_id: ConstructId| {
        stream_one(ObjectValue::new(
            "Document/new output object", 
            function_call_id.push_child_id(32),
            stream_one(Object::new([
                Variable::new(
                    "Document/new output root_element", 
                    function_call_id.push_child_id(33), 
                    "root_element",
                    arguments.take_expected_variable("root")
                ),
            ]))
        ))
    };

    let function_element_stripe = |mut arguments: Object, function_call_id: ConstructId| {
        stream_one(TaggedObjectValue::new(
            "Element/stripe output object", 
            function_call_id.push_child_id(34),
            stream_one(("Element", Object::new([
                Variable::new(
                    "Element/stripe output type", 
                    function_call_id.push_child_id(36), 
                    "type",
                    stream_one(TagValue::new("Element/stripe output type tag", 37, stream_one(String::from("Stripe"))))
                ),
                Variable::new(
                    "Element/stripe output settings", 
                    function_call_id.push_child_id(35), 
                    "settings",
                    stream_one(ObjectValue::new(
                        "Element_stripe output object", 
                        2000, 
                        stream_one(Object::new([
                            arguments.take_expected_variable("direction"),
                            arguments.take_expected_variable("style"),
                            arguments.take_expected_variable("items"),
                        ]))
                    ))
                ),
                Variable::new(
                    "Element/button output event", 
                    function_call_id.push_child_id(44), 
                    "event",
                    arguments.take_expected_variable("element").flat_map(|value| {
                        value
                            .expect_object_value()
                            .flat_map(|mut object| {
                                object
                                    .take_variable("event")
                                    .map(|variable| variable.boxed())
                                    .unwrap_or_else(|| {
                                        stream_one(Value::from(ObjectValue::new(
                                            "Element/button output event default object",
                                            1234,
                                            stream_one(Object::new([]))
                                        ))).boxed()
                                    })
                            })
                    })
                ),
            ])))
        ))
    };

    let function_element_button = |mut arguments: Object, function_call_id: ConstructId| {
        stream_one(TaggedObjectValue::new(
            "Element/button output object", 
            function_call_id.push_child_id(38),
            stream_one(("Element", Object::new([
                Variable::new(
                    "Element/button output type", 
                    function_call_id.push_child_id(40), 
                    "type",
                    stream_one(TagValue::new("Element/button output type tag", 41, stream_one(String::from("Button"))))
                ),
                Variable::new(
                    "Element/button output settings", 
                    function_call_id.push_child_id(39), 
                    "settings",
                    stream_one(ObjectValue::new(
                        "Element/button output settings object", 
                        function_call_id.push_child_id(45),
                        stream_one(Object::new([
                            arguments.take_expected_variable("style"),
                            arguments.take_expected_variable("label"),
                        ]))
                    ))
                ),
                Variable::new(
                    "Element/button output event", 
                    function_call_id.push_child_id(46), 
                    "event",
                    arguments.take_expected_variable("element").flat_map(|value| {
                        value
                            .expect_object_value()
                            .flat_map(|mut object| {
                                object
                                    .take_variable("event")
                                    .map(|variable| variable.boxed())
                                    .unwrap_or_else(|| stream_one(Value::from(ObjectValue::new(
                                        "empty event object value",
                                        1000,
                                        stream_one(Object::new([]))
                                    ))).boxed())
                            })
                    })
                ),
            ])))
        ))
    };

    let function_math_sum = |mut arguments: Object, function_call_id: ConstructId| {
        stream_one(NumberValue::new(
            "counter default number", 
            function_call_id.push_child_id(43),
            arguments
                .take_expected_variable("increment")
                .flat_map(|value| value.expect_number_value())
                .scan(0., |state, increment| {
                    *state += increment;
                    future::ready(Some(*state))
                })
        ))
    };

    let (increment_button_event_press_to_counter_then_variable_reference_16_sender, increment_button_event_press_to_counter_then_variable_reference_16_receiver) = oneshot::channel(); 
    let increment_button_event_press_to_counter_then_variable_reference_16 = VariableReference::new(
        "counter button press", 
        16, 
        "increment_button.event.press",
        increment_button_event_press_to_counter_then_variable_reference_16_receiver 
    );

    let (counter_to_document_element_stripe_item_0_variable_reference_47_sender, counter_to_document_element_stripe_item_0_variable_reference_47_receiver) = oneshot::channel(); 
    let counter_to_document_element_stripe_item_0_variable_reference_47 =  VariableReference::new(
        "document Element/stripe item 0", 
        47, 
        "counter",
        counter_to_document_element_stripe_item_0_variable_reference_47_receiver
    );

    let (increment_button_to_document_element_stripe_item_1_variable_reference_48_sender, increment_button_to_document_element_stripe_item_1_variable_reference_48_receiver) = oneshot::channel(); 
    let increment_button_to_document_element_stripe_item_1_variable_reference_48 = VariableReference::new(
        "document Element/stripe item 1", 
        48, 
        "increment_button",
        increment_button_to_document_element_stripe_item_1_variable_reference_48_receiver
    );

    let root_object_value = ObjectValue::new(
        "root", 
        0,
        stream_one(Object::new([
            Variable::new(
                "document", 
                1, 
                "document",
                FunctionCall::new(
                    "Document/new call", 
                    2, 
                    "Document/new", 
                    function_document_new,
                    Object::new([
                        Variable::new(
                            "Document/new root", 
                            3, 
                            "root",
                            FunctionCall::new(
                                "Element/stripe call", 
                                4, 
                                "Element/stripe", 
                                function_element_stripe,
                                Object::new([
                                    Variable::new(
                                        "Element/stripe element", 
                                        5, 
                                        "element",
                                        stream_one(ObjectValue::new("Element/stripe element object", 6, stream_one(Object::new([]))))
                                    ),
                                    Variable::new(
                                        "Element/stripe direction", 
                                        7, 
                                        "direction",
                                        stream_one(TagValue::new("Element/stripe direction tag", 8, stream_one(String::from("Column"))))
                                    ),
                                    Variable::new(
                                        "Element/stripe style", 
                                        9, 
                                        "style",
                                        stream_one(ObjectValue::new("Element/stripe style object", 10, stream_one(Object::new([]))))
                                    ),
                                    Variable::new(
                                        "Element/stripe items", 
                                        11, 
                                        "items",
                                        stream_one(ListValue::new(
                                            "Element/stripe items list", 
                                            12,
                                            stream_one(List::new([
                                                CloneableStream::new(counter_to_document_element_stripe_item_0_variable_reference_47),
                                                CloneableStream::new(increment_button_to_document_element_stripe_item_1_variable_reference_48),
                                            ]))
                                        ))
                                    )
                                ])
                            )
                            
                        )
                    ])
                )
                
            ),
            {
                let variable = Variable::new(
                    "counter", 
                    13, 
                    "counter",
                    LatestCombinator::new(
                        "counter latest", 
                        14,
                        FixedList::new([
                            stream_one(Value::from(NumberValue::new("counter default number", 15, stream_one(0.)))).boxed(),
                            FunctionCall::new(
                                "Math/sum call", 
                                19, 
                                "Math/sum", 
                                function_math_sum,
                                Object::new([
                                    Variable::new(
                                        "Math/sum increment", 
                                        42, 
                                        "increment",
                                        ThenCombinator::new(
                                            "counter button press then", 
                                            17,
                                            increment_button_event_press_to_counter_then_variable_reference_16,
                                            stream_one(NumberValue::new("counter after button press number", 18, stream_one(1.)),
                                        ))
                                    )
                                ])
                            )
                            .boxed()
                        ])
                    )
                );
                if counter_to_document_element_stripe_item_0_variable_reference_47_sender.send(variable.clone()).is_err() {
                    eprintln!("Failed to send Variable to VariableReference")
                }
                variable
            },
            {
                let variable = Variable::new(
                    "increment_button", 
                    20, 
                    "increment_button",
                    FunctionCall::new(
                        "Element/button call", 
                        21, 
                        "Element/button", 
                        function_element_button,
                        Object::new([
                            Variable::new(
                                "Element/button argument element", 
                                22, 
                                "element",
                                stream_one(ObjectValue::new(
                                    "Element/button element object", 
                                    23,
                                    stream_one(Object::new([
                                        Variable::new(
                                            "Element/button element event", 
                                            24, 
                                            "event",
                                            stream_one(
                                                ObjectValue::new(
                                                    "Element/button element event object", 
                                                    25,
                                                    stream_one(Object::new([
                                                        Variable::new(
                                                            "Element/button element event press", 
                                                            26, 
                                                            "press",
                                                            stream_one(LinkValue::new("Element/button element event press link", 27))
                                                        )
                                                    ]))
                                                )
                                            )
                                        )
                                    ]))
                                ))
                            ),
                            Variable::new(
                                "Element/button style", 
                                28, 
                                "style",
                                stream_one(ObjectValue::new("Element/button style object", 29, stream_one(Object::new([]))))
                            ),
                            Variable::new(
                                "Element/button label", 
                                30, 
                                "label",
                                stream_one(TextValue::new("Element/button label text", 31, stream_one(String::from("+"))))
                            )
                        ])
                    )
                    
                );
                if increment_button_event_press_to_counter_then_variable_reference_16_sender.send(variable.clone()).is_err() {
                    eprintln!("Failed to send Variable to VariableReference")
                }
                if increment_button_to_document_element_stripe_item_1_variable_reference_48_sender.send(variable.clone()).is_err() {
                    eprintln!("Failed to send Variable to VariableReference")
                }
                variable
            }
        ]))
    );

    El::new().child_signal(root_object_value_to_element_signal(root_object_value))
}
