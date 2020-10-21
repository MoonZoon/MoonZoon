use wasm_bindgen::JsCast;
use crate::{Cx, raw_el, log};

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
    label: Option<Label<'a>>,
    on_press: Option<OnPress<'a>>,
}

impl<'a> Button<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    #[topo::nested]
    pub fn build(self, cx: Cx) {
        log!("button, index: {}", cx.index);

        let state_node = raw_el(cx, |cx| {
            if let Some(label) = self.label {
                (label.0)(cx)
            }
        });
        state_node.update(|node| {
            let element = node.node_ws.unchecked_ref::<web_sys::Element>();
            element.set_attribute("class", "button");
            element.set_attribute("role", "button");
            element.set_attribute("tabindex", "0");
        });
    }
}

pub trait ApplyToButton<'a> {
    fn apply_to_button(self, button: &mut Button<'a>);
}

pub struct Label<'a>(Box<dyn FnOnce(Cx) + 'a>);
pub fn label<'a>(label: impl FnOnce(Cx) + 'a) -> Label<'a> {
    Label(Box::new(label))
}
impl<'a> ApplyToButton<'a> for Label<'a> {
    fn apply_to_button(self, button: &mut Button<'a>) {
        button.label = Some(self);
    }
}

pub struct OnPress<'a>(Box<dyn FnOnce() + 'a>);
pub fn on_press<'a>(on_press: impl FnOnce() + 'a) -> OnPress<'a> {
    OnPress(Box::new(on_press))
}
impl<'a> ApplyToButton<'a> for OnPress<'a> {
    fn apply_to_button(self, button: &mut Button<'a>) {
        button.on_press = Some(self);
    }
}
