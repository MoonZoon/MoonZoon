use std::mem;
use crate::*;

pub trait RawElWrapper: Sized {
    type RawEl: RawEl;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl;

    #[track_caller]
    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        let raw_el = mem::replace(self.raw_el_mut(), RawEl::new_dummy());
        mem::swap(self.raw_el_mut(), &mut updater(raw_el));
        self
    }

    #[track_caller]
    fn into_raw_el(mut self) -> Self::RawEl {
        mem::replace(self.raw_el_mut(), RawEl::new_dummy())
    }
}
