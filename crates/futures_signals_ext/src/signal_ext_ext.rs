use crate::*;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

// ------ SignalExtExt ------

pub trait SignalExtExt: SignalExt {
    #[inline]
    fn for_each_sync<F>(self, mut callback: F) -> ForEachSync<Self, F>
    where
        F: FnMut(Self::Item) + 'static,
        Self: 'static + Sized,
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

impl<S: SignalExt> SignalExtExt for S {}

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
