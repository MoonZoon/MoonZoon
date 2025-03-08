use super::{Alias, Expression, ParseError, Span, Spanned, Token};
use std::collections::{BTreeMap, HashSet};

// @TODO Immutables or different tree traversal algorithm?
pub type ReachableReferenceables<'code> = BTreeMap<&'code str, Vec<Referenceable<'code>>>;

#[derive(Debug)]
pub struct Referenceables<'code> {
    pub referenced: Option<Referenceable<'code>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Referenceable<'code> {
    pub name: &'code str,
    pub span: Span,
    pub level: usize,
}

pub type ResolveError<'code> = ParseError<'code, Token<'code>>;

// @TODO How to handle loops?
pub fn resolve_references(
    mut expressions: Vec<Spanned<Expression>>,
) -> Result<Vec<Spanned<Expression>>, Vec<ResolveError>> {
    let mut reachable_referenceables = ReachableReferenceables::default();
    let level = 0;
    let parent_name = None::<&str>;
    for expressions in &expressions {
        let Spanned {
            span,
            node: expression,
            persistence: _,
        } = expressions;
        if let Expression::Variable(variable) = expression {
            let name = &variable.name;
            reachable_referenceables
                .entry(name)
                .or_default()
                .push(Referenceable {
                    name,
                    span: *span,
                    level,
                });
        }
    }
    let mut errors = Vec::new();
    let mut all_referenced = HashSet::new();
    for expression in &mut expressions {
        set_is_referenced_and_alias_referenceables(
            expression,
            reachable_referenceables.clone(),
            level,
            parent_name,
            &mut errors,
            &mut all_referenced,
        );
    }
    for expressions in &mut expressions {
        let Spanned {
            span,
            node: expression,
            persistence: _,
        } = expressions;
        if let Expression::Variable(variable) = expression {
            let name = &variable.name;
            if all_referenced.contains(&Referenceable {
                name,
                span: *span,
                level,
            }) {
                variable.is_referenced = true;
            }
        }
    }
    if errors.is_empty() {
        Ok(expressions)
    } else {
        Err(errors)
    }
}

fn set_is_referenced_and_alias_referenceables<'a, 'code>(
    mut expression: &'a mut Spanned<Expression<'code>>,
    mut reachable_referenceables: ReachableReferenceables<'code>,
    mut level: usize,
    parent_name: Option<&str>,
    errors: &mut Vec<ResolveError>,
    all_referenced: &mut HashSet<Referenceable<'code>>,
) {
    let Spanned {
        span,
        node: expression,
        persistence: _,
    } = &mut expression;
    match expression {
        Expression::Variable(variable) => {
            set_is_referenced_and_alias_referenceables(
                &mut variable.value,
                reachable_referenceables,
                level,
                Some(variable.name),
                errors,
                all_referenced,
            );
        }
        Expression::Object(object) => {
            level += 1;
            for variable in &object.variables {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                let name = &variable.name;
                reachable_referenceables
                    .entry(name)
                    .or_default()
                    .push(Referenceable {
                        name,
                        span: *span,
                        level,
                    });
            }
            for variable in &mut object.variables {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                set_is_referenced_and_alias_referenceables(
                    &mut variable.value,
                    reachable_referenceables.clone(),
                    level,
                    Some(variable.name),
                    errors,
                    all_referenced,
                );
            }
            for variable in &mut object.variables {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                let name = &variable.name;
                if all_referenced.contains(&Referenceable {
                    name,
                    span: *span,
                    level,
                }) {
                    variable.is_referenced = true;
                }
            }
        }
        Expression::TaggedObject { tag, object } => {
            level += 1;
            for variable in &object.variables {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                let name = &variable.name;
                reachable_referenceables
                    .entry(name)
                    .or_default()
                    .push(Referenceable {
                        name,
                        span: *span,
                        level,
                    });
            }
            for variable in &mut object.variables {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                set_is_referenced_and_alias_referenceables(
                    &mut variable.value,
                    reachable_referenceables.clone(),
                    level,
                    Some(variable.name),
                    errors,
                    all_referenced,
                );
            }
            for variable in &mut object.variables {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                let name = &variable.name;
                if all_referenced.contains(&Referenceable {
                    name,
                    span: *span,
                    level,
                }) {
                    variable.is_referenced = true;
                }
            }
        }
        Expression::FunctionCall { path, arguments } => {
            level += 1;
            for argument in arguments.iter() {
                let Spanned {
                    span,
                    node: argument,
                    persistence: _,
                } = argument;
                let name = &argument.name;
                reachable_referenceables
                    .entry(name)
                    .or_default()
                    .push(Referenceable {
                        name,
                        span: *span,
                        level,
                    });
            }
            for argument in arguments.iter_mut() {
                let Spanned {
                    span,
                    node: argument,
                    persistence: _,
                } = argument;
                if let Some(value) = argument.value.as_mut() {
                    set_is_referenced_and_alias_referenceables(
                        value,
                        reachable_referenceables.clone(),
                        level,
                        Some(argument.name),
                        errors,
                        all_referenced,
                    );
                }
            }
            for argument in arguments.iter_mut() {
                let Spanned {
                    span,
                    node: argument,
                    persistence: _,
                } = argument;
                let name = &argument.name;
                if all_referenced.contains(&Referenceable {
                    name,
                    span: *span,
                    level,
                }) {
                    argument.is_referenced = true;
                }
            }
        }
        Expression::Block { variables, output } => {
            level += 1;
            for variable in variables.iter() {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                let name = &variable.name;
                reachable_referenceables
                    .entry(name)
                    .or_default()
                    .push(Referenceable {
                        name,
                        span: *span,
                        level,
                    });
            }
            for variable in variables.iter_mut() {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                set_is_referenced_and_alias_referenceables(
                    &mut variable.value,
                    reachable_referenceables.clone(),
                    level,
                    Some(variable.name),
                    errors,
                    all_referenced,
                );
            }
            set_is_referenced_and_alias_referenceables(
                output,
                reachable_referenceables,
                level,
                parent_name,
                errors,
                all_referenced,
            );
            for variable in variables.iter_mut() {
                let Spanned {
                    span,
                    node: variable,
                    persistence: _,
                } = variable;
                let name = &variable.name;
                if all_referenced.contains(&Referenceable {
                    name,
                    span: *span,
                    level,
                }) {
                    variable.is_referenced = true;
                }
            }
        }
        Expression::List { items } => {
            for item in items {
                set_is_referenced_and_alias_referenceables(
                    item,
                    reachable_referenceables.clone(),
                    level,
                    parent_name,
                    errors,
                    all_referenced,
                );
            }
        }
        Expression::Map { entries } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Scope resolver cannot resolve references in Expression::Map yet, sorry".to_owned(),
            ))
        }
        Expression::Latest { inputs } => {
            for input in inputs {
                set_is_referenced_and_alias_referenceables(
                    input,
                    reachable_referenceables.clone(),
                    level,
                    parent_name,
                    errors,
                    all_referenced,
                );
            }
        }
        Expression::Then { body } => {
            set_is_referenced_and_alias_referenceables(
                body,
                reachable_referenceables,
                level,
                parent_name,
                errors,
                all_referenced,
            );
        }
        Expression::When { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Scope resolver cannot resolve references in Expression::When yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::While { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Scope resolver cannot resolve references in Expression::While yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Pipe { from, to } => {
            set_is_referenced_and_alias_referenceables(
                from,
                reachable_referenceables.clone(),
                level,
                parent_name,
                errors,
                all_referenced,
            );
            set_is_referenced_and_alias_referenceables(
                to,
                reachable_referenceables,
                level,
                parent_name,
                errors,
                all_referenced,
            );
        }
        Expression::ArithmeticOperator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::ArithmeticOperator yet, sorry".to_owned()))
        }
        Expression::Comparator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Scope resolver cannot resolve references in Expression::Comparator yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Function { .. } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(
                *span,
                "Scope resolver cannot resolve references in Expression::Function yet, sorry"
                    .to_owned(),
            ))
        }
        Expression::Alias(alias) => set_referenced_referenceable(
            alias,
            *span,
            reachable_referenceables,
            parent_name,
            errors,
            all_referenced,
        ),
        Expression::LinkSetter { alias } => {
            let Spanned {
                span,
                node: alias,
                persistence: _,
            } = alias;
            set_referenced_referenceable(
                alias,
                *span,
                reachable_referenceables,
                parent_name,
                errors,
                all_referenced,
            )
        }
        Expression::Literal(_) => (),
        Expression::Link => (),
        Expression::Skip => (),
    }
}

fn set_referenced_referenceable<'code>(
    alias: &mut Alias<'code>,
    span: Span,
    reachable_referenceables: ReachableReferenceables<'code>,
    parent_name: Option<&str>,
    errors: &mut Vec<ResolveError>,
    all_referenced: &mut HashSet<Referenceable<'code>>,
) {
    match alias {
        Alias::WithPassed { extra_parts } => (),
        Alias::WithoutPassed {
            parts,
            referenceables: unset_referenceables,
        } => {
            // @TODO make the first part a standalone property (name or PASSED)?
            let first_part = *parts.first().expect("Failed to get first alias part");
            // @TODO make Argument name optional to model the case when an argument is piped better?
            if first_part.is_empty() {
                return;
            }
            let reachable_referenceables: BTreeMap<&str, Referenceable> = reachable_referenceables
                .into_iter()
                .filter_map(|(name, referenceables)| {
                    referenceables.into_iter().rev().enumerate().find_map(
                        |(index, referenceable)| {
                            if index == 0 && Some(referenceable.name) == parent_name {
                                None
                            } else {
                                Some((referenceable.name, referenceable))
                            }
                        },
                    )
                })
                .collect();
            let referenced = reachable_referenceables.get(first_part).copied();
            if let Some(referenced) = referenced {
                all_referenced.insert(referenced);
            } else {
                let reachable_names = reachable_referenceables.keys();
                errors.push(ResolveError::custom(span, format!("Cannot find the variable or argument '{first_part}'. You can refer to: {reachable_names:?}")))
            }
            let referenceables = Referenceables { referenced };
            *unset_referenceables = Some(referenceables);
        }
    }
}
