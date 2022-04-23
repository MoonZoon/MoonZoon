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
    fn merge_with_group(self, mut group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self {
            static_css_props,
            dynamic_css_props,
        } = self;
        group.static_css_props.extend(static_css_props);
        group.dynamic_css_props.extend(dynamic_css_props);
        group
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
}
