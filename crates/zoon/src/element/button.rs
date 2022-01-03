use crate::{web_sys::HtmlDivElement, *};
use std::iter;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Label, OnPress);

pub struct Button<LabelFlag, OnPressFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(LabelFlag, OnPressFlag)>,
}

impl Button<LabelFlagNotSet, OnPressFlagNotSet> {
    pub fn new() -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".button > *")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(StyleGroup::new(".button > .align_bottom").style("margin-top", "auto"))
                .style_group(
                    StyleGroup::new(".button > .align_left").style("align-self", "flex-start"),
                )
                .style_group(
                    StyleGroup::new(".button > .align_right").style("align-self", "flex-end"),
                )
                .style_group(StyleGroup::new(".button > .exact_height").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".button > .fill_height").style("flex-grow", "1"));
        });
        Self {
            raw_el: RawHtmlEl::new("div")
                .class("button")
                .attr("role", "button")
                .attr("tabindex", "0")
                .style("cursor", "pointer")
                .style("user-select", "none")
                .style("text-align", "center")
                .style("display", "flex")
                .style("flex-direction", "column")
                .style("align-items", "center")
                .style("touch-action", "manipulation"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, OnPressFlag> IntoIterator for Button<LabelFlag, OnPressFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<LabelFlag, OnPressFlag> UpdateRawEl<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, OnPressFlag> Styleable<'_, RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> KeyboardEventAware<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> Focusable for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> MouseEventAware<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> PointerEventAware<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> TouchEventAware<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> Hookable<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {
    type WSElement = HtmlDivElement;
}
impl<LabelFlag, OnPressFlag> AddNearbyElement<'_> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> HasClassId<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(mut self, label: impl IntoElement<'a> + 'a) -> Button<LabelFlagSet, OnPressFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Button<LabelFlagSet, OnPressFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn on_press(
        mut self,
        on_press: impl FnOnce() + Clone + 'static,
    ) -> Button<LabelFlag, OnPressFlagSet>
    where
        OnPressFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .event_handler(move |_: events::Click| (on_press.clone())());
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewOnPressFlag>(self) -> Button<NewLabelFlag, NewOnPressFlag> {
        Button {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
