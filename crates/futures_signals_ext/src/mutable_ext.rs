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

    fn map_cloned<B>(&self, f: impl FnOnce(T) -> B) -> B
    where
        T: Clone,
    {
        f(self.get_cloned())
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

    fn unwrap_cloned(&self) -> T::Unwrapped
    where
        T: private::Unwrappable + Clone,
    {
        self.get_cloned().unwrap()
    }

    fn unwrap(&self) -> T::Unwrapped
    where
        T: private::Unwrappable + Copy,
    {
        self.get().unwrap()
    }
}

impl<T> MutableExt<T> for Mutable<T> {}

mod private {
    use super::*;

    pub trait Unwrappable {
        type Unwrapped;
        fn unwrap(self) -> Self::Unwrapped;
    }
    impl<T> Unwrappable for Option<T> {
        type Unwrapped = T;
        fn unwrap(self) -> Self::Unwrapped {
            // @TODO `.unwrap_throw()` in a browser
            self.unwrap()
        }
    }

    pub trait MutableExt<T> {
        fn lock_mut(&self) -> MutableLockMut<T>;
        fn lock_ref(&self) -> MutableLockRef<T>;
        fn set(&self, value: T);
        fn get_cloned(&self) -> T
        where
            T: Clone;
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
        fn get_cloned(&self) -> T
        where
            T: Clone,
        {
            self.deref().get_cloned()
        }
        fn get(&self) -> T
        where
            T: Copy,
        {
            self.deref().get()
        }
    }
}
