use std::ops::{Deref, DerefMut};

#[cfg(feature = "chrono")]
mod duration;
#[cfg(feature = "chrono")]
mod datetime;

pub struct Wrapper<T> {
    pub inner: T
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
