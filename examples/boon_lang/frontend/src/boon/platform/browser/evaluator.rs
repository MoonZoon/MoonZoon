use std::sync::Arc;

use super::super::super::parser::{self, Expression, Spanned, ParseError, Token, Span};
use super::api;
use super::engine::*;

type EvaluateResult<'code, T> = Result<Arc<T>, ParseError<'code, Token<'code>>>;

pub fn evaluate(expressions: Vec<Spanned<Expression>>) -> EvaluateResult<Object> {
    Ok(Object::new_arc(
        ConstructInfo::new(0, "root"),
        expressions.into_iter().map(|Spanned { span, node: expression }| {
            match expression {
                Expression::Variable(variable) => {
                    parser_variable_into_engine_variable(variable, span)
                }
                Expression::Function { name, parameters, body } => {
                    // @TODO implement
                    todo!("Function definitions are not supported yet, sorry")
                }
                _ => Err(ParseError::custom(span, "Only variables or functions expected"))
            }
        }).collect::<Result<Vec<_>, _>>()?,
    ))
}

fn parser_variable_into_engine_variable(variable: Box<parser::Variable>, span: Span) -> EvaluateResult<Variable> {
    // @TODO link variable
    Ok(Variable::new_arc(
        ConstructInfo::new(1, format!("{span}; {}", variable.name)),
        variable.name.to_owned(),
        spanned_expression_into_value_actor(variable.value)?
    ))
}

// @TODO resolve ids
// @TODO RunDuration
fn spanned_expression_into_value_actor(expression: Spanned<Expression>) -> EvaluateResult<ValueActor> {
    let Spanned { span, node: expression } = expression;
    let actor = match expression {
        Expression::Variable(variable) => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Literal(literal) => {
            match literal {
                parser::Literal::Number(number) => {
                    Number::new_arc_value_actor(
                        ConstructInfo::new(8, format!("{span}; Number {number}")),
                        RunDuration::Nonstop,
                        number,
                    )
                }
                parser::Literal::Text(text) => {
                    let text = text.to_owned();
                    Text::new_arc_value_actor(
                        ConstructInfo::new(8, format!("{span}; Text {text}")),
                        RunDuration::Nonstop,
                        text,
                    )
                }
                parser::Literal::Tag(tag) => {
                    let tag = tag.to_owned();
                    Tag::new_arc_value_actor(
                        ConstructInfo::new(8, format!("{span}; Tag {tag}")),
                        RunDuration::Nonstop,
                        tag,
                    )
                }
            }
        }
        Expression::List { items } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Object(object) => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::TaggedObject { tag, object } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Map { entries } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Function { name, parameters, body } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::FunctionCall { path, arguments } => {
            // @TODO better argument error handling
            FunctionCall::new_arc_value_actor(
                ConstructInfo::new(2, format!("{span}; {}(..)", path.join("/"))),
                RunDuration::Nonstop,
                api::function_document_new,
                arguments.into_iter().map(|Spanned { span, node: argument }| {
                    let parser::Argument { name, value } = argument;
                    let Some(value) = value else {
                        // @TODO support out arguments
                        Err(ParseError::custom(span, "Out arguments not supported yet, sorry"))?
                    };
                    spanned_expression_into_value_actor(value)
                }).collect::<Result<Vec<_>, _>>()?,
            )
        }
        Expression::Alias(aliast) => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::LinkSetter { alias } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Link => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Latest { inputs } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Then { body } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::When { arms } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::While { arms } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Pipe { from, to } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Skip => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Block { variables, output } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Comparator(comparator) => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::ArithmeticOperator(arithmetic_operator) => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
    };
    Ok(actor)
}


