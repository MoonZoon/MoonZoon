use crate::*;
use std::borrow::Cow;

// ------ TouchEventAware ------

pub trait TouchEventAware<T: RawEl>: UpdateRawEl<T> + Sized {
    fn touch_native_handling(self, handling: TouchHandling) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.style("touch-action", &handling.touch_action.unwrap_or_else(|| "auto".into()))
        })
    }
}

// ------ TouchHandling ------

#[derive(Default, Clone)]
pub struct TouchHandling {
    touch_action: Option<Cow<'static, str>>,
}

// @TODO compile-time combination verification? (like Element flags)
// @TODO https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action

impl TouchHandling {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn none() -> Self {
        Self { touch_action: Some("none".into()) }
    }

    pub fn manipulation() -> Self {
        Self { touch_action: Some("manipulation".into()) }
    }

    pub fn pan_x(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pan-x ");
        self
    }

    pub fn pan_y(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pan-y ");
        self
    }

    pub fn pan_left(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pan-left ");
        self
    }

    pub fn pan_right(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pan-right ");
        self
    }

    pub fn pan_up(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pan-up ");
        self
    }

    pub fn pan_down(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pan-down ");
        self
    }

    pub fn pinch_zoom(mut self) -> Self {
        // @TODO `.get_or_default` once stable ?
        let touch_action = self.touch_action.get_or_insert_with(|| Cow::Owned(String::new())).to_mut();
        touch_action.push_str("pinch-zoom ");
        self
    }
}
