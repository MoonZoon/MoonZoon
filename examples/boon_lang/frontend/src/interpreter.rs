mod engine;
use engine::*;

mod element_helper;
use element_helper::*;

type ArgumentName = &'static str;

fn stream_one<T>(item: T) -> impl Stream<Item = T> {
    stream::once(future::ready(item))
}

pub async fn run(_program: &str) -> impl Element {

    let function_document_new = |arguments: Object, function_call_id: ConstructId| {
        stream_one(ObjectValue::new(
            "Document/new output object", 
            function_call_id.push_child_id(32),
            stream_one(Object::new([
                Variable::new(
                    "Document/new output root_element", 
                    function_call_id.push_child_id(33), 
                    "root_element",
                ),
                arguments.get_expected_variable("root").value_stream()
            ]))
        ))
    };

    let function_element_stripe = |arguments: Object, function_call_id: ConstructId| {
        stream_one(TaggedObjectValue::new(
            "Element/stripe output object", 
            function_call_id.push_child_id(34),
            "Element",
            stream_one(Object::new([
                Variable::new(
                    "Element/stripe output type", 
                    function_call_id.push_child_id(36), 
                    "type",
                    stream_one(TagValue::new("Element/stripe output type tag", 37, stream_one("Stripe")))
                ),
                Variable::new(
                    "Element/stripe output settings", 
                    function_call_id.push_child_id(35), 
                    "settings",
                    stream_one(Object::new([
                        arguments.get_expected_variable("direction"),
                        arguments.get_expected_variable("style"),
                        arguments.get_expected_variable("items"),
                    ]))
                ),
                Variable::new(
                    "Element/button output event", 
                    function_call_id.push_child_id(44), 
                    "event",
                    arguments.get_expected_variable("element").value_stream().flat_map(|value| {
                        value
                            .expect_object_value()
                            .object_stream()
                            .flat_map(|object| {
                                object
                                    .get_variable("event")
                                    .map(|variable| variable.value_stream().boxed())
                                    .or_else(|| stream_one(Object::new([])).boxed())
                            })
                    })
                ),
            ]))
        ))
    };

    let function_element_button = |arguments: Object, function_call_id: ConstructId| {
        stream_one(TaggedObjectValue::new(
            "Element/button output object", 
            function_call_id.push_child_id(38),
            "Element",
            stream_one(Object::new([
                Variable::new(
                    "Element/button output type", 
                    function_call_id.push_child_id(40), 
                    "type",
                    stream_one(TagValue::new("Element/button output type tag", 41, stream_one("Button")))
                ),
                Variable::new(
                    "Element/button output settings", 
                    function_call_id.push_child_id(39), 
                    "settings",
                    stream_one(ObjectValue::new(
                        "Element/button output settings object", 
                        function_call_id.push_child_id(45),
                        stream_one(Object::new([
                            arguments.get_expected_variable("style"),
                            arguments.get_expected_variable("label"),
                        ]))
                    ))
                ),
                Variable::new(
                    "Element/button output event", 
                    function_call_id.push_child_id(46), 
                    "event",
                    arguments.get_expected_variable("element").value_stream().flat_map(|value| {
                        value
                            .expect_object_value()
                            .object_stream()
                            .flat_map(|object| {
                                object
                                    .get_variable("event")
                                    .map(|variable| variable.value_stream().boxed())
                                    .or_else(|| stream_one(Object::new([])).boxed())
                            })
                    })
                ),
            ]))
        ))
    };

    let function_math_sum = |arguments: Object, function_call_id: ConstructId| {
        stream_one(NumberValue::new(
            "counter default number", 
            function_call_id.push_child_id(43),
            arguments
                .get_expected_variable("increment")
                .value_stream()
                .flat_map(|value| {
                    value.expect_number_value().number_stream()
                })
                .scan(0, |state, increment| {
                    *state += increment;
                    future::ready(Some(*state))
                })
        ))
    };

    let increment_button_event_press_to_counter_then_variable_reference_16 = VariableReference::new("counter button press", 16, "increment_button.event.press");
    let counter_to_document_element_stripe_item_0_variable_reference_47 =  VariableReference::new("document Element/stripe item 0", 47, "counter");
    let increment_button_to_document_element_stripe_item_1_variable_reference_48 = VariableReference::new("document Element/stripe item 1", 48, "increment_button");

    let root_object_value = ObjectValue::new(
        "root", 
        0,
        stream_one(Object::new([
            Variable::new(
                "document", 
                1, 
                "document",
                stream_one(FunctionCall::new(
                    "Document/new call", 
                    2, 
                    "Document/new", 
                    function_document_new,
                    Object::new([
                        Variable::new(
                            "Document/new root", 
                            3, 
                            "root",
                            stream_one(
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
                                            stream_one(TagValue::new("Element/stripe direction tag", 8, stream_one("Column")))
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
                                                    counter_to_document_element_stripe_item_0_variable_reference_47,
                                                    increment_button_to_document_element_stripe_item_1_variable_reference_48,
                                                ]))
                                            ))
                                        )
                                    ])
                                ))
                        )
                    ])
                ))
            ),
            Variable::new(
                "counter", 
                13, 
                "counter",
                stream_one(LatestOperator::new(
                    "counter latest", 
                    14,
                    FixedList::new([
                        NumberValue::new("counter default number", 15, stream_one(0.)),
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
                                    stream_one(ThenOperator::new(
                                        "counter button press then", 
                                        17,
                                        increment_button_event_press_to_counter_then_variable_reference_16.value_stream(),
                                        stream_one(NumberValue::new("counter after button press number", 18, stream_one(1.)),
                                    )))
                                )
                            ])
                        )
                    ])
                ))
            )
            .pass_as_reference_root(counter_to_document_element_stripe_item_0_variable_reference_47),
            Variable::new(
                "increment_button", 
                20, 
                "increment_button",
                stream_one(FunctionCall::new(
                    "Element/button call", 
                    21, 
                    "Element/button", 
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
                            stream_one(TextValue::new("Element/button label text", 31, stream_one("+")))
                        )
                    ])
                ))
            )
            .pass_as_reference_root(increment_button_event_press_to_counter_then_variable_reference_16)
            .pass_as_reference_root(increment_button_to_document_element_stripe_item_1_variable_reference_48)
        ]))
    );

    El::new().child_signal(root_object_value_to_element_signal(root_object_value))
}
