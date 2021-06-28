use crate::*;

mod background;
mod font;
mod color;

pub use background::Background;
pub use font::Font;
pub use color::{Color, NamedColor};

pub trait Style {
    fn update_raw_el_style<T: RawEl>(self, raw_el: T) -> T;
}




