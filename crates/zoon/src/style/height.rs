use crate::{style::supports_dvx, *};
use std::{cell::Cell, collections::BTreeMap};
use strum::{EnumIter, IntoEnumIterator, IntoStaticStr};

/// Styling for height.
#[derive(Default)]
pub struct Height<'a> {
    css_props: BTreeMap<CssName, Cell<Option<CssPropValue<'a>>>>,
    height_mode: HeightMode,
    self_signal: Option<Box<dyn Signal<Item = Option<Self>> + Unpin>>,
}

fn into_prop_value<'a>(value: impl IntoCowStr<'a>) -> Cell<Option<CssPropValue<'a>>> {
    Cell::new(Some(CssPropValue::new(value)))
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "kebab-case")]
enum CssName {
    MinHeight,
    Height,
    MaxHeight,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
enum HeightMode {
    #[default]
    ExactHeight,
    FillHeight,
}

impl<'a> Height<'a> {
    /// Define the height with pixels for an element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Height::exact(50)).label("Click me");
    /// ```
    pub fn exact(height: u32) -> Self {
        let mut this = Self::default();
        this.css_props
            .insert(CssName::Height, into_prop_value(px(height)));
        this.height_mode = HeightMode::ExactHeight;
        this
    }

    /// Define the height with pixels depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (is_hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Height::exact_signal(hover_signal.map_bool(|| 50, || 100)))
    ///     .on_hovered_change(move |hover| is_hovered.set(hover))
    ///     .label("hover me");
    /// ```
    pub fn exact_signal(
        height: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        Self::with_signal(height.map(|height| height.into().map(Height::exact)))
    }

    pub fn with_signal(
        height: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let height = height.map(|height| height.into());
        this.self_signal = Some(Box::new(height));
        this
    }

    /// Set the element height to fill its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn fill() -> Self {
        let mut this = Self::default();
        this.css_props
            .insert(CssName::Height, into_prop_value("100%"));
        this.height_mode = HeightMode::FillHeight;
        this
    }

    /// The element height will be the height of the device screen or web
    /// browser frame.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::screen())
    ///     .label("Hover this giant button");
    /// ```
    pub fn screen() -> Self {
        let mut this = Self::default();
        this.css_props.insert(
            CssName::Height,
            into_prop_value(if *supports_dvx() { "100dvh" } else { "100vh" }),
        );
        this.height_mode = HeightMode::ExactHeight;
        this
    }

    pub fn min(mut self, height: u32) -> Self {
        self.css_props
            .insert(CssName::MinHeight, into_prop_value(px(height)));
        self
    }

    /// The element minimum height will be the height of thw device screen or
    /// web browser frame.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill().min_screen())
    ///     .label("Hover this giant button");
    /// ```
    pub fn min_screen(mut self) -> Self {
        self.css_props.insert(
            CssName::MinHeight,
            into_prop_value(if *supports_dvx() { "100dvh" } else { "100vh" }),
        );
        self
    }

    /// The element maximum height can be set by value in pixels.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill().max(150))
    ///     .label("Hover this giant button");
    /// ```
    pub fn max(mut self, height: u32) -> Self {
        self.css_props
            .insert(CssName::MaxHeight, into_prop_value(px(height)));
        self
    }

    /// Set the maximum element height to fill its container.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Height::fill().max_fill())
    ///     .label("Hover this giant button");
    /// ```
    pub fn max_fill(mut self) -> Self {
        self.css_props
            .insert(CssName::MaxHeight, into_prop_value("100%"));
        self
    }
}

impl<'a> Style<'a> for Height<'static> {
    fn merge_with_group(self, mut group: StyleGroup<'a>) -> StyleGroup<'a> {
        let Self {
            css_props,
            height_mode,
            self_signal,
        } = self;

        if let Some(self_signal) = self_signal {
            let self_signal = self_signal.broadcast();

            for name in CssName::iter() {
                group = group.style_signal(
                    <&str>::from(name),
                    self_signal.signal_ref(move |this| {
                        this.as_ref().and_then(|this| {
                            this.css_props
                                .get(&name)
                                .and_then(|value| value.take().map(|value| value.value))
                        })
                    }),
                );
            }

            for mode in HeightMode::iter() {
                group = group.class_signal(
                    <&str>::from(mode),
                    self_signal.signal_ref(move |this| {
                        this.as_ref()
                            .map(|this| this.height_mode == mode)
                            .unwrap_or_default()
                    }),
                );
            }
            group
        } else {
            group.static_css_props.extend(
                css_props
                    .into_iter()
                    .map(|(name, value)| (name.into(), value.take().unwrap_throw())),
            );
            group.class(height_mode.into())
        }
    }
}
