use crate::call_tree;
use std::hash::Hash;
use std::borrow::Borrow;

#[call_tree]
pub fn call_in_slot<F, Q, R, S>(slot: &Q, op: F) -> R
where
    F: FnOnce() -> R,
    Q: Eq + Hash + ToOwned<Owned = S> + ?Sized,
    S: Borrow<Q> + Eq + Hash + Send + 'static,
{
    // Scope::with_current(|p| p.make_child(Callsite::here(), slot)).enter(op)
    op()
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CallId {
    // callsite: Callsite,
    // parent: Slot<CallId>,
    // slot: OpaqueSlot,
}

impl CallId {
    /// Returns the root `CallId`.
    pub(crate) fn root() -> Self {
        Self {
            // callsite: Callsite::here(),
            // parent: Slot::fake(),
            // slot: Slot::<String>::fake().into(),
        }
    }

    /// Returns the current `CallId`.
    pub fn current() -> Self {
        // Scope::with_current(|current| current.id)
        CallId {}
    }

    pub(crate) fn child<Q, S>(&self, callsite: Callsite, slot: &Q) -> Self
    where
        Q: Eq + Hash + ToOwned<Owned = S> + ?Sized,
        S: Borrow<Q> + Eq + Hash + Send + 'static,
    {
        // Self { callsite, parent: Slot::make(self), slot: Slot::make(slot).into() }
        CallId {}
    }
}

/// A value unique to the source location where it is created.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub(crate) struct Callsite {
    location: usize,
}
