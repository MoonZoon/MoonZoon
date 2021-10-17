use crate::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

// ------ SignalExtBool ------

pub trait SignalExtBool {
    fn map_bool<B, TM: FnMut() -> B, FM: FnMut() -> B>(self, t: TM, f: FM) -> MapBool<Self, TM, FM>
    where
        Self: Sized;

    fn map_true<B, F: FnMut() -> B>(self, f: F) -> MapTrue<Self, F>
    where
        Self: Sized;

    fn map_false<B, F: FnMut() -> B>(self, f: F) -> MapFalse<Self, F>
    where
        Self: Sized;

    fn map_bool_signal<I, TS, FS, TM, FM>(self, t: TM, f: FM) -> MapBoolSignal<Self, I, TS, FS>
    where
        TS: Signal<Item = I>,
        FS: Signal<Item = I>,
        TM: FnMut() -> TS + 'static, 
        FM: FnMut() -> FS + 'static,
        Self: Sized + Signal<Item = bool>;

    fn map_true_signal<I, MS, F>(self, f: F) -> MapTrueSignal<Self, I, MS>
    where
        MS: Signal<Item = I>,
        F: FnMut() -> MS + 'static,
        Self: Sized + Signal<Item = bool>;

    fn map_false_signal<I, MS, F>(self, f: F) -> MapFalseSignal<Self, I, MS>
    where
        MS: Signal<Item = I>,
        F: FnMut() -> MS + 'static,
        Self: Sized + Signal<Item = bool>;
}

impl<S: Signal<Item = bool>> SignalExtBool for S {
    #[inline]
    fn map_bool<B, TM: FnMut() -> B, FM: FnMut() -> B>(self, t: TM, f: FM) -> MapBool<Self, TM, FM>
    where
        Self: Sized,
    {
        MapBool {
            signal: self,
            true_mapper: t,
            false_mapper: f,
        }
    }

    #[inline]
    fn map_true<B, F: FnMut() -> B>(self, f: F) -> MapTrue<Self, F>
    where
        Self: Sized,
    {
        MapTrue { signal: self, f }
    }

    #[inline]
    fn map_false<B, F: FnMut() -> B>(self, f: F) -> MapFalse<Self, F>
    where
        Self: Sized,
    {
        MapFalse { signal: self, f }
    }

    #[inline]
    fn map_bool_signal<I, TS, FS, TM, FM>(self, mut t: TM, mut f: FM) -> MapBoolSignal<Self, I, TS, FS>
    where
        TS: Signal<Item = I>,
        FS: Signal<Item = I>,
        TM: FnMut() -> TS + 'static, 
        FM: FnMut() -> FS + 'static,
        Self: Sized + Signal<Item = bool>
    {
        MapBoolSignal {
            inner: self.map_bool(
                Box::new(move || t().left_either()) as Box<dyn FnMut() -> _>, 
                Box::new(move || f().right_either()) as Box<dyn FnMut() -> _>
            ).flatten(),
        }
    }

    #[inline]
    fn map_true_signal<I, MS, F>(self, mut f: F) -> MapTrueSignal<Self, I, MS>
    where
        MS: Signal<Item = I>,
        F: FnMut() -> MS + 'static,
        Self: Sized + Signal<Item = bool>
    {
        MapTrueSignal { 
            inner: self.map_bool_signal(
                move || f().map(Some as fn(_) -> _),
                || always(None),
            ),
        }
    }

    #[inline]
    fn map_false_signal<I, MS, F>(self, mut f: F) -> MapFalseSignal<Self, I, MS>
    where
        MS: Signal<Item = I>,
        F: FnMut() -> MS + 'static,
        Self: Sized + Signal<Item = bool>
    {
        MapFalseSignal { 
            inner: self.map_bool_signal(
                || always(None),
                move || f().map(Some as fn(_) -> _),
            ),
        }
    }
}

// -- MapBool --

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
        let MapBoolProj {
            signal,
            true_mapper,
            false_mapper,
        } = self.project();

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| if value { true_mapper() } else { false_mapper() }))
    }
}

// -- MapTrue --

#[pin_project(project = MapTrueProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapTrue<S, F> {
    #[pin]
    signal: S,
    f: F,
}

impl<I, S: Signal<Item = bool>, F: FnMut() -> I> Signal for MapTrue<S, F> {
    type Item = Option<I>;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let MapTrueProj { signal, f } = self.project();

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| value.then(f)))
    }
}

// -- MapFalse --

#[pin_project(project = MapFalseProj)]
#[derive(Debug)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapFalse<S, F> {
    #[pin]
    signal: S,
    f: F,
}

impl<I, S: Signal<Item = bool>, F: FnMut() -> I> Signal for MapFalse<S, F> {
    type Item = Option<I>;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let MapFalseProj { signal, f } = self.project();

        signal
            .poll_change(cx)
            .map(|opt| opt.map(|value| not(value).then(f)))
    }
}

// -- MapBoolSignal --

#[pin_project(project = MapBoolSignalProj)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapBoolSignal<S, I, TS, FS>
where
    S: Signal<Item = bool>,
    TS: Signal<Item = I>,
    FS: Signal<Item = I>,
{
    #[pin]
    inner: signal::Flatten<MapBool<S, Box<dyn FnMut() -> Either<TS, FS>>, Box<dyn FnMut() -> Either<TS, FS>>>>
}

impl<S, I, TS, FS> Signal for MapBoolSignal<S, I, TS, FS>
    where
    S: Signal<Item = bool>,
    TS: Signal<Item = I>,
    FS: Signal<Item = I>,
{
    type Item = I;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_change(cx)
    }
}

// -- MapTrueSignal --

#[pin_project(project = MapTrueSignalProj)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapTrueSignal<S, I, MS>
    where
    S: Signal<Item = bool>,
    MS: Signal<Item = I>,
{
    #[pin]
    inner: MapBoolSignal<
        S, 
        Option<I>, 
        signal::Map<MS, fn(I) -> Option<I>>, 
        signal::Always<Option<I>>,
    >,
}

impl<S, I, MS> Signal for MapTrueSignal<S, I, MS> 
    where
    S: Signal<Item = bool>,
    MS: Signal<Item = I>,
{
    type Item = Option<I>;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_change(cx)
    }
}

#[pin_project(project = MapFalseSignalProj)]
#[must_use = "Signals do nothing unless polled"]
pub struct MapFalseSignal<S, I, MS>
    where
    S: Signal<Item = bool>,
    MS: Signal<Item = I>,
{
    #[pin]
    inner: MapBoolSignal<
        S, 
        Option<I>, 
        signal::Always<Option<I>>,
        signal::Map<MS, fn(I) -> Option<I>>, 
    >,
}

impl<S, I, MS> Signal for MapFalseSignal<S, I, MS> 
    where
    S: Signal<Item = bool>,
    MS: Signal<Item = I>,
{
    type Item = Option<I>;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        self.project().inner.poll_change(cx)
    }
}
