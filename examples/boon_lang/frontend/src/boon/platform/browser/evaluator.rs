use std::sync::Arc;

use super::super::super::parser;
use super::api;
use super::engine::*;

pub fn evaluate(expression: &parser::Expression) -> Result<Arc<Object>, String> {
    // call_document_new.bn
    // -- Display number on web browser page
    // document: Document/new(root: 123)
    Ok(Object::new_arc(
        ConstructInfo::new(0, "root"),
        [Variable::new_arc(
            ConstructInfo::new(1, "document"),
            "document",
            FunctionCall::new_arc_value_actor(
                ConstructInfo::new(2, "Document/new(..)"),
                RunDuration::Nonstop,
                api::function_document_new,
                [Number::new_arc_value_actor(
                    ConstructInfo::new(8, "A number"),
                    RunDuration::Nonstop,
                    123,
                )],
            ),
        )],
    ))
}
