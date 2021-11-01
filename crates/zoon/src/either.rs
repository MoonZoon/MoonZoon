use crate::*;
use std::{
    borrow::Cow,
    iter,
    pin::Pin,
    task::{Context, Poll},
};

// ------ IntoEither ------

pub trait IntoEither: Sized {
    fn left_either<R>(self) -> Either<Self, R> {
        Either::Left(self)
    }

    fn right_either<L>(self) -> Either<L, Self> {
        Either::Right(self)
    }
}

impl<T> IntoEither for T {}

// ------ Either ------

#[pin_project(project = EitherProj)]
pub enum Either<L, R> {
    Left(#[pin] L),
    Right(#[pin] R),
}

// -- Element for Either --

impl<L: Element, R: Element> Element for Either<L, R> {
    fn into_raw_element(self) -> RawElement {
        match self {
            Either::Left(element) => element.into_raw_element(),
            Either::Right(element) => element.into_raw_element(),
        }
    }
}

impl<L: Element, R: Element> IntoIterator for Either<L, R> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

// -- IntoCowStr for Either --

impl<'a, L: IntoCowStr<'a>, R: IntoCowStr<'a>> IntoCowStr<'a> for Either<L, R> {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            Either::Left(into_cow_str) => into_cow_str.into_cow_str(),
            Either::Right(into_cow_str) => into_cow_str.into_cow_str(),
        }
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        match self {
            Either::Left(into_cow_str) => into_cow_str.take_into_cow_str(),
            Either::Right(into_cow_str) => into_cow_str.take_into_cow_str(),
        }
    }
}

// -- Signal for Either

impl<I, L: Signal<Item = I>, R: Signal<Item = I>> Signal for Either<L, R> {
    type Item = I;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.project() {
            EitherProj::Left(left) => left.poll_change(cx),
            EitherProj::Right(right) => right.poll_change(cx),
        }
    }
}
