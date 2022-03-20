use crate::{web_sys::HtmlElement, *};
use std::iter;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Stack<EmptyFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<EmptyFlag>,
}

impl Stack<EmptyFlagSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl Element for Stack<EmptyFlagNotSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag> IntoIterator for Stack<EmptyFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag> UpdateRawEl<RawHtmlEl> for Stack<EmptyFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Stack<EmptyFlagSet> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".stack > *")
                        .style("grid-column", "1")
                        .style("grid-row", "1"),
                )
                .style_group(
                    StyleGroup::new(".stack > .center_x")
                        .style("margin-left", "auto")
                        .style("margin-right", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stack > .center_y")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".stack > .align_top").style("margin-bottom", "auto"))
                .style_group(StyleGroup::new(".stack > .align_bottom").style("margin-top", "auto"))
                .style_group(StyleGroup::new(".stack > .align_left").style("margin-right", "auto"))
                .style_group(StyleGroup::new(".stack > .align_right").style("margin-left", "auto"))
                .style_group(StyleGroup::new(".stack > .fill_width").style("width", "100%"))
                .style_group(StyleGroup::new(".stack > .fill_height").style("height", "100%"));
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("stack")
                .style("display", "inline-grid")
                .style("grid-auto-columns", "minmax(0, auto)")
                .style("grid-auto-rows", "minmax(0, auto)"),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag> Styleable<'_, RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> KeyboardEventAware<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> MouseEventAware<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> PointerEventAware<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> TouchEventAware<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> MutableViewport<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> ResizableViewport<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> Hookable<RawHtmlEl> for Stack<EmptyFlag> {
    type WSElement = HtmlElement;
}
impl<EmptyFlag> AddNearbyElement<'_> for Stack<EmptyFlag> {}
impl<EmptyFlag> HasClassId<RawHtmlEl> for Stack<EmptyFlag> {}
impl<EmptyFlag> SelectableTextContent<RawHtmlEl> for Stack<EmptyFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag> Stack<EmptyFlag> {
    pub fn layer(mut self, layer: impl IntoOptionElement<'a> + 'a) -> Stack<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child(layer);
        self.into_type()
    }

    pub fn layer_signal(
        mut self,
        layer: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Stack<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.child_signal(layer);
        self.into_type()
    }

    pub fn layers(
        mut self,
        layers: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Stack<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children(layers);
        self.into_type()
    }

    pub fn layers_signal_vec(
        mut self,
        layers: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Stack<EmptyFlagNotSet> {
        self.raw_el = self.raw_el.children_signal_vec(layers);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Stack<NewEmptyFlag> {
        Stack {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
