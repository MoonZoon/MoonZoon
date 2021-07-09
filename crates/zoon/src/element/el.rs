use crate::{web_sys::HtmlDivElement, *};
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Child);

pub struct El<ChildFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<ChildFlag>,
}

impl El<ChildFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("div").attr("class", "el"),
            flags: PhantomData,
        }
    }
}

impl<ChildFlag> Element for El<ChildFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<ChildFlag> UpdateRawEl<RawHtmlEl> for El<ChildFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<ChildFlag> Styleable<'_, RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> KeyboardEventAware<RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> Hoverable<RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> Hookable<RawHtmlEl> for El<ChildFlag> {
    type WSElement = HtmlDivElement;
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, ChildFlag> El<ChildFlag> {
    pub fn child(mut self, child: impl IntoElement<'a> + 'a) -> El<ChildFlagSet>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(child);
        self.into_type()
    }

    pub fn child_signal(
        mut self,
        child: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> El<ChildFlagSet>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(child);
        self.into_type()
    }

    fn into_type<NewChildFlag>(self) -> El<NewChildFlag> {
        El {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
