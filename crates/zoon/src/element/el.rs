use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element 
// ------ ------

make_flags!(Child);

pub struct El<ChildFlag> {
    raw_el: RawEl,
    flags: PhantomData<ChildFlag>
}

impl El<ChildFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawEl::with_tag("div").attr("class", "el"),
            flags: PhantomData,
        }
    }
}

impl Element for El<ChildFlagSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, ChildFlag> El<ChildFlag> {
    pub fn child(self, 
        child: impl IntoElement<'a> + 'a
    ) -> El<ChildFlagSet>
        where ChildFlag: FlagNotSet
    {
        El {
            raw_el: self.raw_el.child(child),
            flags: PhantomData
        }
    }

    pub fn child_signal(
        self, 
        child: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> El<ChildFlagSet> 
        where ChildFlag: FlagNotSet
    {
        El {
            raw_el: self.raw_el.child_signal(child),
            flags: PhantomData
        }
    }
} 
