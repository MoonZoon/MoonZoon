use wasm_bindgen::{closure::Closure, JsCast};
use crate::{RenderContext, dom::dom_element, log, Node, Component, IntoComponent, ApplyToComponent, render};
use crate::hook::el_var;
use crate::state::State;
use std::{cell::RefCell, rc::Rc};

// ------ ------
//   Component 
// ------ ------

// component_macro!(button, Button);

#[macro_export]
macro_rules! button {
    ( $($attribute:expr),* $(,)?) => {
        {
            #[allow(unused_mut)]
            let mut button = $crate::component::button::Button::default();
            $( $attribute.apply_to_component(&mut button); )*
            button
        }
    }
}

#[derive(Default)]
pub struct Button<'a> {
    label: Option<Box<dyn Component + 'a>>,
    on_press: Option<OnPress>,
}

impl<'a> Component for Button<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext ) {
        log!("button, index: {}", rcx.index);

        let state_node = dom_element(rcx, |rcx: RenderContext| {
            if let Some(label) = self.label.as_mut() {
                label.render(rcx);
            }
        });
        state_node.update_mut(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "button").unwrap();
            element.set_attribute("role", "button").unwrap();
            element.set_attribute("tabindex", "0").unwrap();
        });

        if let Some(on_press) = self.on_press.take() {
            let state_listener = el_var(|| Listener::new("click", state_node));
            state_listener.update_mut(|listener| listener.set_handler(on_press.0));
        }
    }
}

struct Listener {
    event: &'static str,
    state_node: State<Node>,
    handler: Rc<RefCell<Box<dyn Fn()>>>,
    callback: Closure<dyn Fn()>,
}

impl Listener {
    fn new(event: &'static str, state_node: State<Node>) -> Self {
        let dummy_handler = Box::new(||()) as Box<dyn Fn()>;
        let handler = Rc::new(RefCell::new(dummy_handler));

        let handler_clone = Rc::clone(&handler);
        let callback = Closure::wrap(Box::new(move || {
            handler_clone.borrow()();
        }) as Box<dyn Fn()>);

        state_node.update_mut(|node| {
            node
                .node_ws
                .unchecked_ref::<web_sys::EventTarget>()
                .add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())
                .expect("add event listener");
        });

        Self {
            event,
            state_node,
            handler,
            callback,
        }
    }

    fn set_handler(&mut self, handler: Box<dyn Fn()>) {
        *self.handler.borrow_mut() = handler;
    }
}

impl Drop for Listener{
    fn drop(&mut self) {
        if !self.state_node.exists() {
            return;
        }
        self.state_node.update_mut(|node| {
            node
                .node_ws
                .unchecked_ref::<web_sys::EventTarget>()
                .remove_event_listener_with_callback(
                    self.event,
                    self.callback.as_ref().unchecked_ref(),
                )
                .expect("remove event listener");
        });
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ IntoComponent ------

impl<'a, T: IntoComponent<'a> + 'a> ApplyToComponent<Button<'a>> for T {
    fn apply_to_component(self, button: &mut Button<'a>) {
        button.label = Some(Box::new(self.into_component()));
    }
} 

// ------ button::on_press(...)

pub struct OnPress(Box<dyn Fn()>);
pub fn on_press(on_press: impl FnOnce() + Clone + 'static) -> OnPress {
    OnPress(Box::new(move || on_press.clone()()))
}
impl<'a> ApplyToComponent<Button<'a>> for OnPress {
    fn apply_to_component(self, button: &mut Button<'a>) {
        button.on_press = Some(self);
    }
}
