use crate::*;
use futures_util::{future::FusedFuture, ready, stream::FusedStream};
use pin_project::pin_project;
use std::fmt;
use std::pin::Pin;
use std::task::{Context, Poll};

pub trait StreamExtExt: StreamExt {
    fn for_each_sync<F>(self, f: F) -> ForEachSync<Self, F>
    where
        F: FnMut(Self::Item),
        Self: Sized,
    {
        assert_future::<(), _>(ForEachSync::new(self, f))
    }
}
impl<S: StreamExt> StreamExtExt for S {}

#[pin_project(project = ForEachSyncProj)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct ForEachSync<St, F> {
    #[pin]
    stream: St,
    f: F,
}

impl<St, F> fmt::Debug for ForEachSync<St, F>
where
    St: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ForEachSync")
            .field("stream", &self.stream)
            .finish()
    }
}

impl<St, F> ForEachSync<St, F>
where
    St: Stream,
    F: FnMut(St::Item),
{
    pub(super) fn new(stream: St, f: F) -> Self {
        Self { stream, f }
    }
}

impl<St, F> FusedFuture for ForEachSync<St, F>
where
    St: FusedStream,
    F: FnMut(St::Item),
{
    fn is_terminated(&self) -> bool {
        self.stream.is_terminated()
    }
}

impl<St, F> Future for ForEachSync<St, F>
where
    St: Stream,
    F: FnMut(St::Item),
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        let mut this = self.project();
        while let Some(item) = ready!(this.stream.as_mut().poll_next(cx)) {
            (this.f)(item);
        }
        Poll::Ready(())
    }
}

// --

// Just a helper function to ensure the futures we're returning all have the
// right implementations.
pub(crate) fn assert_future<T, F>(future: F) -> F
where
    F: Future<Output = T>,
{
    future
}
