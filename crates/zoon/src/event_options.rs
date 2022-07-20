use crate::*;

#[derive(Copy, Clone, Debug, Default)]
pub struct EventOptions {
    parents_first: bool,
    preventable: bool,
}

impl EventOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parents_first(mut self) -> Self {
        self.parents_first = true;
        self
    }

    pub fn preventable(mut self) -> Self {
        self.preventable = true;
        self
    }
}

impl From<EventOptions> for dominator::EventOptions {
    fn from(options: EventOptions) -> Self {
        Self {
            bubbles: not(options.parents_first),
            preventable: options.preventable,
        }
    }
}
