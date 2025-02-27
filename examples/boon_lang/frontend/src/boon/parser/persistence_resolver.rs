use super::{Alias, Expression, ParseError, Span, Spanned, Token};
use std::collections::{BTreeMap, HashSet};

#[derive(Debug, Clone, Copy)]
pub struct Persistence {
}

pub type ResolveError<'code> = ParseError<'code, Token<'code>>;

// @TODO return diff to remove
pub fn resolve_persistence(
    mut expressions: Vec<Spanned<Expression>>,
) -> Result<Vec<Spanned<Expression>>, Vec<ResolveError>> {
    Ok(expressions)
}
