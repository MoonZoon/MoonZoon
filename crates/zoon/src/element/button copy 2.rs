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
    dom_builder: Option<DomBuilder<web_sys::HtmlElement>>
    // label: Option<Dom>,
    // label_signal: Option<LabelSignal>,
    // on_press: Option<OnPress>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            dom_builder: Some(DomBuilder::new_html("div")
                .class("button")
                .attribute("role", "button")
                .attribute("tabindex", "0")
            )
        }
    }

    fn update_dom_builder(&mut self, f: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>) {
        if let Some(dom_builder) = self.dom_builder.take() {
            self.dom_builder = Some(f(dom_builder));
        }
    }

    fn use_dom_builder(mut self, f: impl FnOnce(DomBuilder<web_sys::HtmlElement>) -> DomBuilder<web_sys::HtmlElement>) -> Self {
        if let Some(dom_builder) = self.dom_builder.take() {
            self.dom_builder = Some(f(dom_builder));
        }
        self
    }
}

impl Element for Button {
    fn render(self) -> Dom {
        self.dom_builder.unwrap_throw().into_dom()
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

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Button> for T {
    fn apply_to_element(self, button: &mut Button) {
        button.update_dom_builder(|builder| {
            builder.child(self.into_element().render())
        })
    }
} 

// ------ button::label_signal(...) -------

pub struct LabelSignal<'a, IE: IntoElement<'a>, S: Signal<Item = IE> + Unpin + 'static>{
    label: S,
    phantom: PhantomData<&'a ()>,
}
pub fn label_signal<'a, IE: IntoElement<'a>, S: Signal<Item = IE> + Unpin + 'static>(label: S) -> LabelSignal<'a, IE, S> {
    LabelSignal { label, phantom: PhantomData }
}
impl<'a, IE: IntoElement<'a>, S: Signal<Item = IE> + Unpin + 'static> ApplyToElement<Button> for LabelSignal<'a, IE, S> {
    fn apply_to_element(self, button: &mut Button) {
        button.update_dom_builder(|builder| {
            builder.child_signal(
                self.label.map(|label| Some(label.into_element().render()))
            )
        })
    }
}

// ------ button::on_press(...) ------

pub struct OnPress<F: FnOnce() + Clone + 'static>(F);
pub fn on_press<F: FnOnce() + Clone + 'static>(on_press: F) -> OnPress<F> {
    OnPress(on_press)
}
impl<F: FnOnce() + Clone + 'static> ApplyToElement<Button> for OnPress<F> {
    fn apply_to_element(self, button: &mut Button) {
        button.update_dom_builder(|builder| {
            builder.event(move |_: events::Click| (self.0.clone())())
        })
    }
}
