use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

type ParagraphRawEl = RawHtmlEl<web_sys::HtmlParagraphElement>;

pub struct Paragraph<EmptyFlag> {
    raw_el: ParagraphRawEl,
    flags: PhantomData<EmptyFlag>,
}

impl Paragraph<EmptyFlagSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("p"))
    }
}

impl Element for Paragraph<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlagSet> IntoIterator for Paragraph<EmptyFlagSet> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag> UpdateRawEl<ParagraphRawEl> for Paragraph<EmptyFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(ParagraphRawEl) -> ParagraphRawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Paragraph<EmptyFlagSet> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".paragraph > *").style_important("display", "inline"))
                .style_group(StyleGroup::new(".paragraph > .align_left").style("float", "left"))
                .style_group(StyleGroup::new(".paragraph > .align_right").style("float", "right"));
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str()).class("paragraph").dom_element_type(),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag> Styleable<'_, ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> KeyboardEventAware<ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> MouseEventAware<ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> PointerEventAware<ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> TouchEventAware<ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> MutableViewport<ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> Hookable<ParagraphRawEl> for Paragraph<EmptyFlag> {
}
impl<EmptyFlag> AddNearbyElement<'_, ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> HasClassId<ParagraphRawEl> for Paragraph<EmptyFlag> {}
impl<EmptyFlag> SelectableTextContent<ParagraphRawEl> for Paragraph<EmptyFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag> Paragraph<EmptyFlag> {
    pub fn content(
        mut self,
        content: impl IntoOptionElement<'a> + 'a,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child(content);
        self.into_type()
    }

    pub fn content_signal(
        mut self,
        content: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child_signal(content);
        self.into_type()
    }

    pub fn contents(
        mut self,
        contents: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children(contents);
        self.into_type()
    }

    pub fn contents_signal_vec(
        mut self,
        contents: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Paragraph<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children_signal_vec(contents);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Paragraph<NewEmptyFlag> {
        Paragraph {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
