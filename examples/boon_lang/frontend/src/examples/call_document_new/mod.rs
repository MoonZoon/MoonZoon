use crate::boon::platform::browser::preludes::for_generated_code::{println, *};

use crate::boon::parser::{parser, Parser};

#[allow(dead_code)]
pub async fn run() -> Arc<Object> {
    let program = include_str!("call_document_new.bn");
    println!("{program}");

    println!("{:#?}", parser().parse(program));

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
                        Number::new_arc_value_actor(
                            ConstructInfo::new(8, "A number"),
                            RunDuration::Nonstop,
                            123,
                        )
                    ]
                )
            )
        ]
    )
}
