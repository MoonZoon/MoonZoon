use std::ops::{Deref, DerefMut};

#[cfg(feature = "chrono")]
mod datetime;
#[cfg(feature = "chrono")]
mod duration;

#[derive(Debug)]
pub struct Wrapper<T> {
    pub inner: T,
}

impl<T> Wrapper<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Wrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> From<T> for Wrapper<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T: Clone> Clone for Wrapper<T> {
    fn clone(&self) -> Self {
        Wrapper::new(self.inner.clone())
    }
}

impl<T: Copy> Copy for Wrapper<T> {}

impl<T: PartialEq> PartialEq for Wrapper<T> {
    fn eq(&self, other: &Self) -> bool {
        self.inner == other.inner
    }
}

impl<T: Eq> Eq for Wrapper<T> {}
