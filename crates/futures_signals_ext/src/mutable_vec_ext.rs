use crate::futures_signals::signal_vec::{MutableVecLockMut, MutableVecLockRef};
use crate::*;

pub trait MutableVecExt<T>: private::MutableVecExt<T> {
    fn update_mut(&self, f: impl FnOnce(&mut MutableVecLockMut<T>)) {
        f(&mut self.lock_mut())
    }

    fn use_ref(&self, f: impl FnOnce(&MutableVecLockRef<T>)) {
        f(&self.lock_ref())
    }

    fn take(&self) -> Vec<T> {
        // @TODO better way (`Mutable.0` + `mem::take`?) (`futures-signals` update needed?)
        let mut lock = self.lock_mut();
        let new_vec = lock.drain(..).collect();
        lock.shrink_to_fit();
        new_vec
    }
}
impl<T> MutableVecExt<T> for MutableVec<T> {}

mod private {
    use super::*;
    pub trait MutableVecExt<T> {
        fn lock_mut(&self) -> MutableVecLockMut<'_, T>;
        fn lock_ref(&self) -> MutableVecLockRef<'_, T>;
    }
    impl<T> MutableVecExt<T> for MutableVec<T> {
        fn lock_mut(&self) -> MutableVecLockMut<'_, T> {
            self.lock_mut()
        }
        fn lock_ref(&self) -> MutableVecLockRef<'_, T> {
            self.lock_ref()
        }
    }
}
