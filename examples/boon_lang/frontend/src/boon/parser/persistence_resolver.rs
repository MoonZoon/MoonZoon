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

// @TODO return diff to remove
pub fn resolve_persistence<'new_code, 'old_code>(
    mut new_expressions: Vec<Spanned<Expression<'new_code>>>,
    mut old_expressions: Option<Vec<Spanned<Expression<'old_code>>>>,
) -> Result<Vec<Spanned<Expression<'new_code>>>, Vec<ResolveError<'new_code>>> {
    Ok(new_expressions)
}
