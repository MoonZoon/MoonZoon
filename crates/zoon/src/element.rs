use dominator::Dom;
use futures_signals::signal::Signal;
use std::borrow::Cow;

// -- modules --

pub mod button;
pub use button::Button;

pub mod column;
pub use column::Column;

pub mod el;
pub use el::El;

pub mod row;
pub use row::Row;

pub mod text;
pub use text::Text;

pub mod raw_el;
// pub use raw_el::RawEl;

// ------ Element ------

pub trait Element {
    fn render(self) -> Dom;
}

// ------ IntoOptionElement ------

pub trait IntoOptionElement<'a> {
    type EL: Element;
    fn into_option_element(self) -> Option<Self::EL>; 
}

impl<'a, E: Element, T: IntoElement<'a, EL = E>> IntoOptionElement<'a> for Option<T> {
    type EL = E;
    fn into_option_element(self) -> Option<Self::EL> {
        self.map(|into_element| into_element.into_element())
    }
}

impl<'a, E: Element, T: IntoElement<'a, EL = E>> IntoOptionElement<'a> for T {
    type EL = E;
    fn into_option_element(self) -> Option<Self::EL> {
        Some(self.into_element())
    }
}

// ------ IntoElement ------

pub trait IntoElement<'a> {
    type EL: Element;
    fn into_element(self) -> Self::EL; 
}

impl<'a, T: Element> IntoElement<'a> for T {
    type EL = T;
    fn into_element(self) -> Self::EL {
        self
    }
}

impl<'a> IntoElement<'a> for String {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self)
    }
}

impl<'a> IntoElement<'a> for &'a str {
    type EL = Text;
    fn into_element(self) -> Self::EL {
        Text::with_text(self)
    }
}

impl<'a> IntoElement<'a> for Cow<'a, str> {
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





