use crate::futures_signals::signal::MutableLockRef;
use crate::*;

pub trait ReadOnlyMutableExt<T>: private::ReadOnlyMutableExt<T> {
    fn map<B>(&self, f: impl FnOnce(&T) -> B) -> B {
        f(&self.lock_ref())
    }

    fn map_cloned<B>(&self, f: impl FnOnce(T) -> B) -> B
    where
        T: Clone,
    {
        f(self.get_cloned())
    }

    fn use_ref(&self, f: impl FnOnce(&MutableLockRef<T>)) {
        f(&self.lock_ref())
    }

    fn unwrap_cloned(&self) -> T::Unwrapped
    where
        T: private::Unwrappable + Clone,
    {
        self.get_cloned().unwrap()
    }

    fn unwrap(&self) -> T::Unwrapped
    where
        T: private::Unwrappable + Copy,
    {
        self.get().unwrap()
    }
}

impl<T> ReadOnlyMutableExt<T> for ReadOnlyMutable<T> {}

mod private {
    use super::*;

    pub trait Unwrappable {
        type Unwrapped;
        fn unwrap(self) -> Self::Unwrapped;
    }
    impl<T> Unwrappable for Option<T> {
        type Unwrapped = T;
        fn unwrap(self) -> Self::Unwrapped {
            // @TODO `.unwrap_throw()` in a browser
            self.unwrap()
        }
    }

    pub trait ReadOnlyMutableExt<T> {
        fn lock_ref(&self) -> MutableLockRef<'_, T>;
        fn get_cloned(&self) -> T
        where
            T: Clone;
        fn get(&self) -> T
        where
            T: Copy;
    }
    impl<T> ReadOnlyMutableExt<T> for ReadOnlyMutable<T> {
        fn lock_ref(&self) -> MutableLockRef<'_, T> {
            self.lock_ref()
        }
        fn get_cloned(&self) -> T
        where
            T: Clone,
        {
            self.get_cloned()
        }
        fn get(&self) -> T
        where
            T: Copy,
        {
            self.get()
        }
    }
}
