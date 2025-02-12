use std::sync::Arc;
use std::pin::Pin;
use zoon::{Stream, StreamExt};

use super::super::super::parser::{self, Expression, Spanned, ParseError, Token, Span};
use super::api;
use super::engine::*;

type EvaluateResult<'code, T> = Result<T, ParseError<'code, Token<'code>>>;

pub fn evaluate(expressions: Vec<Spanned<Expression>>) -> EvaluateResult<Arc<Object>> {
    let run_duration = RunDuration::Nonstop;
    Ok(Object::new_arc(
        ConstructInfo::new(0, "root"),
        expressions.into_iter().map(|Spanned { span, node: expression }| {
            match expression {
                Expression::Variable(variable) => {
                    parser_variable_into_engine_variable(variable, span, run_duration)
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

fn parser_variable_into_engine_variable(variable: Box<parser::Variable>, span: Span, run_duration: RunDuration) -> EvaluateResult<Arc<Variable>> {
    // @TODO link variable
    Ok(Variable::new_arc(
        ConstructInfo::new(1, format!("{span}; {}", variable.name)),
        variable.name.to_owned(),
        spanned_expression_into_value_actor(variable.value, run_duration)?
    ))
}

// @TODO resolve ids
fn spanned_expression_into_value_actor(expression: Spanned<Expression>, run_duration: RunDuration) -> EvaluateResult<Arc<ValueActor>> {
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
                        run_duration,
                        number,
                    )
                }
                parser::Literal::Text(text) => {
                    let text = text.to_owned();
                    Text::new_arc_value_actor(
                        ConstructInfo::new(8, format!("{span}; Text {text}")),
                        run_duration,
                        text,
                    )
                }
                parser::Literal::Tag(tag) => {
                    let tag = tag.to_owned();
                    Tag::new_arc_value_actor(
                        ConstructInfo::new(8, format!("{span}; Tag {tag}")),
                        run_duration,
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
                run_duration,
                function_call_path_to_definition(&path, span)?,
                arguments.into_iter().map(|Spanned { span, node: argument }| {
                    let parser::Argument { name, value } = argument;
                    let Some(value) = value else {
                        // @TODO support out arguments
                        Err(ParseError::custom(span, "Out arguments not supported yet, sorry"))?
                    };
                    spanned_expression_into_value_actor(value, run_duration)
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
            Err(ParseError::custom(span, "You have to pipe things into THEN - e.g. `..press |> THEN { .. }`"))?
        }
        Expression::When { arms } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::While { arms } => {
            Err(ParseError::custom(span, "Not supported yet, sorry"))?
        }
        Expression::Pipe { from, to } => {
            pipe(from, to, run_duration)?
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

fn function_call_path_to_definition<'code>(path: &[&'code str], span: Span) -> EvaluateResult<'code, impl Fn(Arc<Vec<Arc<ValueActor>>>, ConstructId) -> Pin<Box<dyn Stream<Item = Value>>>> {
    let definition = match path {
        ["Document", "new"] => |arguments, id| api::function_document_new(arguments, id).boxed_local(),
        ["Element", "container"] => |arguments, id| api::function_element_container(arguments, id).boxed_local(),
        ["Element", "stripe"] => |arguments, id| api::function_element_stripe(arguments, id).boxed_local(),
        ["Element", "button"] => |arguments, id| api::function_element_button(arguments, id).boxed_local(),
        ["Math", "sum"] => |arguments, id| api::function_math_sum(arguments, id).boxed_local(),
        ["Timer", "interval"] => |arguments, id| api::function_timer_interval(arguments, id).boxed_local(),
        _ => Err(ParseError::custom(span, format!("Unknown function '{}(..)'", path.join("/"))))?
    };
    Ok(definition)
}

fn pipe<'code>(from: Box<Spanned<Expression<'code>>>, mut to: Box<Spanned<Expression<'code>>>, run_duration: RunDuration) -> EvaluateResult<'code, Arc<ValueActor>> {
    match to.node {
        Expression::FunctionCall { ref path, ref mut arguments } => {
            let argument = Spanned {
                span: from.span,
                node: parser::Argument { name: "", value: Some(*from) }
            };
            // @TODO arguments: Vec -> arguments: VecDeque?
            arguments.insert(0, argument);
            spanned_expression_into_value_actor(*to, run_duration)
        }
        Expression::LinkSetter { alias } => {
            Err(ParseError::custom(to.span, "Piping into it is not supported yet, sorry"))?
        }
        Expression::Then { body } => {
            Ok(ThenCombinator::new_arc_value_actor(
                ConstructInfo::new(4, "THEN"),
                run_duration,
                spanned_expression_into_value_actor(*from, run_duration)?,
                {
                    let body_actor = spanned_expression_into_value_actor(*body, RunDuration::UntilFirstValue)?;
                    move || body_actor.subscribe()
                }),
            )
        }
        Expression::When { arms } => {
            Err(ParseError::custom(to.span, "Piping into it is not supported yet, sorry"))?
        }
        Expression::While { arms } => {
            Err(ParseError::custom(to.span, "Piping into it is not supported yet, sorry"))?
        }
        Expression::Pipe { from, to } => {
            Err(ParseError::custom(to.span, "Piping into it is not supported yet, sorry"))?
        }
        _ => {
            Err(ParseError::custom(to.span, "Piping into this target is not supported"))?
        }
    }
}


