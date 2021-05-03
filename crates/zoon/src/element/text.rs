use crate::*;
use std::borrow::Cow;

// ------ ------
//    Element 
// ------ ------

pub struct Text {
    raw_text: RawText,
}

impl Text {
    pub fn with_text(text: impl AsRef<str>) -> Self {
        Self {
            raw_text: RawText::with_text(text),
        }
    }

    pub fn with_signal(text: impl Signal<Item = impl ToString> + Unpin + 'static) -> Self {
        Self {
            raw_text: RawText::with_signal(text)
        }
    }
}

impl Element for Text {
    fn into_raw<RE: RawElement>(self) -> RE {
        self.raw_el
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

impl<'a> IntoElement<'a> for &String {
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

