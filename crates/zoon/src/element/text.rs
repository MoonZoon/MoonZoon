use crate::{RenderContext, dom::dom_text, Element, __TrackedCall, __TrackedCallStack, ApplyToElement, render, element_macro};
use std::borrow::Cow;
use dominator::{Dom, text};

// ------ ------
//    Element 
// ------ ------

element_macro!(text, Text::default());

#[derive(Default)]
pub struct Text<'a> {
    key: u64,
    after_removes: Vec<Box<dyn FnOnce()>>,
    text: Cow<'a, str>,
}

impl<'a> Element for Text<'a> {
    #[topo::nested]
    fn render(self) -> Dom {
        text(&self.text)
        // dom_text(rcx, &self.text);

        // for after_remove in self.after_removes {
        //     builder = builder.after_removed(move |_| after_remove());
        // }
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a> Text<'a> {
    pub fn after_remove(mut self, after_remove: impl FnOnce() + 'static) -> Self {
        self.after_removes.push(Box::new(after_remove));
        self
    }

    pub fn after_removes(mut self, after_removes: Vec<Box<dyn FnOnce()>>) -> Self {
        self.after_removes.extend(after_removes);
        self
    }
}

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
