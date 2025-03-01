use super::{Alias, Expression, ParseError, Span, Spanned, Token};
use std::collections::{BTreeMap, HashSet};
use ulid::Ulid;

pub type PersistenceId = Ulid;

#[derive(Debug, Clone, Copy)]
pub struct Persistence {
    pub id: PersistenceId,
    pub status: PersistenceStatus
}

#[derive(Debug, Clone, Copy)]
pub enum PersistenceStatus {
    NewOrChanged,
    Unchanged,
}

pub type ResolveError<'code> = ParseError<'code, Token<'code>>;

pub fn resolve_persistence<'new_code, 'old_code>(
    mut new_expressions: Vec<Spanned<Expression<'new_code>>>,
    old_expressions: Option<Vec<Spanned<Expression<'old_code>>>>,
) -> Result<(Vec<Spanned<Expression<'new_code>>>, HashSet<PersistenceId>), Vec<ResolveError<'new_code>>> {
    let old_expressions = old_expressions.unwrap_or_default();
    let mut all_persistence_ids = HashSet::new();
    let mut errors = Vec::new();
    for new_expression in &mut new_expressions {
        set_persistence(
            new_expression,
            &old_expressions.iter().collect::<Vec<_>>(),
            &mut all_persistence_ids,
            &mut errors,
        );
    }
    if errors.is_empty() {
        Ok((new_expressions, all_persistence_ids))
    } else {
        Err(errors)
    }
}

fn set_persistence<'a, 'code, 'old_code>(
    mut new_expression: &'a mut Spanned<Expression<'code>>,
    old_expressions: &'a [&Spanned<Expression<'old_code>>],
    all_persistence_ids: &mut HashSet<PersistenceId>,
    errors: &mut Vec<ResolveError>,
) {
    let Spanned {
        span,
        node: expression,
        persistence,
    } = &mut new_expression;

    if old_expressions.is_empty() {
        let id: Ulid = PersistenceId::new();
        all_persistence_ids.insert(id);
        *persistence = Some(Persistence {
            id,
            status: PersistenceStatus::NewOrChanged,
        });
    }

    match expression {
        Expression::Variable(variable) => {
            let old_variable_value_and_id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span: _, persistence: Some(Persistence { id, status: _ }), node: Expression::Variable(old_variable) } if variable.name == old_variable.name => {
                        Some((&old_variable.value, *id))
                    }
                    _ => None
                }
            });
            if let Some((old_variable_value, id)) = old_variable_value_and_id {
                all_persistence_ids.insert(id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
                set_persistence(
                    &mut variable.value,
                    &[old_variable_value],
                    all_persistence_ids,
                    errors,
                );
            } else {
                let id: Ulid = PersistenceId::new();
                all_persistence_ids.insert(id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
                set_persistence(
                    &mut variable.value,
                    &[],
                    all_persistence_ids,
                    errors,
                )
            }
        }
        Expression::Object(object) => {
            let old_object_variables_and_id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span: _, persistence: Some(Persistence { id, status: _ }), node: Expression::Object(old_object) } => {
                        Some((&old_object.variables, *id))
                    }
                    _ => None
                }
            });
            if let Some((old_object_variables, id)) = old_object_variables_and_id {
                all_persistence_ids.insert(id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
                for variable in &mut object.variables {
                    let Spanned {
                        span: _,
                        node: variable,
                        persistence,
                    } = variable;
                    let old_variable_value_and_id = old_object_variables
                        .iter()
                        .find_map(|old_variable| {
                            match old_variable {
                                Spanned { span: _, persistence: Some(Persistence { id, status: _ }), node: old_variable } if variable.name == old_variable.name => {
                                    Some((&old_variable.value, *id))
                                },
                                _ => None
                            }
                        });
                    if let Some((old_variable_value, id)) = old_variable_value_and_id {
                        all_persistence_ids.insert(id);
                        *persistence = Some(Persistence {
                            id,
                            status: PersistenceStatus::Unchanged,
                        });
                        set_persistence(
                            &mut variable.value,
                            &[old_variable_value],
                            all_persistence_ids,
                            errors,
                        );
                    } else {
                        let id: Ulid = PersistenceId::new();
                        all_persistence_ids.insert(id);
                        *persistence = Some(Persistence {
                            id,
                            status: PersistenceStatus::NewOrChanged,
                        });
                        set_persistence(
                            &mut variable.value,
                            &[],
                            all_persistence_ids,
                            errors,
                        )
                    }
                }
            } else {
                let id: Ulid = PersistenceId::new();
                all_persistence_ids.insert(id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
                for variable in &mut object.variables {
                    let Spanned {
                        span: _,
                        node: variable,
                        persistence,
                    } = variable;
                    let id: Ulid = PersistenceId::new();
                    all_persistence_ids.insert(id);
                    *persistence = Some(Persistence {
                        id,
                        status: PersistenceStatus::NewOrChanged,
                    });
                    set_persistence(
                        &mut variable.value,
                        &[],
                        all_persistence_ids,
                        errors,
                    );
                }
            }
        }
        Expression::TaggedObject { tag, object } => {
            for variable in &mut object.variables {
                let Spanned {
                    span: _,
                    node: variable,
                    persistence,
                } = variable;
                set_persistence(
                    &mut variable.value,
                    &[],
                    all_persistence_ids,
                    errors,
                );
            }
        }
        Expression::FunctionCall { path, arguments } => {
            for argument in arguments.iter_mut() {
                let Spanned {
                    span: _,
                    node: argument,
                    persistence,
                } = argument;
                if let Some(value) = argument.value.as_mut() {
                    set_persistence(
                        value,
                        &[],
                        all_persistence_ids,
                        errors,
                    );
                }
            }
        }
        Expression::Block { variables, output } => {
            for variable in variables.iter_mut() {
                let Spanned {
                    span: _,
                    node: variable,
                    persistence,
                } = variable;
                set_persistence(
                    &mut variable.value,
                    &[],
                    all_persistence_ids,
                    errors,
                );
            }
            set_persistence(
                output,
                &[],
                all_persistence_ids,
                errors,
            );
        }
        Expression::List { items } => {
            for item in items {
                set_persistence(
                    item,
                    &[],
                    all_persistence_ids,
                    errors,
                );
            }
        }
        Expression::Map { entries } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve references in Expression::Map yet, sorry".to_owned(),
            ))
        }
        Expression::Latest { inputs } => {
            for input in inputs {
                set_persistence(
                    input,
                    &[],
                    all_persistence_ids,
                    errors,
                );
            }
        }
        Expression::Then { body } => {
            set_persistence(
                body,
                &[],
                all_persistence_ids,
                errors,
            );
        }
        Expression::When { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve references in Expression::When yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::While { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve references in Expression::While yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Pipe { from, to } => {
            set_persistence(
                from,
                &[],
                all_persistence_ids,
                errors,
            );
            set_persistence(
                to,
                &[],
                all_persistence_ids,
                errors,
            );
        }
        Expression::ArithmeticOperator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Persistence resolver cannot resolve references in Expression::ArithmeticOperator yet, sorry".to_owned()))
        }
        Expression::Comparator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve references in Expression::Comparator yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Function { .. } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve references in Expression::Function yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::LinkSetter { alias } => (),
        Expression::Alias(alias) => (),
        Expression::Literal(_) => (),
        Expression::Link => (),
        Expression::Skip => (),
    }
}
