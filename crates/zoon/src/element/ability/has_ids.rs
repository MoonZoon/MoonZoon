use crate::*;

pub trait HasIds: UpdateRawEl + Sized {
    fn id<'a>(self, id: impl IntoCowStr<'a>) -> Self {
        self.update_raw_el(move |raw_el| {
            raw_el.id(id)
        })
    }

    fn class_id(self, consumer: impl FnOnce(ClassId)) -> Self {
        self.update_raw_el(move |raw_el| {
            consumer(raw_el.class_id());
            raw_el
        })
    }
}
