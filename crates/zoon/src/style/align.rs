use crate::*;

#[derive(Default)]
pub struct Align<'a> {
    static_css_classes: StaticCssClasses<'a>,
}

impl<'a> Align<'a> {
    pub fn center() -> Self {
        Self::default().center_x().center_y()
    }

    pub fn center_x(mut self) -> Self {
        self.static_css_classes.insert("center_x".into());
        self.static_css_classes.remove("align_left".into());
        self.static_css_classes.remove("align_right".into());
        self
    }

    pub fn center_y(mut self) -> Self {
        self.static_css_classes.insert("center_y".into());
        self.static_css_classes.remove("align_top".into());
        self.static_css_classes.remove("align_bottom".into());
        self
    }

    pub fn top(mut self) -> Self {
        self.static_css_classes.insert("align_top".into());
        self.static_css_classes.remove("center_y".into());
        self.static_css_classes.remove("align_bottom".into());
        self
    }

    pub fn bottom(mut self) -> Self {
        self.static_css_classes.insert("align_bottom".into());
        self.static_css_classes.remove("center_y".into());
        self.static_css_classes.remove("align_top".into());
        self
    }

    pub fn left(mut self) -> Self {
        self.static_css_classes.insert("align_left".into());
        self.static_css_classes.remove("center_x".into());
        self.static_css_classes.remove("align_right".into());
        self
    }

    pub fn right(mut self) -> Self {
        self.static_css_classes.insert("align_right".into());
        self.static_css_classes.remove("center_x".into());
        self.static_css_classes.remove("align_left".into());
        self
    }
}

impl<'a> Style<'a> for Align<'a> {
    fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E, style_group: Option<StyleGroup<'a>>) -> (E, Option<StyleGroup<'a>>) {
        for class in self.static_css_classes {
            raw_el = raw_el.class(&class);
        }
        (raw_el, style_group)
    }
}
