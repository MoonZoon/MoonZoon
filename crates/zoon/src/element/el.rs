use crate::{make_flags,  Element, IntoElement, FlagNotSet};
use dominator::{Dom, DomBuilder};
use futures_signals::signal::{Signal, SignalExt};
use std::marker::PhantomData;

// ------ ------
//   Element 
// ------ ------

make_flags!(Child);

pub struct El<ChildFlag> {
    dom_builder:DomBuilder<web_sys::HtmlElement>,
    flags: PhantomData<ChildFlag>
}

impl El<ChildFlagNotSet> {
    pub fn new() -> Self {
        Self {
            dom_builder: DomBuilder::new_html("div").class("el"),
            flags: PhantomData,
        }
    }
}

impl Element for El<ChildFlagSet> {
    fn render(self) -> Dom {
        self.dom_builder.into_dom()
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
            dom_builder: self.dom_builder.child(child.into_element().render()),
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
            dom_builder: self.dom_builder.child_signal(
                child.map(|child| Some(child.into_element().render()))
            ),
            flags: PhantomData
        }
    }
} 
