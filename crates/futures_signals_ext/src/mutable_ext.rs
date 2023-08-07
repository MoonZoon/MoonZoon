use crate::futures_signals::signal::{
    MutableLockMut, MutableLockRef, MutableSignal, MutableSignalCloned,
};
use crate::*;
use std::mem;
use std::ops::Deref;

pub trait MutableExt<T>: private::MutableExt<T> {
    fn map<B>(&self, f: impl FnOnce(&T) -> B) -> B {
        f(&self.lock_ref())
    }

    fn map_mut<B>(&self, f: impl FnOnce(&mut T) -> B) -> B {
        f(&mut self.lock_mut())
    }

    fn update(&self, f: impl FnOnce(T) -> T)
    where
        T: Copy,
    {
        self.set(f(self.get()))
    }

    fn update_mut(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.lock_mut())
    }

    fn take(&self) -> T
    where
        T: Default,
    {
        self.map_mut(mem::take)
    }

    fn use_ref(&self, f: impl FnOnce(&MutableLockRef<T>)) {
        f(&self.lock_ref())
    }

    fn new_and_signal(value: T) -> (Mutable<T>, MutableSignal<T>)
    where
        T: Copy,
    {
        let this = Mutable::new(value);
        let signal = this.signal();
        (this, signal)
    }

    fn new_and_signal_cloned(value: T) -> (Mutable<T>, MutableSignalCloned<T>)
    where
        T: Clone,
    {
        let this = Mutable::new(value);
        let signal = this.signal_cloned();
        (this, signal)
    }
}

impl<T> MutableExt<T> for Mutable<T> {}

mod private {
    use super::*;
    pub trait MutableExt<T> {
        fn lock_mut(&self) -> MutableLockMut<T>;
        fn lock_ref(&self) -> MutableLockRef<T>;
        fn set(&self, value: T);
        fn get(&self) -> T
        where
            T: Copy;
    }
    impl<T> MutableExt<T> for Mutable<T> {
        fn lock_mut(&self) -> MutableLockMut<T> {
            self.lock_mut()
        }
        fn lock_ref(&self) -> MutableLockRef<T> {
            self.deref().lock_ref()
        }
        fn set(&self, value: T) {
            self.set(value)
        }
        fn get(&self) -> T
        where
            T: Copy,
        {
            self.deref().get()
        }
    }
}
