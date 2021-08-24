use crate::{web_sys::HtmlElement, *};
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Row<EmptyFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<EmptyFlag>,
}

impl Row<EmptyFlagSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl Element for Row<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag> UpdateRawEl<RawHtmlEl> for Row<EmptyFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Row<EmptyFlagSet> {
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
                .style_group(StyleGroup::new(".row > .exact_width").style("flex-shrink", "0"));
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
impl<EmptyFlag> Styleable<'_, RawHtmlEl> for Row<EmptyFlag> {}
impl<EmptyFlag> KeyboardEventAware<RawHtmlEl> for Row<EmptyFlag> {}
impl<EmptyFlag> MouseEventAware<RawHtmlEl> for Row<EmptyFlag> {}
impl<EmptyFlag> MutableViewport<RawHtmlEl> for Row<EmptyFlag> {}
impl<EmptyFlag> ResizableViewport<RawHtmlEl> for Row<EmptyFlag> {}
impl<EmptyFlag> Hookable<RawHtmlEl> for Row<EmptyFlag> {
    type WSElement = HtmlElement;
}
impl<EmptyFlag> AddNearbyElement<'_> for Row<EmptyFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag> Row<EmptyFlag> {
    pub fn item(mut self, item: impl IntoOptionElement<'a> + 'a) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Row<NewEmptyFlag> {
        Row {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
