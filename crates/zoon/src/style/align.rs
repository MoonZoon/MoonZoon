use crate::*;
use std::borrow::Cow;

#[derive(Default)]
pub struct Align<'a> {
    static_classes: Vec<Cow<'a, str>>,
}

impl<'a> Align<'a> {
    pub fn center_x() -> Self {
        let mut this = Self::default();
        this.static_classes.push("center_x".into());
        this
    }

    pub fn center_y() -> Self {
        let mut this = Self::default();
        this.static_classes.push("center_y".into());
        this
    }

    pub fn left() -> Self {
        let mut this = Self::default();
        this.static_classes.push("align_left".into());
        this
    }

    pub fn right() -> Self {
        let mut this = Self::default();
        this.static_classes.push("align_right".into());
        this
    }
}

impl<'a> Style<'a> for Align<'a> {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        CssPropsContainer::default().static_classes(self.static_classes)
    }
}
