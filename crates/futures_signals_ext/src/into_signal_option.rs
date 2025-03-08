use futures_signals::signal::{self, Signal};
use std::future::Future;

pub trait IntoSignalOption {
    type Item;

    fn into_signal_option(self) -> impl Signal<Item = Option<Self::Item>>;
}

impl<I, T: Future<Output = I>> IntoSignalOption for T {
    type Item = I;

    fn into_signal_option(self) -> impl Signal<Item = Option<Self::Item>> {
        signal::from_future(Box::pin(self))
    }
}
