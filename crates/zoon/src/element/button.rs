use crate::{Element, IntoElement, make_flags};
use dominator::{Dom, DomBuilder, events};
use futures_signals::signal::{Signal, SignalExt};
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

make_flags!(Label, OnPress);

pub struct Button<LabelFlag, OnPressFlag> {
    dom_builder:DomBuilder<web_sys::HtmlElement>,
    flags: PhantomData<(LabelFlag, OnPressFlag)>
}

impl Button<LabelFlagNotSet, OnPressFlagNotSet> {
    pub fn new() -> Self {
        Self {
            dom_builder: DomBuilder::new_html("div")
                .class("button")
                .attribute("role", "button")
                .attribute("tabindex", "0"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn render(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(
        self, 
        label: impl IntoElement<'a> + 'a
    ) -> Button<LabelFlagSet, OnPressFlag>
        where LabelFlag: FlagNotSet
    {
        Button {
            dom_builder: self.dom_builder.child(label.into_element().render()),
            flags: PhantomData
        }
    }

    pub fn label_signal(
        self, 
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Button<LabelFlagSet, OnPressFlag> 
        where LabelFlag: FlagNotSet
    {
        Button {
            dom_builder: self.dom_builder.child_signal(
                label.map(|label| Some(label.into_element().render()))
            ),
            flags: PhantomData
        }
    }

    pub fn on_press(
        self, 
        on_press: impl FnOnce() + Clone + 'static
    ) -> Button<LabelFlag, OnPressFlagSet> 
        where OnPressFlag: FlagNotSet
    {
        Button {
            dom_builder: self.dom_builder.event(move |_: events::Click| (on_press.clone())()),
            flags: PhantomData
        }
    }
} 
