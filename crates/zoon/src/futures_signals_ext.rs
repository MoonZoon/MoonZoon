use crate::*;
use futures_signals::{
    signal::{Mutable, MutableSignal, MutableSignalCloned},
    signal_vec::{MutableVec, MutableVecLockMut, MutableVecLockRef},
};
use std::mem;
use std::pin::Pin;
use std::task::{Context, Poll};

// ------ MutableExt ------

pub trait MutableExt<A> {
    fn map<B>(&self, f: impl FnOnce(&A) -> B) -> B;

    fn map_mut<B>(&self, f: impl FnOnce(&mut A) -> B) -> B;

    fn update(&self, f: impl FnOnce(A) -> A)
    where
        A: Copy;

    fn update_mut(&self, f: impl FnOnce(&mut A));

    fn take(&self) -> A
    where
        A: Default,
    {
        self.map_mut(mem::take)
    }

    fn new_and_signal(value: A) -> (Self, MutableSignal<A>)
    where
        A: Copy,
        Self: Sized;

    fn new_and_signal_cloned(value: A) -> (Self, MutableSignalCloned<A>)
    where
        A: Clone,
        Self: Sized;
}

impl<A> MutableExt<A> for Mutable<A> {
    #[inline]
    fn map<B>(&self, f: impl FnOnce(&A) -> B) -> B {
        f(&self.lock_ref())
    }

    #[inline]
    fn map_mut<B>(&self, f: impl FnOnce(&mut A) -> B) -> B {
        f(&mut self.lock_mut())
    }

    #[inline]
    fn update(&self, f: impl FnOnce(A) -> A)
    where
        A: Copy,
    {
        self.set(f(self.get()))
    }

    #[inline]
    fn update_mut(&self, f: impl FnOnce(&mut A)) {
        f(&mut self.lock_mut())
    }

    fn new_and_signal(value: A) -> (Self, MutableSignal<A>)
    where
        A: Copy,
    {
        let this = Self::new(value);
        let signal = this.signal();
        (this, signal)
    }

    fn new_and_signal_cloned(value: A) -> (Self, MutableSignalCloned<A>)
    where
        A: Clone,
    {
        let this = Self::new(value);
        let signal = this.signal_cloned();
        (this, signal)
    }
}

// ------ MutableVecExt ------

pub trait MutableVecExt<A> {
    fn update_mut(&self, f: impl FnOnce(&mut MutableVecLockMut<A>));

    fn use_ref(&self, f: impl FnOnce(&MutableVecLockRef<A>));
}

impl<A> MutableVecExt<A> for MutableVec<A> {
    #[inline]
    fn update_mut(&self, f: impl FnOnce(&mut MutableVecLockMut<A>)) {
        f(&mut self.lock_mut())
    }

    #[inline]
    fn use_ref(&self, f: impl FnOnce(&MutableVecLockRef<A>)) {
        f(&self.lock_ref())
    }
}

// ------ SignalExtMapBool ------

pub trait SignalExtMapBool {
    fn map_bool<B, TM: FnMut() -> B, FM: FnMut() -> B>(self, t: TM, f: FM) -> MapBool<Self, TM, FM> where Self: Sized;
}

impl<T: Signal<Item = bool>> SignalExtMapBool for T {
    #[inline]
    fn map_bool<B, TM: FnMut() -> B, FM: FnMut() -> B>(self, t: TM, f: FM) -> MapBool<Self, TM, FM> where Self: Sized {
        MapBool {
            signal: self,
            true_mapper: t,
            false_mapper: f,
        }
    }
}

#[pin_project(project = MapBoolProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapBool<S, TM, FM> {
    #[pin]
    signal: S,
    true_mapper: TM,
    false_mapper: FM,
}

impl<I, S: Signal<Item = bool>, TM: FnMut() -> I, FM: FnMut() -> I> Signal for MapBool<S, TM, FM> {
    type Item = I;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let MapBoolProj { signal, true_mapper, false_mapper } = self.project();

        signal.poll_change(cx).map(|opt| opt.map(|value| {
            if value {
                true_mapper()
            } else {
                false_mapper()
            }
        }))
    }
}



