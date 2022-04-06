use crate::*;
use std::borrow::Cow;

// ------ Transitions ------

/// Define transitions for an element between two states.
/// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/transition>
#[derive(Default)]
pub struct Transitions<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Transitions<'a> {
    /// Apply a transformation to an element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    /// let button = Button::new()
    ///     .update_raw_el(|raw_el| {
    ///         raw_el.style_group(StyleGroup::new(":hover").style("margin-right", "40%"))
    ///     })
    ///     .s(Transitions::new([
    ///         Transition::property("margin-right").duration(2000)
    ///     ]))
    ///     .label("Hover me");
    /// ```
    pub fn new(transitions: impl IntoIterator<Item = Transition>) -> Self {
        let transitions = transitions
            .into_iter()
            .map(|shadow| shadow.into_cow_str())
            .collect::<Cow<_>>()
            .join(", ");
        let mut this = Self::default();
        this.static_css_props.insert("transition", transitions);
        this
    }

    // @TODO: write documentation when Api is improved.
    pub fn with_signal(
        transitions: impl Signal<Item = impl IntoIterator<Item = Transition>> + Unpin + 'static,
    ) -> Self {
        let transitions = transitions.map(|transitions| {
            transitions
                .into_iter()
                .map(|shadow| shadow.into_cow_str())
                .collect::<Cow<_>>()
                .join(", ")
        });
        let mut this = Self::default();
        this.dynamic_css_props
            .insert("transition".into(), box_css_signal(transitions));
        this
    }
}

impl<'a> Style<'a> for Transitions<'a> {
    fn apply_to_raw_el<E: RawEl>(
        self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        if let Some(mut style_group) = style_group {
            for (name, css_prop_value) in self.static_css_props {
                style_group = if css_prop_value.important {
                    style_group.style(name, css_prop_value.value)
                } else {
                    style_group.style_important(name, css_prop_value.value)
                };
            }
            for (name, value) in self.dynamic_css_props {
                style_group = style_group.style_signal(name, value);
            }
            return (raw_el, Some(style_group));
        }
        for (name, css_prop_value) in self.static_css_props {
            raw_el = if css_prop_value.important {
                raw_el.style_important(name, &css_prop_value.value)
            } else {
                raw_el.style(name, &css_prop_value.value)
            };
        }
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        (raw_el, None)
    }
}

// ------ Transition ------

pub struct Transition {
    property: Cow<'static, str>,
    duration: u32,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            property: "all".into(),
            duration: 1000,
        }
    }
}

impl Transition {
    pub fn property(property: impl IntoCowStr<'static>) -> Self {
        Self {
            property: property.into_cow_str(),
            ..Self::default()
        }
    }

    pub fn all() -> Self {
        Self::default()
    }

    pub fn width() -> Self {
        Self::property("width")
    }

    pub fn height() -> Self {
        Self::property("height")
    }

    pub fn transform() -> Self {
        Self::property("transform")
    }

    pub fn color() -> Self {
        Self::property("color")
    }

    pub fn background_color() -> Self {
        Self::property("background-color")
    }

    pub fn duration(mut self, ms: u32) -> Self {
        self.duration = ms;
        self
    }
}

impl<'a> IntoCowStr<'a> for Transition {
    fn into_cow_str(self) -> Cow<'a, str> {
        crate::format!("{} {}ms", self.property, self.duration).into()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        unimplemented!()
    }
}
