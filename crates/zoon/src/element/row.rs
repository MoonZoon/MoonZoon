use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Row<EmptyFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<EmptyFlag>,
}

impl Row<EmptyFlagSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("div").attr("class", "row"),
            flags: PhantomData,
        }
    }
}

impl Element for Row<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag> UpdateRawEl<RawHtmlEl> for Row<EmptyFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

impl<EmptyFlag> Styleable<RawHtmlEl> for Row<EmptyFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag> Row<EmptyFlag> {
    pub fn item(mut self, item: impl IntoOptionElement<'a> + 'a) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Row<NewEmptyFlag> {
        Row {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
