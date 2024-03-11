// ------ LineHeight ------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineHeight {
    Px(u32),
    Normal,
}

impl Default for LineHeight {
    fn default() -> Self {
        Self::Normal
    }
}

impl From<u32> for LineHeight {
    fn from(line_height: u32) -> Self {
        Self::Px(line_height)
    }
}

// ------ IntoOptionLineHeight ------

pub trait IntoOptionLineHeight {
    fn into_option_line_height(self) -> Option<LineHeight>;
}

impl<T: Into<LineHeight>> IntoOptionLineHeight for T {
    fn into_option_line_height(self) -> Option<LineHeight> {
        Some(self.into())
    }
}

impl<T: Into<LineHeight>> IntoOptionLineHeight for Option<T> {
    fn into_option_line_height(self) -> Option<LineHeight> {
        self.map(Into::into)
    }
}
