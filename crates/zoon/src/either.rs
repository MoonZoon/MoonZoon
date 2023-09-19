use crate::*;
use std::{
    borrow::Cow,
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

// -- ElementUnchecked for Either --

impl<L: ElementUnchecked, R: ElementUnchecked> ElementUnchecked for Either<L, R> {
    fn into_raw_unchecked(self) -> RawElOrText {
        match self {
            Either::Left(element) => element.into_raw_unchecked(),
            Either::Right(element) => element.into_raw_unchecked(),
        }
    }
}

// -- Element for Either --

impl<L: Element, R: Element> Element for Either<L, R> {}

// -- IntoCowStr for Either --

impl<'a, L: IntoCowStr<'a>, R: IntoCowStr<'a>> IntoCowStr<'a> for Either<L, R> {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            Either::Left(into_cow_str) => into_cow_str.into_cow_str(),
            Either::Right(into_cow_str) => into_cow_str.into_cow_str(),
        }
    }
}

// -- Signal for Either

impl<L: Signal, R: Signal<Item = L::Item>> Signal for Either<L, R> {
    type Item = L::Item;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.project() {
            EitherProj::Left(left) => left.poll_change(cx),
            EitherProj::Right(right) => right.poll_change(cx),
        }
    }
}

// -- Iterator for Either

impl<L: Iterator, R: Iterator<Item = L::Item>> Iterator for Either<L, R> {
    type Item = L::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::Left(iterator) => iterator.next(),
            Either::Right(iterator) => iterator.next(),
        }
    }

    // @TODO optimize by implementing other methods?
    // see https://docs.rs/either/1.9.0/either/enum.Either.html#impl-Iterator-for-Either%3CL,+R%3E
    // Or can we use `either` crate instead or under the hood?
}
