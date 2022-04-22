use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

make_flags!(Empty, Multiline);

pub struct Row<EmptyFlag, MultilineFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<(EmptyFlag, MultilineFlag)>,
}

impl Row<EmptyFlagSet, MultilineFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<MultilineFlag, RE: RawEl + Into<RawElement>> Element
    for Row<EmptyFlagNotSet, MultilineFlag, RE>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag, MultilineFlag, RE: RawEl> IntoIterator for Row<EmptyFlag, MultilineFlag, RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag, MultilineFlag, RE: RawEl> UpdateRawEl for Row<EmptyFlag, MultilineFlag, RE> {
    type RawEl = RE;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Row<EmptyFlagSet, MultilineFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".row > .center_x")
                        .style("margin-left", "auto")
                        .style("margin-right", "auto"),
                )
                .style_group(StyleGroup::new(".row > .align_top").style("align-self", "flex-start"))
                .style_group(
                    StyleGroup::new(".row > .align_bottom").style("align-self", "flex-end"),
                )
                .style_group(StyleGroup::new(".row > .align_right").style("margin-left", "auto"))
                .style_group(StyleGroup::new(".row > .exact_width").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".row > .fill_width").style("flex-grow", "1"));
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("row")
                .style("display", "inline-flex")
                .style("align-items", "center"),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> Styleable<'_> for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> KeyboardEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> MouseEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> PointerEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> TouchEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> MutableViewport for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> ResizableViewport for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> Hookable for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> AddNearbyElement<'_>
    for Row<EmptyFlag, MultilineFlag, RE>
{
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> HasIds for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> SelectableTextContent
    for Row<EmptyFlag, MultilineFlag, RE>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, MultilineFlag, RE: RawEl> Row<EmptyFlag, MultilineFlag, RE> {
    pub fn item(
        mut self,
        item: impl IntoOptionElement<'a> + 'a,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    pub fn multiline(mut self) -> Row<EmptyFlag, MultilineFlagSet, RE>
    where
        MultilineFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.style("flex-wrap", "wrap");
        self.raw_el = self.raw_el.style("flex-basis", "0");
        self.raw_el = self.raw_el.style("flex-grow", "1");
        self.into_type()
    }

    fn into_type<NewEmptyFlag, NewMultilineFlag>(self) -> Row<NewEmptyFlag, NewMultilineFlag, RE> {
        Row {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
