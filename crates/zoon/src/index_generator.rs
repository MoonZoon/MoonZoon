use crate::*;
use std::{
    collections::BTreeSet,
    sync::atomic::{AtomicU32, Ordering},
    sync::{Arc, RwLock},
};

#[derive(Default)]
pub struct IndexGenerator {
    index: AtomicU32,
    freed: Arc<RwLock<BTreeSet<u32>>>,
}

impl IndexGenerator {
    pub fn new() -> Self {
        Self::default()
    }

    /// - Returns the value 2 when 0 and 1 have been already returned.
    ///   (2 is the first higher unused value.)
    /// - Returns the value 2 when 0, 1, 2 and 3 have been generated, but then 2 has been freed.
    ///   (2 is the value of the first "gap" / freed value.)  
    pub fn next_index(&self) -> u32 {
        // https://github.com/rust-lang/rust/issues/62924
        let lowest_freed = self.freed.read().unwrap_throw().iter().next().copied();
        if let Some(lowest_freed) = lowest_freed {
            self.freed.write().unwrap_throw().remove(&lowest_freed);
            return lowest_freed;
        }
        self.index.fetch_add(1, Ordering::SeqCst)
    }

    pub fn free_index(&self, index: u32) {
        self.freed.write().unwrap_throw().insert(index);
    }
}
