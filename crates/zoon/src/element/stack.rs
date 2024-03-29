use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Stack<EmptyFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<EmptyFlag>,
}

impl<RE: RawEl> Element for Stack<EmptyFlagNotSet, RE> {}

impl Stack<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<EmptyFlag, RE: RawEl> RawElWrapper for Stack<EmptyFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Stack<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".stack > *")
                        .style("grid-column", "1")
                        .style("grid-row", "1"),
                )
                .style_group(StyleGroup::new(".stack > .align_top").style("align-self", "start"))
                .style_group(StyleGroup::new(".stack > .align_bottom").style("align-self", "end"))
                .style_group(StyleGroup::new(".stack > .align_left").style("justify-self", "left"))
                .style_group(
                    StyleGroup::new(".stack > .align_right").style("justify-self", "right"),
                )
                .style_group(StyleGroup::new(".stack > .center_x").style("justify-self", "center"))
                .style_group(StyleGroup::new(".stack > .center_y").style("align-self", "center"))
                .style_group(StyleGroup::new(".stack > .fill_width").style("width", "100%"))
                .style_group(StyleGroup::new(".stack > .fill_height").style("height", "100%"))
                .style_group(
                    StyleGroup::new(".stack.align_left_content").style("justify-content", "left"),
                )
                .style_group(
                    StyleGroup::new(".stack.align_right_content").style("justify-content", "right"),
                )
                .style_group(
                    StyleGroup::new(".stack.align_top_content")
                        .style_important("align-items", "start"),
                )
                .style_group(
                    StyleGroup::new(".stack.align_bottom_content")
                        .style_important("align-items", "end"),
                )
                .style_group(
                    StyleGroup::new(".stack.center_x_content").style("justify-content", "center"),
                )
                .style_group(
                    StyleGroup::new(".stack.center_y_content")
                        .style_important("align-items", "center"),
                );
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
impl<EmptyFlag, RE: RawEl> Styleable<'_> for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> KeyboardEventAware for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MouseEventAware for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> PointerEventAware for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> TouchEventAware for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MutableViewport for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> ResizableViewport for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> AddNearbyElement<'_> for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> HasIds for Stack<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> SelectableTextContent for Stack<EmptyFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, RE: RawEl> Stack<EmptyFlag, RE> {
    pub fn layer(mut self, layer: impl IntoOptionElement<'a> + 'a) -> Stack<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child(layer);
        self.into_type()
    }

    pub fn layer_signal(
        mut self,
        layer: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Stack<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child_signal(layer);
        self.into_type()
    }

    pub fn layers(
        mut self,
        layers: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Stack<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children(layers);
        self.into_type()
    }

    pub fn layers_signal_vec(
        mut self,
        layers: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Stack<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children_signal_vec(layers);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Stack<NewEmptyFlag, RE> {
        Stack {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
