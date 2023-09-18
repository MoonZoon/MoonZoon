use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Child);

pub struct El<ChildFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<ChildFlag>,
}

impl<ChildFlag, RE: RawEl> Element for El<ChildFlag, RE> {}

impl El<ChildFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<ChildFlag, RE: RawEl> RawElWrapper for El<ChildFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for El<ChildFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".el > .align_top").style("margin-bottom", "auto"))
                .style_group(StyleGroup::new(".el > .align_bottom").style("margin-top", "auto"))
                .style_group(StyleGroup::new(".el > .align_left").style("align-self", "start"))
                .style_group(StyleGroup::new(".el > .align_right").style("align-self", "end"))
                .style_group(StyleGroup::new(".el > .center_x").style("align-self", "center"))
                .style_group(
                    StyleGroup::new(".el > .center_y")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".el > .exact_height").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".el > .fill_height").style("flex-grow", "1"))
                .style_group(
                    StyleGroup::new(".el.align_left_content").style("align-items", "start"),
                )
                .style_group(StyleGroup::new(".el.align_right_content").style("align-items", "end"))
                .style_group(
                    StyleGroup::new(".el.align_top_content")
                        .style_important("justify-content", "start"),
                )
                .style_group(
                    StyleGroup::new(".el.align_bottom_content")
                        .style_important("justify-content", "end"),
                )
                .style_group(StyleGroup::new(".el.center_x_content").style("align-items", "center"))
                .style_group(
                    StyleGroup::new(".el.center_y_content")
                        .style_important("justify-content", "center"),
                );
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("el")
                .style("display", "inline-flex")
                .style("flex-direction", "column"),
            flags: PhantomData,
        }
    }
}
impl<ChildFlag, RE: RawEl> Styleable<'_> for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> KeyboardEventAware for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> MouseEventAware for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> PointerEventAware for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> TouchEventAware for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> MutableViewport for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> ResizableViewport for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> Hookable for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> AddNearbyElement<'_> for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> HasIds for El<ChildFlag, RE> {}
impl<ChildFlag, RE: RawEl> SelectableTextContent for El<ChildFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, ChildFlag, RE: RawEl> El<ChildFlag, RE> {
    pub fn child(mut self, child: impl IntoOptionElement<'a> + 'a) -> El<ChildFlagSet, RE>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(child);
        self.into_type()
    }

    pub fn child_signal(
        mut self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> El<ChildFlagSet, RE>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(child);
        self.into_type()
    }

    fn into_type<NewChildFlag>(self) -> El<NewChildFlag, RE> {
        El {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
