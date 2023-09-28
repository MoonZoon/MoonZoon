use crate::futures_signals::signal::MutableSignalRef;
use crate::*;

pub trait ReadOnlyMutableExtOption<T>: private::ReadOnlyMutableExtOption<T> {
    fn wait_for_some_ref<U: 'static>(
        &self,
        f: impl for<'f> FnOnce(&'f T) -> U + 'static,
    ) -> future::LocalBoxFuture<'static, U>;

    fn wait_for_some_cloned(&self) -> future::LocalBoxFuture<'static, T>
    where
        T: Clone + 'static,
    {
        self.wait_for_some_ref(Clone::clone)
    }
}

impl<T: 'static> ReadOnlyMutableExtOption<T> for ReadOnlyMutable<Option<T>> {
    fn wait_for_some_ref<U: 'static>(
        &self,
        f: impl for<'f> FnOnce(&'f T) -> U + 'static,
    ) -> future::LocalBoxFuture<'static, U> {
        let mut f = Some(f);
        self.signal_ref(move |value| value.as_ref().map(|value| f.take().unwrap()(value)))
            .stop_if(|value| value.is_some())
            .to_future()
            .map(|value| value.unwrap())
            .boxed_local()
    }
}

mod private {
    use super::*;

    pub trait ReadOnlyMutableExtOption<T> {
        fn signal_ref<U, F>(&self, f: F) -> MutableSignalRef<Option<T>, F>
        where
            F: FnMut(&Option<T>) -> U;
    }
    impl<T> ReadOnlyMutableExtOption<T> for ReadOnlyMutable<Option<T>> {
        fn signal_ref<U, F>(&self, f: F) -> MutableSignalRef<Option<T>, F>
        where
            F: FnMut(&Option<T>) -> U,
        {
            self.signal_ref(f)
        }
    }
}
