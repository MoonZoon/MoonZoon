use std::borrow::Cow;
use std::pin::Pin;
use std::sync::Arc;

use zoon::mpsc;
use zoon::{Stream, StreamExt};

use super::super::super::parser::{
    self, Expression, ParseError, PersistenceId, Span, Spanned, Token,
};
use super::api;
use super::engine::*;

type EvaluateResult<'code, T> = Result<T, ParseError<'code, Token<'code>>>;

pub fn evaluate(
    expressions: Vec<Spanned<Expression>>,
    states_local_storage_key: impl Into<Cow<'static, str>>,
) -> EvaluateResult<(Arc<Object>, ConstructContext)> {
    let construct_context = ConstructContext {
        construct_storage: Arc::new(ConstructStorage::new(states_local_storage_key)),
    };
    let actor_context = ActorContext::default();
    let reference_connector = Arc::new(ReferenceConnector::new());
    let root_object = Object::new_arc(
        ConstructInfo::new("root", None, "root"),
        construct_context.clone(),
        expressions
            .into_iter()
            .map(
                |Spanned {
                     span,
                     node: expression,
                     persistence,
                 }| {
                    match expression {
                        Expression::Variable(variable) => spanned_variable_into_variable(
                            Spanned {
                                span,
                                node: *variable,
                                persistence,
                            },
                            construct_context.clone(),
                            actor_context.clone(),
                            reference_connector.clone(),
                        ),
                        Expression::Function {
                            name,
                            parameters,
                            body,
                        } => {
                            // @TODO implement
                            todo!("Function definitions are not supported yet, sorry")
                        }
                        _ => Err(ParseError::custom(
                            span,
                            "Only variables or functions expected",
                        )),
                    }
                },
            )
            .collect::<Result<Vec<_>, _>>()?,
    );
    Ok((root_object, construct_context))
}

// @TODO Is the rule "LINK has to be the only variable value" necessary? Validate it by the parser?
fn spanned_variable_into_variable(
    variable: Spanned<parser::Variable>,
    construct_context: ConstructContext,
    actor_context: ActorContext,
    reference_connector: Arc<ReferenceConnector>,
) -> EvaluateResult<Arc<Variable>> {
    let Spanned {
        span,
        node: variable,
        persistence,
    } = variable;
    let parser::Variable {
        name,
        value,
        is_referenced,
    } = variable;

    let persistence_id = persistence.expect("Failed to get Persistence").id;
    let name: String = name.to_owned();

    let construct_info = ConstructInfo::new(
        format!("PersistenceId: {persistence_id}"),
        persistence,
        format!("{span}; {name}"),
    );
    let variable = if matches!(
        &value,
        Spanned {
            span: _,
            node: Expression::Link,
            persistence: _,
        }
    ) {
        Variable::new_link_arc(construct_info, construct_context, name, actor_context)
    } else {
        Variable::new_arc(
            construct_info,
            construct_context.clone(),
            name,
            spanned_expression_into_value_actor(
                value,
                construct_context,
                actor_context,
                reference_connector.clone(),
            )?,
        )
    };
    if is_referenced {
        reference_connector.register_referenceable(span, variable.value_actor());
    }
    Ok(variable)
}

// @TODO resolve ids
fn spanned_expression_into_value_actor(
    expression: Spanned<Expression>,
    construct_context: ConstructContext,
    actor_context: ActorContext,
    reference_connector: Arc<ReferenceConnector>,
) -> EvaluateResult<Arc<ValueActor>> {
    let Spanned {
        span,
        node: expression,
        persistence,
    } = expression;

    let persistence_id = persistence.expect("Failed to get Persistence").id;
    let idempotency_key = persistence_id;

    let actor = match expression {
        Expression::Variable(variable) => Err(ParseError::custom(
            span,
            "Failed to evalute the variable in this context.",
        ))?,
        Expression::Literal(literal) => match literal {
            parser::Literal::Number(number) => Number::new_arc_value_actor(
                ConstructInfo::new(
                    format!("PersistenceId: {persistence_id}"),
                    persistence,
                    format!("{span}; Number {number}"),
                ),
                construct_context,
                idempotency_key,
                actor_context,
                number,
            ),
            parser::Literal::Text(text) => {
                let text = text.to_owned();
                Text::new_arc_value_actor(
                    ConstructInfo::new(
                        format!("PersistenceId: {persistence_id}"),
                        persistence,
                        format!("{span}; Text {text}"),
                    ),
                    construct_context,
                    idempotency_key,
                    actor_context,
                    text,
                )
            }
            parser::Literal::Tag(tag) => {
                let tag = tag.to_owned();
                Tag::new_arc_value_actor(
                    ConstructInfo::new(
                        format!("PersistenceId: {persistence_id}"),
                        persistence,
                        format!("{span}; Tag {tag}"),
                    ),
                    construct_context,
                    idempotency_key,
                    actor_context,
                    tag,
                )
            }
        },
        Expression::List { items } => List::new_arc_value_actor(
            ConstructInfo::new(
                format!("PersistenceId: {persistence_id}"),
                persistence,
                format!("{span}; LIST {{..}}"),
            ),
            construct_context.clone(),
            idempotency_key,
            actor_context.clone(),
            items
                .into_iter()
                .map(|item| {
                    spanned_expression_into_value_actor(
                        item,
                        construct_context.clone(),
                        actor_context.clone(),
                        reference_connector.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expression::Object(object) => Object::new_arc_value_actor(
            ConstructInfo::new(
                format!("PersistenceId: {persistence_id}"),
                persistence,
                format!("{span}; [..]"),
            ),
            construct_context.clone(),
            idempotency_key,
            actor_context.clone(),
            object
                .variables
                .into_iter()
                .map(|variable| {
                    spanned_variable_into_variable(
                        variable,
                        construct_context.clone(),
                        actor_context.clone(),
                        reference_connector.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expression::TaggedObject { tag, object } => TaggedObject::new_arc_value_actor(
            ConstructInfo::new(
                format!("PersistenceId: {persistence_id}"),
                persistence,
                format!("{span}; {tag}[..]"),
            ),
            construct_context.clone(),
            idempotency_key,
            actor_context.clone(),
            tag.to_owned(),
            object
                .variables
                .into_iter()
                .map(|variable| {
                    spanned_variable_into_variable(
                        variable,
                        construct_context.clone(),
                        actor_context.clone(),
                        reference_connector.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expression::Map { entries } => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::Map]",
        ))?,
        Expression::Function {
            name,
            parameters,
            body,
        } => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::Function]",
        ))?,
        Expression::FunctionCall { path, arguments } => {
            // @TODO better argument error handling
            FunctionCall::new_arc_value_actor(
                ConstructInfo::new(
                    format!("PersistenceId: {persistence_id}"),
                    persistence,
                    format!("{span}; {}(..)", path.join("/")),
                ),
                construct_context.clone(),
                actor_context.clone(),
                function_call_path_to_definition(&path, span)?,
                arguments
                    .into_iter()
                    .map(
                        |Spanned {
                             span,
                             node: argument,
                             persistence: _,
                         }| {
                            let parser::Argument {
                                name,
                                value,
                                is_referenced,
                            } = argument;
                            let Some(value) = value else {
                                // @TODO support out arguments
                                Err(ParseError::custom(
                                    span,
                                    "Out arguments not supported yet, sorry",
                                ))?
                            };
                            let actor = spanned_expression_into_value_actor(
                                value,
                                construct_context.clone(),
                                actor_context.clone(),
                                reference_connector.clone(),
                            );
                            if is_referenced {
                                if let Ok(actor) = &actor {
                                    reference_connector.register_referenceable(span, actor.clone());
                                }
                            }
                            actor
                        },
                    )
                    .collect::<Result<Vec<_>, _>>()?,
            )
        }
        Expression::Alias(alias) => {
            let root_value_actor = match &alias {
                parser::Alias::WithPassed { extra_parts } => Err(ParseError::custom(
                    span,
                    "Aliases with PASSED not supported yet, sorry",
                ))?,
                parser::Alias::WithoutPassed {
                    parts,
                    referenceables,
                } => {
                    let referenced = referenceables
                        .as_ref()
                        .expect("Failed to get alias referenceables in evaluator")
                        .referenced;
                    if let Some(referenced) = referenced {
                        reference_connector.referenceable(referenced.span)
                    } else {
                        Err(ParseError::custom(
                            span,
                            "Failed to get aliased variable or argument",
                        ))?
                    }
                }
            };
            VariableOrArgumentReference::new_arc_value_actor(
                ConstructInfo::new(
                    format!("PersistenceId: {persistence_id}"),
                    persistence,
                    format!("{span}; {alias} (alias)"),
                ),
                construct_context,
                actor_context,
                alias,
                root_value_actor,
            )
        }
        Expression::LinkSetter { alias } => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::LinkSetter]",
        ))?,
        Expression::Link => Err(ParseError::custom(
            span,
            "LINK has to be the only variable value - e.g. `press: LINK`",
        ))?,
        Expression::Latest { inputs } => LatestCombinator::new_arc_value_actor(
            ConstructInfo::new(
                format!("PersistenceId: {persistence_id}"),
                persistence,
                format!("{span}; LATEST {{..}}"),
            ),
            construct_context.clone(),
            actor_context.clone(),
            inputs
                .into_iter()
                .map(|input| {
                    spanned_expression_into_value_actor(
                        input,
                        construct_context.clone(),
                        actor_context.clone(),
                        reference_connector.clone(),
                    )
                })
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Expression::Then { body } => Err(ParseError::custom(
            span,
            "You have to pipe things into THEN - e.g. `..press |> THEN { .. }`",
        ))?,
        Expression::When { arms } => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::When]",
        ))?,
        Expression::While { arms } => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::While]",
        ))?,
        Expression::Pipe { from, to } => pipe(
            from,
            to,
            construct_context,
            actor_context,
            reference_connector,
        )?,
        Expression::Skip => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::Skip]",
        ))?,
        Expression::Block { variables, output } => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::Block]",
        ))?,
        Expression::Comparator(comparator) => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::Comparator]",
        ))?,
        Expression::ArithmeticOperator(arithmetic_operator) => Err(ParseError::custom(
            span,
            "Not supported yet, sorry [Expression::ArithmeticOperator]",
        ))?,
    };
    Ok(actor)
}

fn function_call_path_to_definition<'code>(
    path: &[&'code str],
    span: Span,
) -> EvaluateResult<
    'code,
    impl Fn(
        Arc<Vec<Arc<ValueActor>>>,
        ConstructId,
        PersistenceId,
        ConstructContext,
        ActorContext,
    ) -> Pin<Box<dyn Stream<Item = Value>>>
    + use<>,
> {
    let definition = match path {
        ["Document", "new"] => |arguments, id, persistence_id, construct_context, actor_context| {
            api::function_document_new(
                arguments,
                id,
                persistence_id,
                construct_context,
                actor_context,
            )
            .boxed_local()
        },
        ["Element", "container"] => {
            |arguments, id, persistence_id, construct_context, actor_context| {
                api::function_element_container(
                    arguments,
                    id,
                    persistence_id,
                    construct_context,
                    actor_context,
                )
                .boxed_local()
            }
        }
        ["Element", "stripe"] => {
            |arguments, id, persistence_id, construct_context, actor_context| {
                api::function_element_stripe(
                    arguments,
                    id,
                    persistence_id,
                    construct_context,
                    actor_context,
                )
                .boxed_local()
            }
        }
        ["Element", "button"] => {
            |arguments, id, persistence_id, construct_context, actor_context| {
                api::function_element_button(
                    arguments,
                    id,
                    persistence_id,
                    construct_context,
                    actor_context,
                )
                .boxed_local()
            }
        }
        ["Math", "sum"] => |arguments, id, persistence_id, construct_context, actor_context| {
            api::function_math_sum(
                arguments,
                id,
                persistence_id,
                construct_context,
                actor_context,
            )
            .boxed_local()
        },
        ["Timer", "interval"] => {
            |arguments, id, persistence_id, construct_context, actor_context| {
                api::function_timer_interval(
                    arguments,
                    id,
                    persistence_id,
                    construct_context,
                    actor_context,
                )
                .boxed_local()
            }
        }
        _ => Err(ParseError::custom(
            span,
            format!("Unknown function '{}(..)'", path.join("/")),
        ))?,
    };
    Ok(definition)
}

fn pipe<'code>(
    from: Box<Spanned<Expression<'code>>>,
    mut to: Box<Spanned<Expression<'code>>>,
    construct_context: ConstructContext,
    actor_context: ActorContext,
    reference_connector: Arc<ReferenceConnector>,
) -> EvaluateResult<'code, Arc<ValueActor>> {
    // @TODO destructure `to`?
    let to_persistence_id = to.persistence.expect("Failed to get persistence").id;
    match to.node {
        Expression::FunctionCall {
            ref path,
            ref mut arguments,
        } => {
            let argument = Spanned {
                span: from.span,
                persistence: from.persistence,
                node: parser::Argument {
                    name: "",
                    value: Some(*from),
                    is_referenced: false,
                },
            };
            // @TODO arguments: Vec -> arguments: VecDeque?
            arguments.insert(0, argument);
            spanned_expression_into_value_actor(
                *to,
                construct_context,
                actor_context,
                reference_connector,
            )
        }
        Expression::LinkSetter { alias } => Err(ParseError::custom(
            to.span,
            "Piping into it is not supported yet, sorry [Expression::LinkSetter]",
        ))?,
        Expression::Then { body } => {
            let (impulse_sender, impulse_receiver) = mpsc::unbounded();
            let mut body_actor_context = actor_context.clone();
            body_actor_context.output_valve_signal =
                Some(Arc::new(ActorOutputValveSignal::new(impulse_receiver)));

            Ok(ThenCombinator::new_arc_value_actor(
                ConstructInfo::new(
                    format!("Persistence: {to_persistence_id}"),
                    to.persistence,
                    format!("{to_persistence_id}; THEN"),
                ),
                construct_context.clone(),
                actor_context.clone(),
                spanned_expression_into_value_actor(
                    *from,
                    construct_context.clone(),
                    actor_context.clone(),
                    reference_connector.clone(),
                )?,
                impulse_sender,
                spanned_expression_into_value_actor(
                    *body,
                    construct_context,
                    body_actor_context,
                    reference_connector,
                )?,
            ))
        }
        Expression::When { arms } => Err(ParseError::custom(
            to.span,
            "Piping into it is not supported yet, sorry [Expression::When]",
        ))?,
        Expression::While { arms } => Err(ParseError::custom(
            to.span,
            "Piping into it is not supported yet, sorry [Expression::While]",
        ))?,
        Expression::Pipe { from, to } => Err(ParseError::custom(
            to.span,
            "Piping into it is not supported yet, sorry [Expression::Pipe]",
        ))?,
        _ => Err(ParseError::custom(
            to.span,
            "Piping into this target is not supported",
        ))?,
    }
}
