pub mod button;
pub mod column;
pub mod el;
pub mod row;
pub mod text;

use crate::Cx;

pub trait Control {
    fn build(&mut self, cx: Cx);
}
