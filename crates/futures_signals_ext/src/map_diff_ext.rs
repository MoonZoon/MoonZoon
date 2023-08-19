use crate::*;
use std::collections::BTreeMap;

pub trait MapDiffExt<K, V> {
    fn apply_to_map(self, map: &mut BTreeMap<K, V>);
}

impl<K: Ord, V> MapDiffExt<K, V> for MapDiff<K, V> {
    fn apply_to_map(self, map: &mut BTreeMap<K, V>) {
        match self {
            MapDiff::Replace { entries } => {
                *map = BTreeMap::from_iter(entries.into_iter());
            }
            MapDiff::Insert { key, value } | MapDiff::Update { key, value } => {
                map.insert(key, value);
            }
            MapDiff::Remove { key } => {
                map.remove(&key);
            }
            MapDiff::Clear {} => {
                map.clear();
            }
        }
    }
}
