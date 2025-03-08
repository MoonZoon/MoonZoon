use crate::*;
use std::{iter, vec};

// -- modules --

pub mod button;
pub use button::{Button, ButtonPressEvent};

pub mod canvas;
pub use canvas::Canvas;

pub mod checkbox;
pub use checkbox::Checkbox;

pub mod column;
pub use column::Column;

pub mod el;
pub use el::El;

pub mod grid;
pub use grid::Grid;

pub mod image;
pub use image::Image;

pub mod label;
pub use label::Label;

pub mod link;
pub use link::{Link, NewTab};

pub mod paragraph;
pub use paragraph::Paragraph;

pub mod row;
pub use row::Row;

pub mod spacer;
pub use spacer::Spacer;

pub mod stack;
pub use stack::Stack;

pub mod stripe;
pub use stripe::{Direction, Stripe};

pub mod text;
pub use text::Text;

pub mod text_area;
pub use text_area::TextArea;

pub mod text_input;
pub use text_input::{InputType, Placeholder, TextInput};

// --

pub mod raw_el;
pub use raw_el::{RawEl, RawElWrapper, RawHtmlEl, RawSvgEl};

pub mod raw_text;
pub use raw_text::RawText;

// --

pub mod ability;
pub use ability::*;

mod raw_el_or_text;
pub use raw_el_or_text::RawElOrText;

// ------ ElementUnchecked ------

pub trait ElementUnchecked {
    fn into_raw_unchecked(self) -> RawElOrText;
}

impl<REW: RawElWrapper> ElementUnchecked for REW {
    fn into_raw_unchecked(self) -> RawElOrText {
        self.into_raw_el().into()
    }
}

// ------ Element ------

pub trait Element: ElementUnchecked + Sized {
    fn into_raw(self) -> RawElOrText {
        self.into_raw_unchecked()
    }
}

// ------ IntoDom ------

pub trait IntoDom {
    fn into_dom(self) -> Dom;
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

// ------ IntoElementIterator ------

pub trait IntoElementIterator {
    type Item: Element;
    type IntoIter: Iterator<Item = Self::Item>;

    fn into_element_iter(self) -> Self::IntoIter;
}

impl<E: Element> IntoElementIterator for E {
    type Item = E;
    type IntoIter = iter::Once<E>;

    #[inline]
    fn into_element_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<E: Element> IntoElementIterator for Vec<E> {
    type Item = E;
    type IntoIter = vec::IntoIter<E>;

    #[inline]
    fn into_element_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}
