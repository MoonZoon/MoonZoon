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

element_macro!(button, Button::new());

pub struct Button {
    dom_builder:DomBuilder<web_sys::HtmlElement>
}

impl Button {
    pub fn new() -> Self {
        Self {
            dom_builder: DomBuilder::new_html("div")
                .class("button")
                .attribute("role", "button")
                .attribute("tabindex", "0")
        }
    }

    fn use_dom_builder(mut self, f: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>) -> Self {
        self.dom_builder = f(self.dom_builder);
        self
    }
}

impl Element for Button {
    fn render(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> Button {
    pub fn label(self, label: impl IntoElement<'a> + 'a) -> Self {
        self.use_dom_builder(|builder| {
            builder.child(label.into_element().render())
        })
    }

    pub fn label_signal(self, label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static) -> Self {
        self.use_dom_builder(|builder| {
            builder.child_signal(
                label.map(|label| Some(label.into_element().render()))
            )
        })
    }

    pub fn on_press(self, on_press: impl FnOnce() + Clone + 'static) -> Self {
        self.use_dom_builder(|builder| {
            builder.event(move |_: events::Click| (on_press.clone())())
        })
    }
} 
