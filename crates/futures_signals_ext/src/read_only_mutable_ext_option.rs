use crate::futures_signals::signal::MutableSignalRef;
use crate::*;

pub trait ReadOnlyMutableExtOption<T>: private::ReadOnlyMutableExtOption<T> {
    fn wait_for_some_ref(
        &self,
        f: impl for<'f> FnOnce(&'f T) + 'static,
    ) -> future::LocalBoxFuture<'static, ()>;
}

impl<T: 'static> ReadOnlyMutableExtOption<T> for ReadOnlyMutable<Option<T>> {
    fn wait_for_some_ref(
        &self,
        f: impl for<'f> FnOnce(&'f T) + 'static,
    ) -> future::LocalBoxFuture<'static, ()> {
        let mut f = Some(f);
        self.signal_ref(move |value| {
            if let Some(value) = value.as_ref() {
                f.take().unwrap()(value);
                true
            } else {
                false
            }
        })
        .stop_if(|is_some| *is_some)
        .map(|_| ())
        .to_future()
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
