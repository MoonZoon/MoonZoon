#[derive(Copy, Clone, Debug, Default)]
pub struct EventOptions {
    bubbles: bool,
    preventable: bool,
}

impl EventOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bubbles(mut self) -> Self {
        self.bubbles = true;
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
            bubbles: options.bubbles,
            preventable: options.preventable,
        }
    }
}
