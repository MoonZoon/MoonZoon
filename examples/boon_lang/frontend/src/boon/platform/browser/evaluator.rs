use std::sync::Arc;

use super::super::super::parser::{self, Expression, Spanned, ParseError, Token};
use super::api;
use super::engine::*;

type EvaluateResult<'code, T> = Result<Arc<T>, ParseError<'code, Token<'code>>>;

pub fn evaluate(expressions: Vec<Spanned<Expression>>) -> EvaluateResult<Object> {
    Ok(Object::new_arc(
        ConstructInfo::new(0, "root"),
        expressions.into_iter().map(|Spanned { span, node: expression }| {
            match expression {
                Expression::Variable(variable) => {
                    expression_variable_into_engine_variable(variable)
                }
                Expression::Function { name, parameters, body } => {
                    // @TODO implement
                    todo!("Function definitions are not supported yet")
                }
                _ => Err(ParseError::custom(span, "Only variables or functions expected"))
            }
        }).collect::<Result<Vec<_>, _>>()?,
    ))
}

fn expression_variable_into_engine_variable(variable: Box<parser::Variable>) -> EvaluateResult<Variable> {
    Ok(Variable::new_arc(
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
    ))
}

// fn expression_into_value_actor(expression: Spanned<Expression>) -> Arc<ValueActor> {

// }
