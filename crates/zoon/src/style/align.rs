use crate::*;

#[derive(Default)]
pub struct Align<'a> {
    static_css_classes: StaticCssClasses<'a>,
    dynamic_css_classes: DynamicCssClasses,
}

impl<'a> Align<'a> {
    pub fn with_signal(
        align: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let align = Broadcaster::new(align.map(|align| align.into()));

        this.dynamic_css_classes.insert("center_x".into(), Box::new(align.signal_ref(|align| {
            align.as_ref().map_or(false, |align| align.static_css_classes.contains("center_x"))
        }).dedupe()));

        this.dynamic_css_classes.insert("center_y".into(), Box::new(align.signal_ref(|align| {
            align.as_ref().map_or(false, |align| align.static_css_classes.contains("center_y"))
        }).dedupe()));

        this.dynamic_css_classes.insert("align_left".into(), Box::new(align.signal_ref(|align| {
            align.as_ref().map_or(false, |align| align.static_css_classes.contains("align_left"))
        }).dedupe()));

        this.dynamic_css_classes.insert("align_right".into(), Box::new(align.signal_ref(|align| {
            align.as_ref().map_or(false, |align| align.static_css_classes.contains("align_right"))
        }).dedupe()));

        this.dynamic_css_classes.insert("align_top".into(), Box::new(align.signal_ref(|align| {
            align.as_ref().map_or(false, |align| align.static_css_classes.contains("align_top"))
        }).dedupe()));

        this.dynamic_css_classes.insert("align_bottom".into(), Box::new(align.signal_ref(|align| {
            align.as_ref().map_or(false, |align| align.static_css_classes.contains("align_bottom"))
        }).dedupe()));

        this
    }

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
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        for class in self.static_css_classes {
            raw_el = raw_el.class(&class);
        }
        for (class, enabled) in self.dynamic_css_classes {
            raw_el = raw_el.class_signal(class, enabled);
        }
        (raw_el, style_group)
    }
}
