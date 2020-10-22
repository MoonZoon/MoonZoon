use wasm_bindgen::{closure::Closure, JsCast};
use crate::{Cx, raw_el, log, Node};
use crate::controls::Control;
use crate::hooks::use_state;
use crate::state::State;
use std::{cell::RefCell, rc::Rc};

#[macro_export]
macro_rules! button {
    ( $($item:expr),* $(,)?) => {
        {
            let mut button = button::Button::new();
            $(
                $item.apply_to_button(&mut button);
            )*
            button
        }
    }
}

#[derive(Default)]
pub struct Button<'a> {
    label: Option<Box<dyn Control + 'a>>,
    on_press: Option<OnPress>,
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Control for Button<'a> {
    #[topo::nested]
    fn build(&mut self, cx: Cx) {
        log!("button, index: {}", cx.index);

        let state_node = raw_el(cx, |cx: Cx| {
            if let Some(label) = self.label.as_mut() {
                label.build(cx);
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "button");
            element.set_attribute("role", "button");
            element.set_attribute("tabindex", "0");
        });

        if let Some(on_press) = self.on_press.take() {
            let state_listener = use_state(|| Listener::new("click", state_node));
            state_listener.update(|listener| listener.set_handler(on_press.0));
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

        state_node.update(|node| {
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
        self.state_node.update(|node| {
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

pub trait ApplyToButton<'a> {
    fn apply_to_button(self, button: &mut Button<'a>);
}

impl<'a, T: ApplyToButton<'a>> ApplyToButton<'a> for Option<T> {
    fn apply_to_button(self, button: &mut Button<'a>) {
        if let Some(applicable_to_button) = self {
            applicable_to_button.apply_to_button(button)
        }
    }
} 

impl<'a, T: Control + 'a> ApplyToButton<'a> for T {
    fn apply_to_button(self, button: &mut Button<'a>) {
         button.label = Some(Box::new(self));
    }
} 

pub struct OnPress(Box<dyn Fn()>);
pub fn on_press(on_press: impl FnOnce() + Clone + 'static) -> OnPress {
    OnPress(Box::new(move || on_press.clone()()))
}
impl<'a> ApplyToButton<'a> for OnPress {
    fn apply_to_button(self, button: &mut Button<'a>) {
        button.on_press = Some(self);
    }
}
