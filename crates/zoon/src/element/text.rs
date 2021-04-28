use crate::{RenderContext, dom::dom_text, Element, __TrackedCall, __TrackedCallStack, ApplyToElement, render, element_macro};
use std::borrow::Cow;
use dominator::{Dom, text};

// ------ ------
//    Element 
// ------ ------

element_macro!(text, Text::default());

#[derive(Default)]
pub struct Text<'a> {
    text: Cow<'a, str>,
}

impl<'a> Element for Text<'a> {
    #[topo::nested]
    fn render(self) -> Dom {
        text(&self.text)
        // dom_text(rcx, &self.text);
    }
}

// ------ ------
//  Attributes 
// ------ ------

// ------ String ------

impl<'a> ApplyToElement<Text<'a>> for String {
    fn apply_to_element(self, text: &mut Text) {
        text.text = Cow::from(self);
    }
}

// ------ &str ------

impl<'a> ApplyToElement<Text<'a>> for &'a str {
    fn apply_to_element(self, text: &mut Text<'a>) {
        text.text = Cow::from(self);
    }
}
