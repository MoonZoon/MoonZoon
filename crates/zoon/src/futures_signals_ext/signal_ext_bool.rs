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

    fn map_bool_signal<I, TS, FS, TM, FM>(self, t: TM, f: FM) -> Pin<Box<dyn Signal<Item = I>>>
    where
        TS: Signal<Item = I>,
        FS: Signal<Item = I>,
        TM: FnMut() -> TS + 'static, 
        FM: FnMut() -> FS + 'static,
        Self: Sized + 'static;

    // fn map_true_signal<I, S, F>(self, f: F) -> MapTrueSignal<Self, I, S, F>
    // where
    //     S: Signal<Item = I>,
    //     F: FnMut() -> S,
    //     Self: Sized;

    // fn map_false_signal<I, S, F>(self, f: F) -> MapFalseSignal<Self, I, S, F>
    // where
    //     S: Signal<Item = I>,
    //     F: FnMut() -> S,
    //     Self: Sized;
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
    fn map_bool_signal<I, TS, FS, TM, FM>(self, mut t: TM, mut f: FM) -> Pin<Box<dyn Signal<Item = I>>>
    where
        TS: Signal<Item = I>,
        FS: Signal<Item = I>,
        TM: FnMut() -> TS + 'static, 
        FM: FnMut() -> FS + 'static,
        Self: Sized + 'static
    {
        Box::pin(self.map_bool(move || t().left_either(), move || f().right_either()).flatten())
    }

    // #[inline]
    // fn map_true_signal<I, S, F>(self, f: F) -> MapTrueSignal<Self, I, S, F>
    // where
    //     S: Signal<Item = I>,
    //     F: FnMut() -> S,
    //     Self: Sized
    // {
    //     MapTrueSignal { signal: self, f }
    // }

    // #[inline]
    // fn map_false_signal<I, S, F>(self, f: F) -> MapFalseSignal<Self, I, S, F>
    // where
    //     S: Signal<Item = I>,
    //     F: FnMut() -> S,
    //     Self: Sized
    // {
    //     MapFalseSignal { signal: self, f }
    // }
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

// #[pin_project(project = MapBoolSignalProj)]
// // #[derive(Debug)]
// #[must_use = "Signals do nothing unless polled"]
// pub struct MapBoolSignal<I> {
//     #[pin]
//     inner: Pin<Box<dyn Signal<Item = I> + Unpin>>
// }

// impl<I> Signal for MapBoolSignal<I> {
//     type Item = I;

//     #[inline]
//     fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
//         // let MapBoolSignalProj {
//         //     signal,
//         //     true_mapper_signal,
//         //     false_mapper_signal,
//         // } = self.project();

//         // crate::println!("Poll!");
//     //    match signal.poll_change(cx) {
//         self.project().inner.poll_change(cx)
//         // }
//     }
// }

// // -- MapTrueSignal --

// #[pin_project(project = MapTrueSignalProj)]
// #[derive(Debug)]
// #[must_use = "Signals do nothing unless polled"]
// pub struct MapTrueSignal<S, F> {
//     #[pin]
//     signal: S,
//     f: F,
// }

// impl<I, S: Signal<Item = bool>, F: FnMut() -> I> Signal for MapTrueSignal<S, F> {
//     type Item = Option<I>;

//     #[inline]
//     fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
//         let MapTrueSignalProj { signal, f } = self.project();

//         signal
//             .poll_change(cx)
//             .map(|opt| opt.map(|value| value.then(f)))
//     }
// }

// // -- MapFalseSignal --

// #[pin_project(project = MapFalseSignalProj)]
// #[derive(Debug)]
// #[must_use = "Signals do nothing unless polled"]
// pub struct MapFalseSignal<S, F> {
//     #[pin]
//     signal: S,
//     f: F,
// }

// impl<I, S: Signal<Item = bool>, F: FnMut() -> I> Signal for MapFalseSignal<S, F> {
//     type Item = Option<I>;

//     #[inline]
//     fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
//         let MapFalseSignalProj { signal, f } = self.project();

//         signal
//             .poll_change(cx)
//             .map(|opt| opt.map(|value| not(value).then(f)))
//     }
// }
