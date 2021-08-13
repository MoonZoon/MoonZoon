use crate::*;
use std::{
    sync::{Arc, Mutex, MutexGuard},
    convert::TryFrom,
};

#[derive(Default)]
pub struct MonotonicIds {
    ids: Arc<Mutex<Vec<u32>>>,
    generator: IndexGenerator,
}

impl MonotonicIds {
    /// u32 is both id and index
    pub fn add_new_id(&self) -> (u32, MutexGuard<Vec<u32>>) {
        let mut ids = self.ids.lock().unwrap_throw();
        let id = self.generator.next_index();
        ids.insert(usize::try_from(id).unwrap_throw(), id);
        (id, ids)
    }

    /// usize is index
    pub fn remove_id(&self, id: u32) -> (usize, MutexGuard<Vec<u32>>)  {
        let mut ids = self.ids.lock().unwrap_throw();
        self.generator.remove_index(id);
        let index = ids.binary_search(&id).unwrap_throw();
        ids.remove(index);
        (index, ids)
    }
}
