use crate::{RenderContext, raw_text, log, Component, ApplyToComponent, render};
use std::borrow::Cow;

// ------ ------
//   Component 
// ------ ------

// component_macro!(text, Text);

#[macro_export]
macro_rules! text {
    ( $($attribute:expr),* $(,)?) => {
        {
            let mut text = $crate::component::text::Text::default();
            $( text = text.with($attribute); )*
            text
        }
    }
}

#[derive(Default)]
pub struct Text<'a> {
    text: Cow<'a, str>,
}

impl<'a> Component for Text<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("text, index: {}", rcx.index);

        raw_text(rcx, &self.text);
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ String ------

impl<'a> ApplyToComponent<Text<'a>> for String {
    fn apply_to_component(self, component: &mut Text) {
        component.text = Cow::from(self);
    }
}

// ------ &str ------

impl<'a> ApplyToComponent<Text<'a>> for &'a str {
    fn apply_to_component(self, component: &mut Text<'a>) {
        component.text = Cow::from(self);
    }
}
