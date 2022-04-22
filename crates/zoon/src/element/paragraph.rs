use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Paragraph<EmptyFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<EmptyFlag>,
}

impl Paragraph<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("p"))
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Paragraph<EmptyFlagNotSet, RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlagSet, RE: RawEl> IntoIterator for Paragraph<EmptyFlagSet, RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag, RE: RawEl> UpdateRawEl for Paragraph<EmptyFlag, RE> {
    type RawEl = RE;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Paragraph<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".paragraph > *").style_important("display", "inline"))
                .style_group(StyleGroup::new(".paragraph > .align_left").style("float", "left"))
                .style_group(StyleGroup::new(".paragraph > .align_right").style("float", "right"));
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str()).class("paragraph"),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag, RE: RawEl> Styleable<'_> for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> KeyboardEventAware for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MouseEventAware for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> PointerEventAware for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> TouchEventAware for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MutableViewport for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> Hookable for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> AddNearbyElement<'_> for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> HasIds for Paragraph<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> SelectableTextContent for Paragraph<EmptyFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, RE: RawEl> Paragraph<EmptyFlag, RE> {
    pub fn content(
        mut self,
        content: impl IntoOptionElement<'a> + 'a,
    ) -> Paragraph<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child(content);
        self.into_type()
    }

    pub fn content_signal(
        mut self,
        content: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Paragraph<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child_signal(content);
        self.into_type()
    }

    pub fn contents(
        mut self,
        contents: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Paragraph<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children(contents);
        self.into_type()
    }

    pub fn contents_signal_vec(
        mut self,
        contents: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Paragraph<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children_signal_vec(contents);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Paragraph<NewEmptyFlag, RE> {
        Paragraph {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
