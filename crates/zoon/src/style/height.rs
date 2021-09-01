use crate::*;

#[derive(Default)]
pub struct Height<'a> {
    static_css_props: StaticCSSProps<'a>,
    static_css_classes: StaticCssClasses<'a>,
}

impl<'a> Height<'a> {
    pub fn new(height: u32) -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", px(height));
        this.static_css_classes.insert("exact_height".into());
        this.static_css_classes.remove("fill_height".into());
        this
    }

    pub fn fill() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100%".into());
        this.static_css_classes.insert("fill_height".into());
        this.static_css_classes.remove("exact_height".into());
        this
    }

    pub fn screen() -> Self {
        let mut this = Self::default();
        this.static_css_props.insert("height", "100vh".into());
        this.static_css_classes.insert("exact_height".into());
        this.static_css_classes.remove("fill_height".into());
        this
    }

    pub fn min_screen(mut self) -> Self {
        self.static_css_props.insert("min-height", "100vh".into());
        self
    }

    pub fn max(mut self, height: u32) -> Self {
        self.static_css_props.insert("max-height", px(height));
        self
    }
}

impl<'a> Style<'a> for Height<'a> {
    fn apply_to_raw_el<T: RawEl>(self, mut raw_el: T) -> T {
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for class in self.static_css_classes {
            raw_el = raw_el.class(&class);
        }
        raw_el
    }
}
