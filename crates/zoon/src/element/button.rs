use wasm_bindgen::{closure::Closure, JsCast};
use crate::{ApplyToElement, Element, IntoElement, Node, RenderContext, __TrackedCall, __TrackedCallStack, dom::dom_element, element_macro, render};
use crate::hook::el_var;
use crate::el_var::ElVar;
use std::{cell::RefCell, rc::Rc};
use dominator::{Dom, DomBuilder, events};
use crate::println;
use futures_signals::signal::{Signal, SignalExt};

// ------ ------
//    Element 
// ------ ------

element_macro!(button, Button::default());

#[derive(Default)]
pub struct Button {
    key: u64,
    after_removes: Vec<Box<dyn FnOnce()>>,
    label: Option<Dom>,
    label_signal: Option<Box<dyn Signal<Item = Option<Dom>> + Unpin>>,
    on_press: Option<OnPress>,
}

impl Element for Button {
    #[topo::nested]
    fn render(self) -> Dom {
        let mut builder = DomBuilder::<web_sys::HtmlElement>::new_html("div")
            .class("button")
            .attribute("role", "button")
            .attribute("tabindex", "0");

        if let Some(label) = self.label {
            builder = builder
                .child(label);
        }

        if let Some(label_signal) = self.label_signal {
            builder = builder
                .child_signal(label_signal);
        }

        if let Some(on_press) = self.on_press {
            let handler = on_press.0;
            builder = builder
                .event(move |_: events::Click| handler());
        }

        for after_remove in self.after_removes {
            builder = builder.after_removed(move |_| after_remove());
        }

        builder.into_dom()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> Button {
    pub fn after_remove(mut self, after_remove: impl FnOnce() + 'static) -> Self {
        self.after_removes.push(Box::new(after_remove));
        self
    }

    pub fn after_removes(mut self, after_removes: Vec<Box<dyn FnOnce()>>) -> Self {
        self.after_removes.extend(after_removes);
        self
    }

    pub fn label(mut self, label: impl IntoElement<'a> + 'a) -> Self {
        label.into_element().apply_to_element(&mut self);
        self
    }

    pub fn label_signal(mut self, label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static) -> Self {
        self.label_signal = Some(
            Box::new(
                label.map(|label| Some(label.into_element().render()))
            )
        );
        self
    }

    pub fn on_press(mut self, on_press: impl FnOnce() + Clone + 'static) -> Self {
        self::on_press(on_press).apply_to_element(&mut self);
        self
    }
} 

// ------ IntoElement ------

impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<Button> for T {
    fn apply_to_element(self, button: &mut Button) {
        button.label = Some(self.into_element().render());
    }
} 

// ------ button::on_press(...)

pub struct OnPress(Box<dyn Fn()>);
pub fn on_press(on_press: impl FnOnce() + Clone + 'static) -> OnPress {
    OnPress(Box::new(move || on_press.clone()()))
}
impl ApplyToElement<Button> for OnPress {
    fn apply_to_element(self, button: &mut Button) {
        button.on_press = Some(self);
    }
}
