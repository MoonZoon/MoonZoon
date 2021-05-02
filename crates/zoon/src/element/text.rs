use crate::Element;
use dominator::{Dom, traits::AsStr};
use futures_signals::signal::Signal;

// ------ ------
//    Element 
// ------ ------

pub struct Text {
    dom: Dom,
}

impl Text {
    pub fn with_text(text: impl AsRef<str>) -> Self {
        Self {
            dom: dominator::text(text.as_ref()),
        }
    }

    pub fn with_signal(text: impl Signal<Item = impl AsStr> + Unpin + 'static) -> Self {
        Self {
            dom: dominator::text_signal(text),
        }
    }
}

impl Element for Text {
    fn render(self) -> Dom {
        self.dom
    }
}

// ------ ------
//   IntoText 
// ------ ------

