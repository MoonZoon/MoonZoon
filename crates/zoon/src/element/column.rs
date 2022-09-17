use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Column<EmptyFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<EmptyFlag>,
}

impl Column<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Column<EmptyFlagNotSet, RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag, RE: RawEl> IntoIterator for Column<EmptyFlag, RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag, RE: RawEl> UpdateRawEl for Column<EmptyFlag, RE> {
    type RawEl = RE;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Column<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".column > .align_top").style("margin-bottom", "auto"))
                .style_group(StyleGroup::new(".column > .align_bottom").style("margin-top", "auto"))
                .style_group(StyleGroup::new(".column > .align_left").style("align-self", "start"))
                .style_group(StyleGroup::new(".column > .align_right").style("align-self", "end"))
                .style_group(StyleGroup::new(".column > .center_x").style("align-self", "center"))
                .style_group(
                    StyleGroup::new(".column > .center_y")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".column > .exact_height").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".column > .fill_height").style("flex-grow", "1"))
                .style_group(
                    StyleGroup::new(".column.align_left_content").style("align-items", "end"),
                )
                .style_group(
                    StyleGroup::new(".column.align_right_content").style("align-items", "start"),
                )
                .style_group(
                    StyleGroup::new(".column.align_top_content")
                        .style_important("justify-content", "start"),
                )
                .style_group(
                    StyleGroup::new(".column.align_bottom_content")
                        .style_important("justify-content", "end"),
                )
                .style_group(
                    StyleGroup::new(".column.center_x_content").style("align-items", "center"),
                )
                .style_group(
                    StyleGroup::new(".column.center_y_content")
                        .style_important("justify-content", "center"),
                );
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("column")
                .style("display", "inline-flex")
                .style("flex-direction", "column"),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag, RE: RawEl> Styleable<'_> for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> KeyboardEventAware for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MouseEventAware for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> PointerEventAware for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> TouchEventAware for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MutableViewport for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> ResizableViewport for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> Hookable for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> AddNearbyElement<'_> for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> HasIds for Column<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> SelectableTextContent for Column<EmptyFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, RE: RawEl> Column<EmptyFlag, RE> {
    pub fn item(mut self, item: impl IntoOptionElement<'a> + 'a) -> Column<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Column<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Column<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Column<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Column<NewEmptyFlag, RE> {
        Column {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
