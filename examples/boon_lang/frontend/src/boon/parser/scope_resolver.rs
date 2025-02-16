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

pub type ResolveResult<'code, T> = Result<T, Vec<ParseError<'code, Token<'code>>>>;

// @TODO Return multiple errors
// @TODO How to resolve loops?
pub fn resolve_references(
    mut expressions: Vec<Spanned<Expression>>,
) -> ResolveResult<Vec<Spanned<Expression>>> {
    let mut reachable_referenceables = ReachableReferenceables::default();
    let level = 0;
    for mut expressions in &expressions {
        let Spanned { span, node: expression } = expressions;
        if let Expression::Variable(variable) = expression {
            let name = &variable.name;
            reachable_referenceables.insert(name, Referenceable { name, span: *span, level });
        }
    }
    for expression in &mut expressions {
        set_reference_count_and_alias_referenceables(expression, reachable_referenceables.clone(), level)?
    }
    Ok(expressions)
}

fn set_reference_count_and_alias_referenceables<'a, 'code>(
    mut expression: &'a mut Spanned<Expression<'code>>,
    mut reachable_referenceables: ReachableReferenceables<'code>,
    mut level: usize,
) -> ResolveResult<'code, ()> {
    let Spanned { span, node: expression } = &mut expression;
    match expression {
        Expression::Variable(variable) => {
            set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables, level)?;
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
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level)?;
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
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level)?;
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
                    set_reference_count_and_alias_referenceables(value, reachable_referenceables.clone(), level)?;
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
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level)?;
            }
            set_reference_count_and_alias_referenceables(output, reachable_referenceables, level)?;
        }
        Expression::List { items } => {
            for mut item in items {
                set_reference_count_and_alias_referenceables(item, reachable_referenceables.clone(), level)?;
            }
        }
        Expression::Map { entries } => {
            // @TODO implement, see the error message below
            Err(vec![ParseError::custom(*span, "Scope resolver cannot resolve references in Expression::Map yet, sorry".to_owned())])?
        }
        Expression::Latest { inputs } => {
            for mut input in inputs {
                set_reference_count_and_alias_referenceables(input, reachable_referenceables.clone(), level)?;
            }
        }
        Expression::Then { body } => {
            set_reference_count_and_alias_referenceables(body, reachable_referenceables, level)?;
        }
        Expression::When { arms } => {
            // @TODO implement, see the error message below
            Err(vec![ParseError::custom(*span, "Scope resolver cannot resolve references in Expression::When yet, sorry".to_owned())])?
        }
        Expression::While { arms } => {
            // @TODO implement, see the error message below
            Err(vec![ParseError::custom(*span, "Scope resolver cannot resolve references in Expression::While yet, sorry".to_owned())])?
        }
        Expression::Pipe { from, to } => {
            set_reference_count_and_alias_referenceables(from, reachable_referenceables.clone(), level)?;
            set_reference_count_and_alias_referenceables(to, reachable_referenceables, level)?;
        }
        Expression::ArithmeticOperator(_) => {
            // @TODO implement, see the error message below
            Err(vec![ParseError::custom(*span, "Scope resolver cannot resolve references in Expression::ArithmeticOperator yet, sorry".to_owned())])?
        },
        Expression::Comparator(_) => {
            // @TODO implement, see the error message below
            Err(vec![ParseError::custom(*span, "Scope resolver cannot resolve references in Expression::Comparator yet, sorry".to_owned())])?
        },
        Expression::Function { .. } => {
            // @TODO implement, see the error message below
            Err(vec![ParseError::custom(*span, "Scope resolver cannot resolve references in Expression::Function yet, sorry".to_owned())])?
        },
        Expression::Alias(alias) => {
            set_referenced_referenceable(alias, *span, reachable_referenceables)?
        }
        Expression::LinkSetter { alias } =>  {
            let Spanned { span, node: alias } = alias;
            set_referenced_referenceable(alias, *span, reachable_referenceables)?
        },
        Expression::Literal(_) => (),
        Expression::Link => (),
        Expression::Skip => (),
    }
    Ok(())
}

fn set_referenced_referenceable<'code>(
    alias: &mut Alias<'code>, 
    span: Span, 
    reachable_referenceables: ReachableReferenceables<'code>
) -> ResolveResult<'code, ()> {
    match alias {
        Alias::WithPassed { extra_parts } => Ok(()),
        Alias::WithoutPassed { parts, referenceables: unset_referenceables } => {
            // @TODO make the first part a standalone property (name or PASSED)?
            let first_part = parts.first().expect("Failed to get first alias part");
            // @TODO make Argument name optional to model the case when an argument is piped better?
            if first_part.is_empty() {
                return Ok(())
            }
            let referenced = reachable_referenceables.get(first_part).copied();
            let error = referenced.is_none().then(|| {
                let reachable_names = reachable_referenceables.keys();
                vec![ParseError::custom(span, format!("Cannot find the variable or argument '{first_part}'. You can refer to: {reachable_names:?}"))]
            });
            let referenceables = Referenceables {
                referenced,
                reachable: reachable_referenceables,
            };
            *unset_referenceables = Some(referenceables);
            if let Some(error) = error {
                Err(error)
            } else {
                Ok(())
            }
        }
    }
}
