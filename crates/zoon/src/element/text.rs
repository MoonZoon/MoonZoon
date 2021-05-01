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
    fn render(self) -> Dom {
        text(&self.text)

//         let text = window()
//     .unwrap()
//     .document()
//     .unwrap()
//     .create_text_node("foo");

// apply_methods!(DomBuilder::new(text), {
//     // ...
//     .into_dom()
// })
//   OR
        // dom_builder!(dom_node_goes_here, {
        //     ...
        // })
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

// ------ Cow<str> ------

impl<'a> ApplyToElement<Text<'a>> for Cow<'a, str> {
    fn apply_to_element(self, text: &mut Text<'a>) {
        text.text = self;
    }
}
