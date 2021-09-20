use crate::*;
use std::{array, borrow::Cow};

// ------ Shadows ------

#[derive(Default)]
pub struct Shadows<'a> {
    static_css_props: StaticCSSProps<'a>,
}

impl<'a> Shadows<'a> {
    pub fn new(shadows: impl IntoIterator<Item = Shadow<'a>>) -> Self {
        let shadows = shadows
            .into_iter()
            .map(|shadow| shadow.into_cow_str())
            .collect::<Cow<_>>()
            .join(", ");
        let mut this = Self::default();
        this.static_css_props.insert("box-shadow", shadows.into());
        this
    }
}

impl<'a> Style<'a> for Shadows<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        if let Some(mut style_group) = style_group {
            for (name, value) in self.static_css_props {
                style_group = style_group.style(name, value);
            }
            return (raw_el, Some(style_group));
        }
        for (name, value) in self.static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        (raw_el, None)
    }
}

// ------ Shadow ------

#[derive(Default)]
pub struct Shadow<'a> {
    inner: bool,
    x: i32,
    y: i32,
    spread: i32,
    blur: u32,
    color: Option<Cow<'a, str>>,
}

impl<'a> Shadow<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inner(mut self) -> Self {
        self.inner = true;
        self
    }

    pub fn x(mut self, x: i32) -> Self {
        self.x = x;
        self
    }

    pub fn y(mut self, y: i32) -> Self {
        self.y = y;
        self
    }

    pub fn spread(mut self, spread: i32) -> Self {
        self.spread = spread;
        self
    }

    pub fn blur(mut self, blur: u32) -> Self {
        self.blur = blur;
        self
    }

    pub fn color(mut self, color: impl Color<'a> + 'a) -> Self {
        self.color = Some(color.into_cow_str());
        self
    }
}

impl<'a> IntoCowStr<'a> for Shadow<'a> {
    fn into_cow_str(self) -> Cow<'a, str> {
        let mut shadow_settings = Vec::<Cow<_>>::new();
        if self.inner {
            shadow_settings.push("inset".into())
        }
        shadow_settings.extend(array::IntoIter::new([
            px(self.x),
            px(self.y),
            px(self.blur),
            px(self.spread),
        ]));
        if let Some(color) = self.color {
            shadow_settings.push(color);
        }
        shadow_settings.join(" ").into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        unimplemented!()
    }
}
