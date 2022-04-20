use crate::*;
use std::borrow::Cow;

// ------ TouchEventAware ------

pub trait TouchEventAware: UpdateRawEl + Sized {
    fn touch_native_handling(self, handling: TouchHandling) -> Self {
        self.update_raw_el(|raw_el| {
            let touch_action = if handling.touch_action.is_empty() {
                "auto".into()
            } else {
                handling.touch_action
            };
            raw_el.style("touch-action", &touch_action)
        })
    }
}

// ------ TouchHandling ------

#[derive(Default, Clone)]
pub struct TouchHandling {
    touch_action: Cow<'static, str>,
}

// @TODO compile-time combination verification? (like Element flags)
// @TODO https://developer.mozilla.org/en-US/docs/Web/CSS/touch-action

impl TouchHandling {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn none() -> Self {
        Self {
            touch_action: "none".into(),
        }
    }

    pub fn manipulation() -> Self {
        Self {
            touch_action: "manipulation".into(),
        }
    }

    pub fn pan_x(mut self) -> Self {
        self.touch_action.to_mut().push_str("pan-x ");
        self
    }

    pub fn pan_y(mut self) -> Self {
        self.touch_action.to_mut().push_str("pan-y ");
        self
    }

    pub fn pan_left(mut self) -> Self {
        self.touch_action.to_mut().push_str("pan-left ");
        self
    }

    pub fn pan_right(mut self) -> Self {
        self.touch_action.to_mut().push_str("pan-right ");
        self
    }

    pub fn pan_up(mut self) -> Self {
        self.touch_action.to_mut().push_str("pan-up ");
        self
    }

    pub fn pan_down(mut self) -> Self {
        self.touch_action.to_mut().push_str("pan-down ");
        self
    }

    pub fn pinch_zoom(mut self) -> Self {
        self.touch_action.to_mut().push_str("pinch-zoom ");
        self
    }
}
