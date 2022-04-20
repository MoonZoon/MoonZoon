use crate::*;

pub trait HasClassId: UpdateRawEl + Sized {
    fn class_id(self, consumer: impl FnOnce(ClassId)) -> Self {
        self.update_raw_el(move |raw_el| {
            consumer(raw_el.class_id());
            raw_el
        })
    }
}
