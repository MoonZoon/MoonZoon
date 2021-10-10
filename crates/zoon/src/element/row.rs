use crate::{web_sys::HtmlElement, *};
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty, Multiline);

pub struct Row<EmptyFlag, MultilineFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(EmptyFlag, MultilineFlag)>,
}

impl Row<EmptyFlagSet, MultilineFlagNotSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<MultilineFlag> Element for Row<EmptyFlagNotSet, MultilineFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag, MultilineFlag> UpdateRawEl<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Row<EmptyFlagSet, MultilineFlagNotSet> {
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
impl<EmptyFlag, MultilineFlag> Styleable<'_, RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {}
impl<EmptyFlag, MultilineFlag> KeyboardEventAware<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {}
impl<EmptyFlag, MultilineFlag> MouseEventAware<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {}
impl<EmptyFlag, MultilineFlag> MutableViewport<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {}
impl<EmptyFlag, MultilineFlag> ResizableViewport<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {}
impl<EmptyFlag, MultilineFlag> Hookable<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {
    type WSElement = HtmlElement;
}
impl<EmptyFlag, MultilineFlag> AddNearbyElement<'_> for Row<EmptyFlag, MultilineFlag> {}
impl<EmptyFlag, MultilineFlag> HasClassId<RawHtmlEl> for Row<EmptyFlag, MultilineFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, MultilineFlag> Row<EmptyFlag, MultilineFlag> {
    pub fn item(mut self, item: impl IntoOptionElement<'a> + 'a) -> Row<EmptyFlagNotSet, MultilineFlag> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet, MultilineFlag> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Row<EmptyFlagNotSet, MultilineFlag> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet, MultilineFlag> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    pub fn multiline(mut self) -> Row<EmptyFlag, MultilineFlagSet> where MultilineFlag: FlagNotSet {
        self.raw_el = self.raw_el.style("flex-wrap", "wrap");
        self.raw_el = self.raw_el.style("flex-basis", "0");
        self.raw_el = self.raw_el.style("flex-grow", "1");
        self.into_type()
    }

    fn into_type<NewEmptyFlag, NewMultilineFlag>(self) -> Row<NewEmptyFlag, NewMultilineFlag> {
        Row {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
