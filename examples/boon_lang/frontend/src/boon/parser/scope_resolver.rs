use super::{Expression, ParseError, Span, Spanned, Token, Alias};
use std::collections::BTreeMap;

// @TODO immutable or different tree traversal algorithm?
pub type ReachableReferenceables<'code> = BTreeMap<&'code str, Referenceable<'code>>;

#[derive(Debug)]
pub struct Referenceables<'code> {
    referenced: Option<Referenceable<'code>>,
    reachable: ReachableReferenceables<'code>,
}

#[derive(Debug, Clone, Copy)]
pub struct Referenceable<'code> {
    name: &'code str,
    span: Span,
    level: usize,
}

pub type ResolveResult<'code, T> = Result<T, Vec<ParseError<'code, Token<'code>>>>;

// @TODO Batch multiple errors?
pub fn resolve_references(
    mut expressions: Vec<Spanned<Expression>>,
) -> ResolveResult<Vec<Spanned<Expression>>> {
    let mut reachable_referenceables = ReachableReferenceables::default();
    reachable_referenceables.insert("dummy_name", Referenceable { name: "dummy_name", span: Span::splat(1), level: 123 });
    for expression in &mut expressions {
        set_reference_count_and_alias_referenceables(expression, reachable_referenceables.clone(), 0)?
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
            for mut variable in &mut object.variables {
                let Spanned { span, node: variable } = &mut variable;
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level)?;
            }
        }
        Expression::TaggedObject { tag, object } => {
            level += 1;
            for mut variable in &mut object.variables {
                let Spanned { span, node: variable } = &mut variable;
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level)?;
            }
        }
        Expression::FunctionCall { path, arguments } => {
            level += 1;
            for mut argument in arguments {
                let Spanned { span, node: argument } = &mut argument;
                if let Some(value) = argument.value.as_mut() {
                    set_reference_count_and_alias_referenceables(value, reachable_referenceables.clone(), level)?;
                }
            }
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
        Expression::Block { variables, output } => {
            level += 1;
            for mut variable in variables {
                let Spanned { span, node: variable } = &mut variable;
                set_reference_count_and_alias_referenceables(&mut variable.value, reachable_referenceables.clone(), level)?;
            }
            set_reference_count_and_alias_referenceables(output, reachable_referenceables, level)?;
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
            match alias {
                Alias::WithPassed { extra_parts } => (),
                Alias::WithoutPassed { parts, referenceables: unset_referenceables } => {
                    let referenceables = Referenceables {
                        // @TODO resolve referenced
                        referenced: None,
                        reachable: reachable_referenceables,
                    };
                    *unset_referenceables = Some(referenceables);
                }
            }
        }
        Expression::LinkSetter { alias } =>  {
            let Spanned { span, node: alias } = alias;
            match alias {
                Alias::WithPassed { extra_parts } => (),
                Alias::WithoutPassed { parts, referenceables: unset_referenceables } => {
                    let referenceables = Referenceables {
                        // @TODO resolve referenced
                        referenced: None,
                        reachable: reachable_referenceables,
                    };
                    *unset_referenceables = Some(referenceables);
                }
            }
        },
        Expression::Literal(_) => (),
        Expression::Link => (),
        Expression::Skip => (),
    }
    Ok(())
}
