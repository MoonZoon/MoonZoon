use crate::*;
use std::{pin::Pin, task::{Context, Poll}, marker::PhantomData};

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
}

impl<T: Signal<Item = bool>> SignalExtBool for T {
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

// ------ SignalExtExt ------

pub trait SignalExtExt: SignalExt {
    #[inline]
    fn for_each_sync<F>(self, mut callback: F) -> ForEachSync<Self, F>
        where 
            F: FnMut(Self::Item) + 'static,
            Self: 'static + Sized
    {
        ForEachSync { 
            future: self
                .for_each(move |item| {
                    callback(item);
                    async {}
                })
                .boxed_local(),
            signal: PhantomData,
            callback: PhantomData,
        }
    }
}

impl<T: SignalExt> SignalExtExt for T {}

// -- ForEachSync --

#[pin_project(project = ForEachSyncProj)]
#[must_use = "Futures do nothing unless polled"]
pub struct ForEachSync<S, F> {
    #[pin]
    future: future::LocalBoxFuture<'static, ()>,
    signal: PhantomData<S>,
    callback: PhantomData<F>,
}

impl<S: Signal, F: FnMut(S::Item)> Future for ForEachSync<S, F> {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}
