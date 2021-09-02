use crate::*;
use futures_signals::signal::{Mutable as FSMutable, MutableLockRef, MutableSignalCloned};
use std::mem;
use std::ops::Deref;

#[derive(Debug, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mutable<T>(FSMutable<T>);

impl<T> Clone for Mutable<T> {
    fn clone(&self) -> Self {
        Mutable(self.0.clone())
    }
}

impl<T> Mutable<T> {
    pub fn new(value: T) -> Self {
        Self(FSMutable::new(value))
    }

    pub fn map<B>(&self, f: impl FnOnce(&T) -> B) -> B {
        f(&self.lock_ref())
    }

    pub fn map_mut<B>(&self, f: impl FnOnce(&mut T) -> B) -> B {
        f(&mut self.lock_mut())
    }

    pub fn update(&self, f: impl FnOnce(T) -> T)
    where
        T: Copy,
    {
        self.set(f(self.get()))
    }

    pub fn update_mut(&self, f: impl FnOnce(&mut T)) {
        f(&mut self.lock_mut())
    }

    pub fn take(&self) -> T
    where
        T: Default,
    {
        self.map_mut(mem::take)
    }

    pub fn use_ref(&self, f: impl FnOnce(&MutableLockRef<T>)) {
        f(&self.lock_ref())
    }

    pub fn new_and_signal(value: T) -> (Self, MutableSignal<T>)
    where
        T: Copy,
    {
        let this = Self::new(value);
        let signal = this.signal();
        (this, signal)
    }

    pub fn new_and_signal_cloned(value: T) -> (Self, MutableSignalCloned<T>)
    where
        T: Clone,
    {
        let this = Self::new(value);
        let signal = this.signal_cloned();
        (this, signal)
    }
}

impl<T> Deref for Mutable<T> {
    type Target = FSMutable<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<FSMutable<T>> for Mutable<T> {
    fn from(mutable: FSMutable<T>) -> Self {
        Self(mutable)
    }
}

impl<T> From<Mutable<T>> for FSMutable<T> {
    fn from(mutable: Mutable<T>) -> Self {
        mutable.0
    }
}

#[cfg(feature = "serde-lite")]
impl<T: Serialize> Serialize for Mutable<T> {
    fn serialize(&self) -> Result<serde_lite::Intermediate, serde_lite::Error> {
        self.lock_ref().serialize()
    }
}

#[cfg(feature = "serde-lite")]
impl<T: Deserialize> Deserialize for Mutable<T> {
    fn deserialize(itermediate: &serde_lite::Intermediate) -> Result<Self, serde_lite::Error> {
        T::deserialize(itermediate).map(Self::new)
    }
}
