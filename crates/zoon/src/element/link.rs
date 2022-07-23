use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//    Element
// ------ ------

make_flags!(Label, To);

pub struct NewTab {
    refer: bool,
    follow: bool,
}

impl NewTab {
    pub fn new() -> Self {
        Self {
            refer: false,
            follow: true,
        }
    }

    pub fn refer(mut self, value: bool) -> Self {
        self.refer = value;
        self
    }

    pub fn follow(mut self, value: bool) -> Self {
        self.follow = value;
        self
    }
}

pub struct Link<LabelFlag, ToFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<(LabelFlag, ToFlag)>,
}

impl Link<LabelFlagNotSet, ToFlagNotSet, RawHtmlEl<web_sys::HtmlAnchorElement>> {
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
            raw_el: RawHtmlEl::<web_sys::HtmlAnchorElement>::new("a")
                .class("link")
                .style("text-decoration", "none")
                .style("color", "inherit")
                .style("display", "inline-flex")
                .style("flex-direction", "column")
                .style("align-items", "center"),
            flags: PhantomData,
        }
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Link<LabelFlagSet, ToFlagSet, RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, ToFlag, RE: RawEl> IntoIterator for Link<LabelFlag, ToFlag, RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<LabelFlag, ToFlag, RE: RawEl> UpdateRawEl for Link<LabelFlag, ToFlag, RE> {
    type RawEl = RE;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, ToFlag, RE: RawEl> Styleable<'_> for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> KeyboardEventAware for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> Focusable for Link<LabelFlag, ToFlag, RE> where
    RE::DomElement: AsRef<web_sys::HtmlElement>
{
}
impl<LabelFlag, ToFlag, RE: RawEl> MouseEventAware for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> PointerEventAware for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> TouchEventAware for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> Hookable for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> AddNearbyElement<'_> for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> HasIds for Link<LabelFlag, ToFlag, RE> {}
impl<LabelFlag, ToFlag, RE: RawEl> SelectableTextContent for Link<LabelFlag, ToFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, ToFlag, RE: RawEl> Link<LabelFlag, ToFlag, RE> {
    pub fn label(mut self, label: impl IntoElement<'a> + 'a) -> Link<LabelFlagSet, ToFlag, RE>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Link<LabelFlagSet, ToFlag, RE>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn to(mut self, to: impl IntoCowStr<'a>) -> Link<LabelFlag, ToFlagSet, RE>
    where
        ToFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("href", &to.into_cow_str());
        self.into_type()
    }

    pub fn to_signal(
        mut self,
        to: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Link<LabelFlag, ToFlagSet, RE>
    where
        ToFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr_signal("href", to);
        self.into_type()
    }

    pub fn new_tab(mut self, new_tab: NewTab) -> Link<LabelFlag, ToFlag, RE> {
        // @TODO remove 'noopener' once all browsers add it implicitly with '_blank'
        let mut rel = vec!["noopener"];
        if !new_tab.refer {
            rel.push("noreferrer");
        }
        if !new_tab.follow {
            rel.push("nofollow");
        }

        self.raw_el = self
            .raw_el
            .attr("target", "_blank")
            .attr("rel", rel.join(" ").as_str());
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewToFlag>(self) -> Link<NewLabelFlag, NewToFlag, RE> {
        Link {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
