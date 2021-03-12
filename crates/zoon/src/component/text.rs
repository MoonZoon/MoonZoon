use crate::{RenderContext, dom::dom_text, log, Component, ApplyToComponent, render, component_macro};
use std::borrow::Cow;

// ------ ------
//   Component 
// ------ ------

component_macro!(text, Text::default());

#[derive(Default)]
pub struct Text<'a> {
    text: Cow<'a, str>,
}

impl<'a> Component for Text<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("text, index: {}", rcx.index);

        dom_text(rcx, &self.text);
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ String ------

impl<'a> ApplyToComponent<Text<'a>> for String {
    fn apply_to_component(self, text: &mut Text) {
        text.text = Cow::from(self);
    }
}

// ------ &str ------

impl<'a> ApplyToComponent<Text<'a>> for &'a str {
    fn apply_to_component(self, text: &mut Text<'a>) {
        text.text = Cow::from(self);
    }
}
