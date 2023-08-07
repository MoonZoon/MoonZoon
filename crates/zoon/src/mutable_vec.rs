use crate::*;
use futures_signals::signal_vec::{
    MutableVec as FSMutableVec, MutableVecLockMut, MutableVecLockRef,
};
use std::ops::Deref;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MutableVec<T>(FSMutableVec<T>);

impl<T> Clone for MutableVec<T> {
    fn clone(&self) -> Self {
        MutableVec(self.0.clone())
    }
}

impl<T> MutableVec<T> {
    pub fn new() -> Self {
        Self(FSMutableVec::new())
    }

    pub fn new_with_values(values: Vec<T>) -> Self {
        Self(FSMutableVec::new_with_values(values))
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(FSMutableVec::with_capacity(capacity))
    }

    pub fn update_mut(&self, f: impl FnOnce(&mut MutableVecLockMut<T>)) {
        f(&mut self.lock_mut())
    }

    pub fn use_ref(&self, f: impl FnOnce(&MutableVecLockRef<T>)) {
        f(&self.lock_ref())
    }
}

impl<T> Deref for MutableVec<T> {
    type Target = FSMutableVec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<FSMutableVec<T>> for MutableVec<T> {
    fn from(mutable_vec: FSMutableVec<T>) -> Self {
        Self(mutable_vec)
    }
}

impl<T> From<MutableVec<T>> for FSMutableVec<T> {
    fn from(mutable_vec: MutableVec<T>) -> Self {
        mutable_vec.0
    }
}
