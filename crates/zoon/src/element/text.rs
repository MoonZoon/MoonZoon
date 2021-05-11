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

    pub fn with_signal(text: impl Signal<Item = impl ToString> + Unpin + 'static) -> Self {
        Self {
            raw_text: RawText::with_signal(text)
        }
    }
}

impl Element for Text {
    fn into_raw_element(self) -> RawElement {
        self.raw_text.into()
    }
}

// ------ ------
//     ToStr
// ------ ------

pub trait IntoCowStr<'a> {
    fn into_cow_str(self) -> Cow<'a, str>; 
}

impl<'a> IntoCowStr<'a> for String {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> IntoCowStr<'a> for &'a String {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> IntoCowStr<'a> for &'a str {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> IntoCowStr<'a> for Cow<'a, str> {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

macro_rules! make_into_cow_str_impls {
    ($($type:ty),*) => (
        $(
        impl<'a> IntoCowStr<'a> for $type {
            fn into_cow_str(self) -> Cow<'a, str> {
                self.to_string().into()
            }
        }
        )*
    )
}
make_into_cow_str_impls!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

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

