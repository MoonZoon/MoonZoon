use std::collections::BTreeMap;
use super::{Expression, Spanned, ParseError, Token, Span};

pub type Referenceables<'code> = BTreeMap<ReferenceableName<'code>, ReferenceablePosition>;
pub type ReferenceableName<'code> = &'code str;
#[derive(Debug, Clone, Copy)]
pub struct ReferenceablePosition { pub span: Span, pub level: usize }

type ResolveResult<'code, T> = Result<T, Vec<ParseError<'code, Token<'code>>>>;

pub fn resolve_references(expressions: Vec<Spanned<Expression>>) -> ResolveResult<Vec<Spanned<Expression>>> {
    Ok(expressions)
    // @TODO remove
    // Err(vec![ParseError::custom(Span::splat(0), "Dummy error".to_owned())])
}
