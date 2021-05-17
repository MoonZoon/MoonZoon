use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Column<EmptyFlag> {
    raw_el: RawEl,
    flags: PhantomData<EmptyFlag>,
}

impl Column<EmptyFlagSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawEl::new("div").attr("class", "column"),
            flags: PhantomData,
        }
    }
}

impl Element for Column<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag> Column<EmptyFlag> {
    pub fn item(mut self, item: impl IntoOptionElement<'a> + 'a) -> Column<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Column<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Column<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Column<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Column<NewEmptyFlag> {
        Column {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
