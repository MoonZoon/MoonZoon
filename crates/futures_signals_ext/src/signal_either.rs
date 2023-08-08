use crate::*;
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[pin_project(project = SignalEitherProj)]
pub enum SignalEither<L, R> {
    Left(#[pin] L),
    Right(#[pin] R),
}

impl<I, L: Signal<Item = I>, R: Signal<Item = I>> Signal for SignalEither<L, R> {
    type Item = I;

    #[inline]
    fn poll_change(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.project() {
            SignalEitherProj::Left(left) => left.poll_change(cx),
            SignalEitherProj::Right(right) => right.poll_change(cx),
        }
    }
}
impl<I, L: SignalVec<Item = I>, R: SignalVec<Item = I>> SignalVec for SignalEither<L, R> {
    type Item = I;

    #[inline]
    fn poll_vec_change(
        self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<VecDiff<Self::Item>>> {
        match self.project() {
            SignalEitherProj::Left(left) => left.poll_vec_change(cx),
            SignalEitherProj::Right(right) => right.poll_vec_change(cx),
        }
    }
}
