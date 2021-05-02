use crate::{Element, IntoElement};
use dominator::{Dom, DomBuilder, events};
use futures_signals::signal::{Signal, SignalExt};
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

pub trait NotSetFlag {}
pub trait SetFlag {}

pub struct LabelFlagNotSet;
pub struct LabelFlagSet;
pub struct OnPressFlagNotSet;
pub struct OnPressFlagSet;

impl NotSetFlag for LabelFlagNotSet {}
impl NotSetFlag for OnPressFlagNotSet {}
impl SetFlag for LabelFlagSet {}
impl SetFlag for OnPressFlagSet {}

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

impl<LabelFlag, OnPressFlag> Element for Button<LabelFlag, OnPressFlag> 
    where LabelFlag: SetFlag
{
    fn render(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(self, label: impl IntoElement<'a> + 'a) -> Button<LabelFlagSet, OnPressFlag>
        where LabelFlag: NotSetFlag
    {
        Button {
            dom_builder: self.dom_builder.child(label.into_element().render()),
            flags: PhantomData
        }
    }

    pub fn label_signal(self, label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static) -> Button<LabelFlagSet, OnPressFlag> 
        where LabelFlag: NotSetFlag
    {
        Button {
            dom_builder: self.dom_builder.child_signal(
                label.map(|label| Some(label.into_element().render()))
            ),
            flags: PhantomData
        }
    }

    pub fn on_press(self, on_press: impl FnOnce() + Clone + 'static) -> Button<LabelFlag, OnPressFlagSet> 
        where OnPressFlag: NotSetFlag
    {
        Button {
            dom_builder: self.dom_builder.event(move |_: events::Click| (on_press.clone())()),
            flags: PhantomData
        }
    }
} 
