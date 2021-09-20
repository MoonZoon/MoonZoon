use crate::*;
use std::{pin::Pin, task::{Context, Poll}};

// ------ SignalExtOption ------

pub trait SignalExtOption<T> {
    fn map_option<B, SM: FnMut(T) -> B, NM: FnMut() -> B>(self, s: SM, n: NM) -> MapOption<Self, SM, NM>
    where
        Self: Sized;

    fn map_some<B, F: FnMut(T) -> B>(self, f: F) -> MapSome<Self, F>
    where
        Self: Sized;

    fn map_none<B, F: FnMut() -> B>(self, f: F) -> MapNone<Self, F>
    where
        Self: Sized;
}

impl<T, S: Signal<Item = Option<T>>> SignalExtOption<T> for S {
    #[inline]
    fn map_option<B, SM: FnMut(T) -> B, NM: FnMut() -> B>(self, s: SM, n: NM) -> MapOption<Self, SM, NM>
    where
        Self: Sized,
    {
        MapOption {
            signal: self,
            some_mapper: s,
            none_mapper: n,
        }
    }

    #[inline]
    fn map_some<B, F: FnMut(T) -> B>(self, f: F) -> MapSome<Self, F>
    where
        Self: Sized,
    {
        MapSome { signal: self, f }
    }

    #[inline]
    fn map_none<B, F: FnMut() -> B>(self, f: F) -> MapNone<Self, F>
    where
        Self: Sized,
    {
        MapNone { signal: self, f }
    }
}

// -- MapOption --

#[pin_project(project = MapOptionProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapOption<S, SM, NM> {
    #[pin]
    signal: S,
    some_mapper: SM,
    none_mapper: NM,
}

impl<T, I, S: Signal<Item = Option<T>>, SM: FnMut(T) -> I, NM: FnMut() -> I> Signal for MapOption<S, SM, NM> {
    type Item = I;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let MapOptionProj {
            signal,
            some_mapper,
            none_mapper,
        } = self.project();

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| value.map_or_else(none_mapper, some_mapper)))
    }
}

// -- MapSome --

#[pin_project(project = MapSomeProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapSome<S, F> {
    #[pin]
    signal: S,
    f: F,
}

impl<T, I, S: Signal<Item = Option<T>>, F: FnMut(T) -> I> Signal for MapSome<S, F> {
    type Item = Option<I>;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let MapSomeProj { signal, f } = self.project();

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| value.map(f)))
    }
}

// -- MapNone --

#[pin_project(project = MapNoneProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapNone<S, F> {
    #[pin]
    signal: S,
    f: F,
}

impl<T, I, S: Signal<Item = Option<T>>, F: FnMut() -> I> Signal for MapNone<S, F> {
    type Item = Option<I>;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let MapNoneProj { signal, f } = self.project();

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| {
                if value.is_some() {
                    None
                } else {
                    Some(f())
                }
            }))
    }
}
