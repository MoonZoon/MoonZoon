use crate::futures_signals::signal_map::{MutableBTreeMapLockMut, MutableBTreeMapLockRef};
use crate::*;
use std::collections::BTreeMap;

pub trait MutableBTreeMapExt<K, V>: private::MutableBTreeMapExt<K, V> {
    fn update_mut(&self, f: impl FnOnce(&mut MutableBTreeMapLockMut<K, V>)) {
        f(&mut self.lock_mut())
    }

    fn use_ref(&self, f: impl FnOnce(&MutableBTreeMapLockRef<K ,V>)) {
        f(&self.lock_ref())
    }

    fn take(&self) -> BTreeMap<K, V> where K: Ord + Clone {
        // @TODO better way (`Mutable.0` + `mem::take`?) (`futures-signals` update needed?)
        let mut lock = self.lock_mut();
        let mut new_map = BTreeMap::new();
        while let Some((key, _)) = lock.last_key_value() {
            let key = key.clone();
            let value = lock.remove(&key).unwrap();
            new_map.insert(key, value);
        }
        new_map
    }
}
impl<K, V> MutableBTreeMapExt<K, V> for MutableBTreeMap<K, V> {}

mod private {
    use super::*;
    pub trait MutableBTreeMapExt<K, V> {
        fn lock_mut(&self) -> MutableBTreeMapLockMut<K, V>;
        fn lock_ref(&self) -> MutableBTreeMapLockRef<K, V>;
    }
    impl<K, V> MutableBTreeMapExt<K, V> for MutableBTreeMap<K, V> {
        fn lock_mut(&self) -> MutableBTreeMapLockMut<K, V> {
            self.lock_mut()
        }
        fn lock_ref(&self) -> MutableBTreeMapLockRef<K, V> {
            self.lock_ref()
        }
    }
}
