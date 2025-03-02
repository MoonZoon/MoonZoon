use super::{Alias, Expression, ParseError, Span, Spanned, Token};

use std::collections::{HashMap, HashSet};

use zoon::{local_storage, WebStorage, eprintln};
use ulid::Ulid;
use serde_json_any_key::*;

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
    old_span_id_pairs_local_storage_key: &str,
) -> Result<(Vec<Spanned<Expression<'new_code>>>, HashMap<Span, PersistenceId>), Vec<ResolveError<'new_code>>> {
    // @TODO add a new Zoon WebStorage method like `get_raw_string`? 
    let old_span_id_pairs = if let Some(Ok(old_span_id_pairs)) = local_storage().get::<String>(&old_span_id_pairs_local_storage_key) {
        match json_to_map::<Span, PersistenceId>(&old_span_id_pairs) {
            Ok(old_span_id_pairs) => { 
                println!("old_span_id_pairs LOADED!");
                Some(old_span_id_pairs)
            }
            Err(error) => { 
                eprintln!("Failed to deserialize Span-PersistenceId pairs: {error:#}");
                None
            }
        }
    } else {
        None
    };

    let old_expressions = old_expressions.unwrap_or_default();
    let old_span_id_pairs = old_span_id_pairs.unwrap_or_default();

    let mut new_span_id_pairs = HashMap::new();
    let mut errors = Vec::new();
    for new_expression in &mut new_expressions {
        set_persistence(
            new_expression,
            &old_expressions.iter().collect::<Vec<_>>(),
            &old_span_id_pairs,
            &mut new_span_id_pairs,
            &mut errors,
        );
    }
    if errors.is_empty() {
        Ok((new_expressions, new_span_id_pairs))
    } else {
        Err(errors)
    }
}

fn set_persistence<'a, 'code, 'old_code>(
    mut new_expression: &'a mut Spanned<Expression<'code>>,
    // @TODO rewrite to use root Object and only one `old_expression` here?
    old_expressions: &'a [&Spanned<Expression<'old_code>>],
    old_span_id_pairs: &HashMap<Span, PersistenceId>,
    new_span_id_pairs: &mut HashMap<Span, PersistenceId>,
    errors: &mut Vec<ResolveError>,
) {
    let Spanned {
        span,
        node: expression,
        persistence,
    } = &mut new_expression;

    if old_expressions.is_empty() {
        let id: Ulid = PersistenceId::new();
        new_span_id_pairs.insert(*span, id);
        *persistence = Some(Persistence {
            id,
            status: PersistenceStatus::NewOrChanged,
        });
    }

    match expression {
        Expression::Variable(variable) => {
            let old_variable_value_and_id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span, persistence: _, node: Expression::Variable(old_variable) } if variable.name == old_variable.name => {
                        Some((&old_variable.value, old_span_id_pairs[span]))
                    }
                    _ => None
                }
            });
            if let Some((old_variable_value, id)) = old_variable_value_and_id {
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
                set_persistence(
                    &mut variable.value,
                    &[old_variable_value],
                    &old_span_id_pairs,
                    new_span_id_pairs,
                    errors,
                );
            } else {
                let id: Ulid = PersistenceId::new();
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
                set_persistence(
                    &mut variable.value,
                    &[],
                    &old_span_id_pairs,
                    new_span_id_pairs,
                    errors,
                )
            }
        }
        Expression::Object(object) => {
            let old_object_variables_and_id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span, persistence: _, node: Expression::Object(old_object) } => {
                        Some((&old_object.variables, old_span_id_pairs[span]))
                    }
                    _ => None
                }
            });
            if let Some((old_object_variables, id)) = old_object_variables_and_id {
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
                for variable in &mut object.variables {
                    let Spanned {
                        span,
                        node: variable,
                        persistence,
                    } = variable;
                    let old_variable_value_and_id = old_object_variables
                        .iter()
                        .find_map(|old_variable| {
                            match old_variable {
                                Spanned { span, persistence: _, node: old_variable } if variable.name == old_variable.name => {
                                    Some((&old_variable.value, old_span_id_pairs[span]))
                                },
                                _ => None
                            }
                        });
                    if let Some((old_variable_value, id)) = old_variable_value_and_id {
                        new_span_id_pairs.insert(*span, id);
                        *persistence = Some(Persistence {
                            id,
                            status: PersistenceStatus::Unchanged,
                        });
                        set_persistence(
                            &mut variable.value,
                            &[old_variable_value],
                            &old_span_id_pairs,
                            new_span_id_pairs,
                            errors,
                        );
                    } else {
                        let id: Ulid = PersistenceId::new();
                        new_span_id_pairs.insert(*span, id);
                        *persistence = Some(Persistence {
                            id,
                            status: PersistenceStatus::NewOrChanged,
                        });
                        set_persistence(
                            &mut variable.value,
                            &[],
                            &old_span_id_pairs,
                            new_span_id_pairs,
                            errors,
                        )
                    }
                }
            } else {
                let id: Ulid = PersistenceId::new();
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
                for variable in &mut object.variables {
                    let Spanned {
                        span,
                        node: variable,
                        persistence,
                    } = variable;
                    let id: Ulid = PersistenceId::new();
                    new_span_id_pairs.insert(*span, id);
                    *persistence = Some(Persistence {
                        id,
                        status: PersistenceStatus::NewOrChanged,
                    });
                    set_persistence(
                        &mut variable.value,
                        &[],
                        &old_span_id_pairs,
                        new_span_id_pairs,
                        errors,
                    );
                }
            }
        }
        Expression::TaggedObject { tag, object } => {
            let old_object_variables_and_id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span, persistence: _, node: Expression::TaggedObject { tag: old_tag, object: old_object} } if tag == old_tag => {
                        Some((&old_object.variables, old_span_id_pairs[span]))
                    }
                    _ => None
                }
            });
            if let Some((old_object_variables, id)) = old_object_variables_and_id {
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
                for variable in &mut object.variables {
                    let Spanned {
                        span,
                        node: variable,
                        persistence,
                    } = variable;
                    let old_variable_value_and_id = old_object_variables
                        .iter()
                        .find_map(|old_variable| {
                            match old_variable {
                                Spanned { span, persistence: _, node: old_variable } if variable.name == old_variable.name => {
                                    Some((&old_variable.value, old_span_id_pairs[span]))
                                },
                                _ => None
                            }
                        });
                    if let Some((old_variable_value, id)) = old_variable_value_and_id {
                        new_span_id_pairs.insert(*span, id);
                        *persistence = Some(Persistence {
                            id,
                            status: PersistenceStatus::Unchanged,
                        });
                        set_persistence(
                            &mut variable.value,
                            &[old_variable_value],
                            &old_span_id_pairs,
                            new_span_id_pairs,
                            errors,
                        );
                    } else {
                        let id: Ulid = PersistenceId::new();
                        new_span_id_pairs.insert(*span, id);
                        *persistence = Some(Persistence {
                            id,
                            status: PersistenceStatus::NewOrChanged,
                        });
                        set_persistence(
                            &mut variable.value,
                            &[],
                            &old_span_id_pairs,
                            new_span_id_pairs,
                            errors,
                        )
                    }
                }
            } else {
                let id: Ulid = PersistenceId::new();
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
                for variable in &mut object.variables {
                    let Spanned {
                        span,
                        node: variable,
                        persistence,
                    } = variable;
                    let id: Ulid = PersistenceId::new();
                    new_span_id_pairs.insert(*span, id);
                    *persistence = Some(Persistence {
                        id,
                        status: PersistenceStatus::NewOrChanged,
                    });
                    set_persistence(
                        &mut variable.value,
                        &[],
                        &old_span_id_pairs,
                        new_span_id_pairs,
                        errors,
                    );
                }
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
                        &old_span_id_pairs,
                        new_span_id_pairs,
                        errors,
                    );
                }
            }
        }
        Expression::Block { variables, output } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::Block yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::List { items } => {
            for item in items {
                set_persistence(
                    item,
                    &[],
                    &old_span_id_pairs,
                    new_span_id_pairs,
                    errors,
                );
            }
        }
        Expression::Map { entries } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::Map yet, sorry".to_owned(),
            ))
        }
        Expression::Latest { inputs } => {
            for input in inputs {
                set_persistence(
                    input,
                    &[],
                    &old_span_id_pairs,
                    new_span_id_pairs,
                    errors,
                );
            }
        }
        Expression::Then { body } => {
            set_persistence(
                body,
                &[],
                &old_span_id_pairs,
                new_span_id_pairs,
                errors,
            );
        }
        Expression::When { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::When yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::While { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::While yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Pipe { from, to } => {
            set_persistence(
                from,
                &[],
                &old_span_id_pairs,
                new_span_id_pairs,
                errors,
            );
            set_persistence(
                to,
                &[],
                &old_span_id_pairs,
                new_span_id_pairs,
                errors,
            );
        }
        Expression::ArithmeticOperator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Persistence resolver cannot resolve Persistence in Expression::ArithmeticOperator yet, sorry".to_owned()))
        }
        Expression::Comparator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::Comparator yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Function { .. } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::Function yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::LinkSetter { alias } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Persistence resolver cannot resolve Persistence in Expression::Skip yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Alias(alias) => (),
        Expression::Literal(_) => (),
        Expression::Link => {
            let id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span, persistence: _, node: Expression::Link } => {
                        Some(old_span_id_pairs[span])
                    }
                    _ => None
                }
            });
            if let Some(id) = id {
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
            } else {
                let id: Ulid = PersistenceId::new();
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
            }
        }
        Expression::Skip => {
            let id = old_expressions.iter().find_map(|old_expression| {
                match old_expression {
                    Spanned { span, persistence: _, node: Expression::Skip } => {
                        Some(old_span_id_pairs[span])
                    }
                    _ => None
                }
            });
            if let Some(id) = id {
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::Unchanged,
                });
            } else {
                let id: Ulid = PersistenceId::new();
                new_span_id_pairs.insert(*span, id);
                *persistence = Some(Persistence {
                    id,
                    status: PersistenceStatus::NewOrChanged,
                });
            }
        }
    }
}
