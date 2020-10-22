use crate::{Cx, raw_text, log};
use crate::controls::Control;

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
}

impl Control for Text {
    #[topo::nested]
    fn build(&mut self, cx: Cx) {
        log!("text, index: {}", cx.index);

        raw_text(cx, &self.text);
    }
}

pub trait ApplyToText {
    fn apply_to_text(self, text: &mut Text);
}

impl<T: ApplyToText> ApplyToText for Option<T> {
    fn apply_to_text(self, text: &mut Text) {
        if let Some(applicable_to_text) = self {
            applicable_to_text.apply_to_text(text)
        }
    }
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
