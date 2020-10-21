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
pub struct Button {
    label: Option<Label>,
    on_press: Option<OnPress>,
}

impl Button {
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

pub trait ApplyToButton {
    fn apply_to_button(self, button: &mut Button);
}

pub struct Label(Box<dyn FnOnce(Cx)>);
pub fn label(label: impl FnOnce(Cx) + 'static) -> Label {
    Label(Box::new(label))
}
impl ApplyToButton for Label {
    fn apply_to_button(self, button: &mut Button) {
        button.label = Some(self);
    }
}

pub struct OnPress(Box<dyn FnOnce()>);
pub fn on_press(on_press: impl FnOnce() + 'static) -> OnPress {
    OnPress(Box::new(on_press))
}
impl ApplyToButton for OnPress {
    fn apply_to_button(self, button: &mut Button) {
        button.on_press = Some(self);
    }
}
