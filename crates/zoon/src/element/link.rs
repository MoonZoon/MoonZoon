use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//    Element
// ------ ------

make_flags!(Label, To);

type LinkRawEl = RawHtmlEl<web_sys::HtmlAnchorElement>;

pub struct Link<LabelFlag, ToFlag> {
    raw_el: LinkRawEl,
    flags: PhantomData<(LabelFlag, ToFlag)>,
}

impl Link<LabelFlagNotSet, ToFlagNotSet> {
    pub fn new() -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".link > *")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".link > .align_top").style("margin-bottom", "auto"))
                .style_group(StyleGroup::new(".link > .align_bottom").style("margin-top", "auto"))
                .style_group(
                    StyleGroup::new(".link > .align_left").style("align-self", "flex-start"),
                )
                .style_group(
                    StyleGroup::new(".link > .align_right").style("align-self", "flex-end"),
                )
                .style_group(StyleGroup::new(".link > .exact_height").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".link > .fill_height").style("flex-grow", "1"));
        });
        Self {
            raw_el: RawHtmlEl::new("a")
                .class("link")
                .style("text-decoration", "none")
                .style("color", "inherit")
                .style("display", "inline-flex")
                .style("flex-direction", "column")
                .style("align-items", "center")
                .dom_element_type(),
            flags: PhantomData,
        }
    }
}

impl Element for Link<LabelFlagSet, ToFlagSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, ToFlag> IntoIterator for Link<LabelFlag, ToFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<LabelFlag, ToFlag> UpdateRawEl<LinkRawEl> for Link<LabelFlag, ToFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(LinkRawEl) -> LinkRawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, ToFlag> Styleable<'_, LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> KeyboardEventAware<LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> Focusable<LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> MouseEventAware<LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> PointerEventAware<LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> TouchEventAware<LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> Hookable<LinkRawEl> for Link<LabelFlag, ToFlag> {
}
impl<LabelFlag, ToFlag> AddNearbyElement<'_, LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> HasClassId<LinkRawEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> SelectableTextContent<LinkRawEl> for Link<LabelFlag, ToFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, ToFlag> Link<LabelFlag, ToFlag> {
    pub fn label(mut self, label: impl IntoElement<'a> + 'a) -> Link<LabelFlagSet, ToFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Link<LabelFlagSet, ToFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn to(mut self, to: impl IntoCowStr<'a>) -> Link<LabelFlag, ToFlagSet>
    where
        ToFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("href", &to.into_cow_str());
        self.into_type()
    }

    pub fn to_signal(
        mut self,
        to: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Link<LabelFlag, ToFlagSet>
    where
        ToFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr_signal("href", to);
        self.into_type()
    }

    pub fn new_tab(mut self) -> Link<LabelFlag, ToFlag> {
        self.raw_el = self.raw_el.attr("target", "_blank");
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewToFlag>(self) -> Link<NewLabelFlag, NewToFlag> {
        Link {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
