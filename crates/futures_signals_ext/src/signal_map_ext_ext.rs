use crate::*;
use std::{
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

// ------ SignalMapExtExt ------

pub trait SignalMapExtExt: SignalMapExt {
    #[inline]
    fn for_each_sync<F>(self, mut callback: F) -> ForEachSync<Self, F>
    where
        F: FnMut(MapDiff<Self::Key, Self::Value>) + 'static,
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

    #[inline]
    fn map_key<A, F>(self, callback: F) -> MapKey<Self, F>
    where
        F: FnMut(Self::Key) -> A,
        Self: Sized,
    {
        MapKey {
            signal: self,
            callback,
        }
    }
}

impl<S: SignalMapExt> SignalMapExtExt for S {}

// -- ForEachSync --

#[pin_project(project = ForEachSyncProj)]
#[must_use = "Futures do nothing unless polled"]
pub struct ForEachSync<S, F> {
    #[pin]
    future: future::LocalBoxFuture<'static, ()>,
    signal: PhantomData<S>,
    callback: PhantomData<F>,
}

impl<S: SignalMap, F: FnMut(MapDiff<S::Key, S::Value>)> Future for ForEachSync<S, F> {
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().future.poll(cx)
    }
}

// -- MapKey --

#[pin_project(project = MapKeyProj)]
#[derive(Debug)]
#[must_use = "SignalMaps do nothing unless polled"]
pub struct MapKey<A, B> {
    #[pin]
    signal: A,
    callback: B,
}

impl<A, B, F> SignalMap for MapKey<A, F>
where
    A: SignalMap,
    F: FnMut(A::Key) -> B,
{
    type Key = B;
    type Value = A::Value;

    // TODO should this inline ?
    #[inline]
    fn poll_map_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<MapDiff<Self::Key, Self::Value>>> {
        let MapKeyProj { signal, callback } = self.project();

        signal.poll_map_change(cx).map(|some| {
            some.map(|change| {
                match change {
                    // TODO figure out a more efficient way of implementing this
                    MapDiff::Replace { entries } => MapDiff::Replace {
                        entries: entries.into_iter().map(|(k, v)| (callback(k), v)).collect(),
                    },
                    MapDiff::Insert { key, value } => MapDiff::Insert {
                        key: callback(key),
                        value,
                    },
                    MapDiff::Update { key, value } => MapDiff::Update {
                        key: callback(key),
                        value,
                    },
                    MapDiff::Remove { key } => MapDiff::Remove { key: callback(key) },
                    MapDiff::Clear {} => MapDiff::Clear {},
                }
            })
        })
    }
}
