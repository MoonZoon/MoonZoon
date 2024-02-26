use crate::*;
use std::borrow::Cow;

#[derive(Clone)]
pub struct Outline {
    width: u32,
    style: OutlineStyle,
    color: CssColor,
    inner: bool,
    self_signal: Option<Broadcaster<LocalBoxSignal<'static, Option<Self>>>>,
}

impl Default for Outline {
    fn default() -> Self {
        Self {
            width: 1,
            style: OutlineStyle::Solid,
            color: oklch().into_color(),
            inner: false,
            self_signal: None,
        }
    }
}

impl Outline {
    pub fn inner() -> Self {
        let mut this = Self::default();
        this.inner = true;
        this
    }

    pub fn outer() -> Self {
        let mut this = Self::default();
        this.inner = false;
        this
    }

    pub fn with_signal(
        outline: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        this.self_signal = Some(
            outline
                .map(|outline| outline.into())
                .boxed_local()
                .broadcast(),
        );
        this
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn color(mut self, color: impl IntoOptionColor) -> Self {
        if let Some(color) = color.into_option_color() {
            self.color = color;
        }
        self
    }

    pub fn solid(mut self) -> Self {
        self.style = OutlineStyle::Solid;
        self
    }

    pub fn dashed(mut self) -> Self {
        self.style = OutlineStyle::Dashed;
        self
    }

    pub fn dotted(mut self) -> Self {
        self.style = OutlineStyle::Dotted;
        self
    }

    fn to_css_outline_value(&self) -> Cow<'static, str> {
        crate::format!(
            "{}px {} {}",
            self.width,
            self.style.as_str(),
            self.color.clone().into_color_string()
        )
        .into()
    }

    fn to_css_outline_offset_value(&self) -> Cow<'static, str> {
        if self.inner {
            return px(self.width as i32 * -1);
        }
        px(0)
    }
}

impl<'a> Style<'a> for Outline {
    fn move_to_groups(self, groups: &mut StyleGroups<'a>) {
        groups.update_first(|mut group| {
            if let Some(self_signal) = self.self_signal {
                group.dynamic_css_props.insert(
                    "outline".into(),
                    box_css_signal(self_signal.signal_ref(|outline| {
                        outline
                            .as_ref()
                            .map(|outline| outline.to_css_outline_value())
                    })),
                );
                group.dynamic_css_props.insert(
                    "outline-offset".into(),
                    box_css_signal(self_signal.signal_ref(|outline| {
                        outline
                            .as_ref()
                            .map(|outline| outline.to_css_outline_offset_value())
                    })),
                );
            } else {
                group
                    .static_css_props
                    .insert("outline", self.to_css_outline_value());
                group
                    .static_css_props
                    .insert("outline-offset", self.to_css_outline_offset_value());
            }
            group
        });
    }
}

// ------ OutlineStyle ------

// @TODO unify with `BorderStyle`?
// and also unify methods like `solid` under one trait?

#[derive(Clone, Copy)]
enum OutlineStyle {
    Solid,
    Dashed,
    Dotted,
}

impl OutlineStyle {
    fn as_str(&self) -> &str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }
}
