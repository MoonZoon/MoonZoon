use crate::{web_sys::HtmlElement, *};
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Child);

pub struct El<ChildFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<ChildFlag>,
}

impl El<ChildFlagNotSet> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<ChildFlag> Element for El<ChildFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<ChildFlag> UpdateRawEl<RawHtmlEl> for El<ChildFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for El<ChildFlagNotSet> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".el > .center_x").style("align-self", "center"))
                .style_group(StyleGroup::new(".el > .center_y")
                    .style("margin-top", "auto")
                    .style("margin-bottom", "auto")
                )
                .style_group(StyleGroup::new(".el > .align_bottom").style("margin-top", "auto"))
                .style_group(StyleGroup::new(".el > .align_left").style("align-self", "flex-start"))
                .style_group(StyleGroup::new(".el > .align_right").style("align-self", "flex-end"));
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("el")
                .style("display", "flex")
                .style("flex-direction", "column"),
            flags: PhantomData,
        }
    }
}
impl<ChildFlag> Styleable<'_, RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> KeyboardEventAware<RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> MouseEventAware<RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> MutableViewport<RawHtmlEl> for El<ChildFlag> {}
impl<ChildFlag> Hookable<RawHtmlEl> for El<ChildFlag> {
    type WSElement = HtmlElement;
}
impl<ChildFlag> AddNearbyElement<'_> for El<ChildFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, ChildFlag> El<ChildFlag> {
    pub fn child(mut self, child: impl IntoOptionElement<'a> + 'a) -> El<ChildFlagSet>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(child);
        self.into_type()
    }

    pub fn child_signal(
        mut self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> El<ChildFlagSet>
    where
        ChildFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(child);
        self.into_type()
    }

    fn into_type<NewChildFlag>(self) -> El<NewChildFlag> {
        El {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
