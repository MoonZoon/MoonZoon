use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element 
// ------ ------

make_flags!(Empty);

pub struct Row<EmptyFlag> {
    raw_el: RawEl,
    flags: PhantomData<EmptyFlag>
}

impl Row<EmptyFlagSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawEl::with_tag("div").attr("class", "row"),
            flags: PhantomData,
        }
    }
}

impl Element for Row<EmptyFlagNotSet> {
    fn into_raw<RE: RawElement>(self) -> RE {
        self.raw_el
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, EmptyFlag> Row<EmptyFlag> {
    pub fn item(self, 
        item: impl IntoOptionElement<'a> + 'a
    ) -> Row<EmptyFlagNotSet> {
        Row {
            raw_el: self.raw_el.child(item),
            flags: PhantomData
        }
    }

    pub fn item_signal(
        self, 
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Row<EmptyFlagNotSet> {
        Row {
            raw_el: self.raw_el.child_signal(item),
            flags: PhantomData
        }
    }

    pub fn items(self, 
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>
    ) -> Row<EmptyFlagNotSet> {
        Row {
            raw_el: self.raw_el.children(items),
            flags: PhantomData
        }
    }

    pub fn items_signal_vec(
        self, 
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Row<EmptyFlagNotSet> {
        Row {
            raw_el: self.raw_el.children_signal_vec(items),
            flags: PhantomData
        }
    }
} 
