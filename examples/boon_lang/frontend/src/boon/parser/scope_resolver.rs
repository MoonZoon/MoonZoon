use super::{Expression, ParseError, Span, Spanned, Token, Alias};
use std::collections::BTreeMap;

// @TODO immutable or different tree traversal algorithm?
pub type ReachableReferenceables<'code> = BTreeMap<&'code str, Referenceable<'code>>;

#[derive(Debug)]
pub struct Referenceables<'code> {
    referenced: Option<Referenceable<'code>>,
    // @TODO remove?
    reachable: ReachableReferenceables<'code>,
}

#[derive(Debug, Clone, Copy)]
pub struct Referenceable<'code> {
    name: &'code str,
    span: Span,
    level: usize,
}

pub type ResolveError<'code> = ParseError<'code, Token<'code>>;

// @TODO How to handle loops?
pub fn resolve_references(
    mut expressions: Vec<Spanned<Expression>>,
) -> Result<Vec<Spanned<Expression>>, Vec<ResolveError>> {
    let mut reachable_referenceables = ReachableReferenceables::default();
    let level = 0;
    for mut expressions in &expressions {
        let Spanned { span, node: expression } = expressions;
        if let Expression::Variable(variable) = expression {
            let name = &variable.name;
            reachable_referenceables.insert(name, Referenceable { name, span: *span, level });
        }
    }
    let mut errors = Vec::new();
    for expression in &mut expressions {
        set_reference_count_and_alias_referenceables(expression, reachable_referenceables.clone(), level, &mut errors);
    }
    if errors.is_empty() {
        Ok(expressions)
    } else {
        Err(errors)
    }
}

fn set_reference_count_and_alias_referenceables<'a, 'code>(
    mut expression: &'a mut Spanned<Expression<'code>>,
    mut reachable_referenceables: ReachableReferenceables<'code>,
    mut level: usize,
    errors: &mut Vec<ResolveError>,
) {
    let Spanned { span, node: expression } = &mut expression;
    match expression {
        Expression::Variable(variable) => {
            set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables, level, errors);
        }
        Expression::Object(object) => {
            level += 1;
            for mut variable in &object.variables {
                let Spanned { span, node: variable } = variable;
                let name = &variable.name;
                reachable_referenceables.insert(name, Referenceable { name, span: *span, level });
            }
            for mut variable in &mut object.variables {
                let Spanned { span, node: variable } = variable;
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level, errors);
            }
        }
        Expression::TaggedObject { tag, object } => {
            level += 1;
            for mut variable in &object.variables {
                let Spanned { span, node: variable } = variable;
                let name = &variable.name;
                reachable_referenceables.insert(name, Referenceable { name, span: *span, level });
            }
            for mut variable in &mut object.variables {
                let Spanned { span, node: variable } = variable;
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level, errors);
            }
        }
        Expression::FunctionCall { path, arguments } => {
            level += 1;
            for argument in arguments.iter() {
                let Spanned { span, node: argument } = argument;
                let name = &argument.name;
                reachable_referenceables.insert(name, Referenceable { name, span: *span, level });
            }
            for mut argument in arguments {
                let Spanned { span, node: argument } = argument;
                if let Some(value) = argument.value.as_mut() {
                    set_reference_count_and_alias_referenceables(value, reachable_referenceables.clone(), level, errors);
                }
            }
        }
        Expression::Block { variables, output } => {
            level += 1;
            for variable in variables.iter() {
                let Spanned { span, node: variable } = variable;
                let name = &variable.name;
                reachable_referenceables.insert(name, Referenceable { name, span: *span, level });
            }
            for mut variable in variables {
                let Spanned { span, node: variable } = variable;
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level, errors);
            }
            set_reference_count_and_alias_referenceables(output, reachable_referenceables, level, errors);
        }
        Expression::List { items } => {
            for mut item in items {
                set_reference_count_and_alias_referenceables(item, reachable_referenceables.clone(), level, errors);
            }
        }
        Expression::Map { entries } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::Map yet, sorry".to_owned()))
        }
        Expression::Latest { inputs } => {
            for mut input in inputs {
                set_reference_count_and_alias_referenceables(input, reachable_referenceables.clone(), level, errors);
            }
        }
        Expression::Then { body } => {
            set_reference_count_and_alias_referenceables(body, reachable_referenceables, level, errors);
        }
        Expression::When { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::When yet, sorry".to_owned()))
        }
        Expression::While { arms } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::While yet, sorry".to_owned()))
        }
        Expression::Pipe { from, to } => {
            set_reference_count_and_alias_referenceables(from, reachable_referenceables.clone(), level, errors);
            set_reference_count_and_alias_referenceables(to, reachable_referenceables, level, errors);
        }
        Expression::ArithmeticOperator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::ArithmeticOperator yet, sorry".to_owned()))
        },
        Expression::Comparator(_) => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::Comparator yet, sorry".to_owned()))
        },
        Expression::Function { .. } => {
            // @TODO implement, see the error message below
            errors.push(ResolveError::custom(*span, "Scope resolver cannot resolve references in Expression::Function yet, sorry".to_owned()))
        },
        Expression::Alias(alias) => {
            set_referenced_referenceable(alias, *span, reachable_referenceables, errors)
        }
        Expression::LinkSetter { alias } =>  {
            let Spanned { span, node: alias } = alias;
            set_referenced_referenceable(alias, *span, reachable_referenceables, errors)
        },
        Expression::Literal(_) => (),
        Expression::Link => (),
        Expression::Skip => (),
    }
}

fn set_referenced_referenceable<'code>(
    alias: &mut Alias<'code>, 
    span: Span, 
    reachable_referenceables: ReachableReferenceables<'code>,
    errors: &mut Vec<ResolveError>,
) {
    match alias {
        Alias::WithPassed { extra_parts } => (),
        Alias::WithoutPassed { parts, referenceables: unset_referenceables } => {
            // @TODO make the first part a standalone property (name or PASSED)?
            let first_part = parts.first().expect("Failed to get first alias part");
            // @TODO make Argument name optional to model the case when an argument is piped better?
            if first_part.is_empty() {
                return;
            }
            let referenced = reachable_referenceables.get(first_part).copied();
            if referenced.is_none() {
                let reachable_names = reachable_referenceables.keys();
                errors.push(ResolveError::custom(span, format!("Cannot find the variable or argument '{first_part}'. You can refer to: {reachable_names:?}")))
            }
            let referenceables = Referenceables {
                referenced,
                reachable: reachable_referenceables,
            };
            *unset_referenceables = Some(referenceables);
        }
    }
}
