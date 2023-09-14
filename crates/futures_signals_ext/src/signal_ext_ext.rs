use crate::*;
use std::{
    collections::BTreeMap,
    pin::Pin,
    task::{Context, Poll},
};

// ------ SignalExtExt ------

pub trait SignalExtExt: SignalExt {
    #[inline]
    fn for_each_sync<F>(self, callback: F) -> ForEachSync<Self, F>
    where
        F: FnMut(Self::Item),
        Self: Sized,
    {
        ForEachSync {
            inner: self.to_stream().for_each_sync(callback),
        }
    }

    #[inline]
    fn switch_signal_map<A, F>(self, callback: F) -> SwitchSignalMap<Self, A, F>
    where
        A: SignalMap,
        F: FnMut(Self::Item) -> A,
        Self: Sized,
    {
        SwitchSignalMap {
            signal: Some(self),
            signal_map: None,
            callback,
            len: 0,
        }
    }
}

impl<S: SignalExt> SignalExtExt for S {}

// -- ForEachSync --

#[pin_project]
#[derive(Debug)]
#[must_use = "Futures do nothing unless polled"]
pub struct ForEachSync<A, C> {
    #[pin]
    inner: futures_util_ext::stream_ext_ext::ForEachSync<SignalStream<A>, C>,
}

impl<A, C> Future for ForEachSync<A, C>
where
    A: Signal,
    C: FnMut(A::Item),
{
    type Output = ();

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        self.project().inner.poll(cx)
    }
}

// -- SwitchSignalMap --

#[pin_project(project = SwitchSignalMapProj)]
#[derive(Debug)]
#[must_use = "SignalMaps do nothing unless polled"]
pub struct SwitchSignalMap<A, B, C>
where
    B: SignalMap,
{
    #[pin]
    signal: Option<A>,
    #[pin]
    signal_map: Option<B>,
    callback: C,
    len: usize,
}

impl<A, B, C> SignalMap for SwitchSignalMap<A, B, C>
where
    A: Signal,
    B: SignalMap,
    C: FnMut(A::Item) -> B,
    B::Key: Ord,
{
    type Key = B::Key;
    type Value = B::Value;

    fn poll_map_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<MapDiff<Self::Key, Self::Value>>> {
        let SwitchSignalMapProj {
            mut signal,
            mut signal_map,
            callback,
            len,
        } = self.project();

        let mut signal_value = None;

        let signal_done = loop {
            break match signal
                .as_mut()
                .as_pin_mut()
                .map(|signal| signal.poll_change(cx))
            {
                None => true,
                Some(Poll::Pending) => false,
                Some(Poll::Ready(None)) => {
                    signal.set(None);
                    true
                }
                Some(Poll::Ready(Some(value))) => {
                    signal_value = Some(value);
                    continue;
                }
            };
        };

        fn new_signal_map<K, V>(len: &mut usize) -> Poll<Option<MapDiff<K, V>>> {
            if *len == 0 {
                Poll::Pending
            } else {
                *len = 0;
                Poll::Ready(Some(MapDiff::Replace { entries: vec![] }))
            }
        }

        fn calculate_len<K, V>(len: &mut usize, map_diff: &MapDiff<K, V>) {
            match map_diff {
                MapDiff::Replace { entries } => {
                    *len = entries.len();
                }
                MapDiff::Insert { .. } => {
                    *len += 1;
                }
                MapDiff::Remove { .. } => {
                    *len -= 1;
                }
                MapDiff::Clear {} => {
                    *len = 0;
                }
                MapDiff::Update { .. } => {}
            }
        }

        if let Some(value) = signal_value {
            signal_map.set(Some(callback(value)));

            match signal_map
                .as_mut()
                .as_pin_mut()
                .map(|signal| signal.poll_map_change(cx))
            {
                None => {
                    if signal_done {
                        Poll::Ready(None)
                    } else {
                        new_signal_map(len)
                    }
                }

                Some(Poll::Pending) => new_signal_map(len),

                Some(Poll::Ready(None)) => {
                    signal_map.set(None);

                    if signal_done {
                        Poll::Ready(None)
                    } else {
                        new_signal_map(len)
                    }
                }

                Some(Poll::Ready(Some(map_diff))) => {
                    if *len == 0 {
                        calculate_len(len, &map_diff);
                        Poll::Ready(Some(map_diff))
                    } else {
                        let mut map = BTreeMap::new();

                        map_diff.apply_to_map(&mut map);

                        *len = map.len();

                        // @TODO `.apply_to_entries` instead of `apply_to_map` above or a better solution>?
                        // (then we could also remove the bound `B::Key: Ord`)
                        Poll::Ready(Some(MapDiff::Replace {
                            entries: map.into_iter().collect(),
                        }))
                    }
                }
            }
        } else {
            match signal_map
                .as_mut()
                .as_pin_mut()
                .map(|signal| signal.poll_map_change(cx))
            {
                None => {
                    if signal_done {
                        Poll::Ready(None)
                    } else {
                        Poll::Pending
                    }
                }

                Some(Poll::Pending) => Poll::Pending,

                Some(Poll::Ready(None)) => {
                    signal_map.set(None);

                    if signal_done {
                        Poll::Ready(None)
                    } else {
                        Poll::Pending
                    }
                }

                Some(Poll::Ready(Some(map_diff))) => {
                    calculate_len(len, &map_diff);
                    Poll::Ready(Some(map_diff))
                }
            }
        }
    }
}
