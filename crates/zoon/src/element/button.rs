use wasm_bindgen::{closure::Closure, JsCast, prelude::*};
use crate::{ApplyToElement, Element, IntoElement, Node, RenderContext, __TrackedCall, __TrackedCallStack, dom::dom_element, element_macro, render};
use crate::hook::el_var;
use crate::el_var::ElVar;
use std::{cell::RefCell, rc::Rc};
use dominator::{Dom, DomBuilder, events};
use crate::println;
use futures_signals::signal::{Signal, SignalExt};
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

element_macro!(button, button::new());

pub struct Yes;
pub struct No;

pub trait IsNo {}
pub trait IsYes {}

impl IsNo for No {}
impl IsYes for Yes {}

pub struct Button<LabelSet, OnPressSet> {
    dom_builder:DomBuilder<web_sys::HtmlElement>,
    flags: PhantomData<(LabelSet, OnPressSet)>
}

impl<LabelSet, OnPressSet> Button<LabelSet, OnPressSet> {
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

pub fn new() -> Button<No, No> {
    Button::new()
}

impl<LabelSet, OnPressSet> Element for Button<LabelSet, OnPressSet> 
    where LabelSet: IsYes
{
    fn render(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, LabelSet, OnPressSet> Button<LabelSet, OnPressSet> {
    pub fn label(self, label: impl IntoElement<'a> + 'a) -> Button<Yes, OnPressSet>
        where LabelSet: IsNo
    {
        Button {
            dom_builder: self.dom_builder.child(label.into_element().render()),
            flags: PhantomData
        }
    }

    pub fn label_signal(self, label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static) -> Button<Yes, OnPressSet> 
        where LabelSet: IsNo
    {
        Button {
            dom_builder: self.dom_builder.child_signal(
                label.map(|label| Some(label.into_element().render()))
            ),
            flags: PhantomData
        }
    }

    pub fn on_press(self, on_press: impl FnOnce() + Clone + 'static) -> Button<LabelSet, Yes> 
        where OnPressSet: IsNo
    {
        Button {
            dom_builder: self.dom_builder.event(move |_: events::Click| (on_press.clone())()),
            flags: PhantomData
        }
    }
} 
