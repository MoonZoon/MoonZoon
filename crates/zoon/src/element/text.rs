use crate::{Element, IntoElement};
use dominator::Dom;
use futures_signals::signal::{Signal, SignalExt};
use std::borrow::Cow;

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

    pub fn with_signal(text: impl Signal<Item = impl ToString> + Unpin + 'static) -> Self {
        Self {
            dom: dominator::text_signal(text.map(|text| text.to_string()))
        }
    }
}

impl Element for Text {
    fn render(self) -> Dom {
        self.dom
    }
}

// ------ ------
//  IntoElement 
// ------ ------

impl<'a> IntoElement<'a> for String {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self)
    }
}

impl<'a> IntoElement<'a> for &str {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self)
    }
}

impl<'a> IntoElement<'a> for Cow<'_, str> {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self)
    }
}

impl<'a> IntoElement<'a> for u8 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for u16 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for u32 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for u64 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for u128 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for usize {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for i8 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for i16 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for i32 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for i64 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for i128 {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

impl<'a> IntoElement<'a> for isize {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self.to_string())
    }
}

