use crate::{web_sys::HtmlElement, *};
use std::iter;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Column<EmptyFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<EmptyFlag>,
}

impl Column<EmptyFlagSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl Element for Column<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag> IntoIterator for Column<EmptyFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag> UpdateRawEl<RawHtmlEl> for Column<EmptyFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Column<EmptyFlagSet> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".column > .center_x").style("align-self", "center"))
                .style_group(
                    StyleGroup::new(".column > .center_y")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".column > .align_top").style("margin-bottom", "auto"))
                .style_group(StyleGroup::new(".column > .align_bottom").style("margin-top", "auto"))
                .style_group(
                    StyleGroup::new(".column > .align_left").style("align-self", "flex-start"),
                )
                .style_group(
                    StyleGroup::new(".column > .align_right").style("align-self", "flex-end"),
                )
                .style_group(StyleGroup::new(".column > .exact_height").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".column > .fill_height").style("flex-grow", "1"));
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
impl<EmptyFlag> Styleable<'_, RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> KeyboardEventAware<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> MouseEventAware<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> PointerEventAware<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> TouchEventAware<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> MutableViewport<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> ResizableViewport<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> Hookable<RawHtmlEl> for Column<EmptyFlag> {
    type WSElement = HtmlElement;
}
impl<EmptyFlag> AddNearbyElement<'_> for Column<EmptyFlag> {}
impl<EmptyFlag> HasClassId<RawHtmlEl> for Column<EmptyFlag> {}
impl<EmptyFlag> SelectableTextContent<RawHtmlEl> for Column<EmptyFlag> {}

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
