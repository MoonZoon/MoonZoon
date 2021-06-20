use crate::*;
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
        Self {
            raw_el: RawHtmlEl::new("div")
                .attr("class", "button")
                .attr("role", "button")
                .attr("tabindex", "0"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, OnPressFlag> UpdateRawEl<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

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
