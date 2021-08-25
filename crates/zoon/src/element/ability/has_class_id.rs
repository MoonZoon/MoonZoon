use crate::*;

pub trait HasClassId<T: RawEl>: UpdateRawEl<T> + Sized {
    fn class_id(self, consumer: impl FnOnce(ClassId)) -> Self {
        self.update_raw_el(move |raw_el| {
            consumer(raw_el.class_id());
            raw_el
        })
    }
}
