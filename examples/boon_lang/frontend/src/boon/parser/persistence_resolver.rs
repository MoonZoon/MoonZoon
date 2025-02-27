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
    New,
    Unchanged,
    Changed,
}

pub type ResolveError<'code> = ParseError<'code, Token<'code>>;

// @TODO return diff to remove
pub fn resolve_persistence(
    mut expressions: Vec<Spanned<Expression>>,
) -> Result<Vec<Spanned<Expression>>, Vec<ResolveError>> {
    Ok(expressions)
}
