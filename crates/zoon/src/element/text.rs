use crate::*;
use std::borrow::Cow;

// ------ ------
//    Element
// ------ ------

pub struct Text {
    raw_text: RawText,
}

impl Text {
    pub fn new<'a>(text: impl IntoCowStr<'a>) -> Self {
        Self {
            raw_text: RawText::new(text.into_cow_str()),
        }
    }

    pub fn with_signal<'a>(text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static) -> Self {
        Self {
            raw_text: RawText::with_signal(text),
        }
    }
}

impl Element for Text {
    fn into_raw_element(self) -> RawElement {
        self.raw_text.into()
    }
}

// ------ ------
//  IntoElement
// ------ ------

impl<'a> IntoElement<'a> for String {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::new(self)
    }
}

impl<'a> IntoElement<'a> for &String {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::new(self)
    }
}

impl<'a> IntoElement<'a> for &str {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::new(self)
    }
}

impl<'a> IntoElement<'a> for Cow<'_, str> {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::new(self)
    }
}

macro_rules! make_into_element_impls {
    ($($type:ty),*) => (
        $(
        impl<'a> IntoElement<'a> for $type {
            type EL = Text;
            fn into_element(self) -> Self::EL {
                Text::new(self)
            }
        }
        )*
    )
}
make_into_element_impls!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);
