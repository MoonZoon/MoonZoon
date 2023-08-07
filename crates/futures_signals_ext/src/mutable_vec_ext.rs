use crate::futures_signals::signal_vec::{MutableVecLockMut, MutableVecLockRef};
use crate::*;

pub trait MutableVecExt<T>: private::MutableVecExt<T> {
    fn update_mut(&self, f: impl FnOnce(&mut MutableVecLockMut<T>)) {
        f(&mut self.lock_mut())
    }

    fn use_ref(&self, f: impl FnOnce(&MutableVecLockRef<T>)) {
        f(&self.lock_ref())
    }
}
impl<T> MutableVecExt<T> for MutableVec<T> {}

mod private {
    use super::*;
    pub trait MutableVecExt<T> {
        fn lock_mut(&self) -> MutableVecLockMut<T>;
        fn lock_ref(&self) -> MutableVecLockRef<T>;
    }
    impl<T> MutableVecExt<T> for MutableVec<T> {
        fn lock_mut(&self) -> MutableVecLockMut<T> {
            self.lock_mut()
        }
        fn lock_ref(&self) -> MutableVecLockRef<T> {
            self.lock_ref()
        }
    }
}
