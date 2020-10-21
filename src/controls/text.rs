use crate::{Cx, raw_text, log};

#[macro_export]
macro_rules! text {
    ( $($item:expr),* $(,)?) => {
        {
            let mut text = text::Text::new();
            $(
                $item.apply_to_text(&mut text);
            )*
            text
        }
    }
}

#[derive(Default)]
pub struct Text {
    text: String,
}

impl Text {
    pub fn new() -> Self {
        Self::default()
    }

    #[topo::nested]
    fn text(cx: Cx, text: &str) {
        log!("text, index: {}", cx.index);

        raw_text(cx, text);
    }

    #[topo::nested]
    pub fn build(self, cx: Cx) {
        log!("text, index: {}", cx.index);

        raw_text(cx, &self.text);
    }
}

pub trait ApplyToText {
    fn apply_to_text(self, text: &mut Text);
}

impl  ApplyToText for String {
    fn apply_to_text(self, text: &mut Text) {
        text.text = self
    }
}

impl ApplyToText for &str {
    fn apply_to_text(self, text: &mut Text) {
        text.text = self.to_owned()
    }
}
