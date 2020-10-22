use crate::{Cx, raw_text, log};
use crate::controls::Control;
use std::borrow::Cow;

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
pub struct Text<'a> {
    text: Cow<'a, str>,
}

impl<'a> Text<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> Control for Text<'a> {
    #[topo::nested]
    fn build(&mut self, cx: Cx) {
        log!("text, index: {}", cx.index);

        raw_text(cx, &self.text);
    }
}

pub trait ApplyToText<'a> {
    fn apply_to_text(self, text: &mut Text<'a>);
}

impl<'a, T: ApplyToText<'a>> ApplyToText<'a> for Option<T> {
    fn apply_to_text(self, text: &mut Text<'a>) {
        if let Some(applicable_to_text) = self {
            applicable_to_text.apply_to_text(text)
        }
    }
} 

impl<'a> ApplyToText<'a> for String {
    fn apply_to_text(self, text: &mut Text<'a>) {
        text.text = Cow::from(self);
    }
}

impl<'a> ApplyToText<'a> for &'a str {
    fn apply_to_text(self, text: &mut Text<'a>) {
        text.text = Cow::from(self);
    }
}
