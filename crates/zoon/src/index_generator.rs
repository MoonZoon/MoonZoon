use crate::*;
use std::{
    collections::BTreeSet,
    sync::atomic::{AtomicU32, Ordering},
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct IndexGenerator {
    index: AtomicU32,
    deleted: Arc<RwLock<BTreeSet<u32>>>,
}

impl IndexGenerator {
    /// - Returns the value 2 when 0 and 1 have been already returned.
    ///   (2 is the first higher unused value.)
    /// - Returns the value 2 when 0, 1, 2 and 3 have been returned, but 2 has been then removed.
    ///   (2 is the value of the first "gap" / deleted value.)  
    pub fn next_index(&self) -> u32 {
        // https://github.com/rust-lang/rust/issues/62924
        let lowest_deleted = self.deleted.read().unwrap_throw().iter().next().copied();
        if let Some(lowest_deleted) = lowest_deleted {
            self.deleted.write().unwrap_throw().remove(&lowest_deleted);
            return lowest_deleted;
        }
        self.index.fetch_add(1, Ordering::SeqCst)
    }

    pub fn remove_index(&self, index: u32) {
        self.deleted.write().unwrap_throw().insert(index);
    }
}
