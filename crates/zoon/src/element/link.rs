use crate::{web_sys::HtmlAnchorElement, *};
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Label, To);

pub struct Link<LabelFlag, ToFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(LabelFlag, ToFlag)>,
}

impl Link<LabelFlagNotSet, ToFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("a")
                .attr("class", "link")
                .style("text-decoration", "none")
                .style("color", "inherit"),
            flags: PhantomData,
        }
    }
}

impl Element for Link<LabelFlagSet, ToFlagSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, ToFlag> UpdateRawEl<RawHtmlEl> for Link<LabelFlag, ToFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, ToFlag> Styleable<'_, RawHtmlEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> KeyboardEventAware<RawHtmlEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> Focusable for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> MouseEventAware<RawHtmlEl> for Link<LabelFlag, ToFlag> {}
impl<LabelFlag, ToFlag> Hookable<RawHtmlEl> for Link<LabelFlag, ToFlag> {
    type WSElement = HtmlAnchorElement;
}

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
