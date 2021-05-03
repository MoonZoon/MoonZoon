use crate::{make_flags,  Element, IntoElement, IntoOptionElement, RawEl};
use dominator::Dom;
use futures_signals::{signal::Signal, signal_vec::SignalVec};
use std::marker::PhantomData;

// ------ ------
//   Element 
// ------ ------

make_flags!(Empty);

pub struct Column<EmptyFlag> {
    raw_el: RawEl,
    flags: PhantomData<EmptyFlag>
}

impl Column<EmptyFlagSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawEl::with_tag("div").attr("class", "column"),
            flags: PhantomData,
        }
    }
}

impl Element for Column<EmptyFlagNotSet> {
    fn render(self) -> Dom {
        self.raw_el.render()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, EmptyFlag> Column<EmptyFlag> {
    pub fn item(self, 
        item: impl IntoOptionElement<'a> + 'a
    ) -> Column<EmptyFlagNotSet> {
        Column {
            raw_el: self.raw_el.child(item),
            flags: PhantomData
        }
    }

    pub fn item_signal(
        self, 
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Column<EmptyFlagNotSet> {
        Column {
            raw_el: self.raw_el.child_signal(item),
            flags: PhantomData
        }
    }

    pub fn items(self, 
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>
    ) -> Column<EmptyFlagNotSet> {
        Column {
            raw_el: self.raw_el.children(items),
            flags: PhantomData
        }
    }

    pub fn items_signal_vec(
        self, 
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Column<EmptyFlagNotSet> {
        Column {
            raw_el: self.raw_el.children_signal_vec(items),
            flags: PhantomData
        }
    }
} 
