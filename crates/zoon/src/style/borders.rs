use crate::*;
use std::borrow::{Borrow, Cow};

// ------ Borders ------

/// Styling to set borders for an element.
#[derive(Default)]
pub struct Borders<'a> {
    /// Default static properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable properties that can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Borders<'a> {
    /// Set properties for bottom, left, right and top borders together.
    ///
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::all(Border::new().color(GREEN_7).dotted().width(5)))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn all(border: impl Borrow<Border>) -> Self {
        let border = border.borrow();
        Self::default().x(border).y(border)
    }

    /// Set borders properties depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::all_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dashed(),
    ///         || Border::new().color(PINK_0).dotted(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn all_signal(border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = Broadcaster::new(border);
        Self::default()
            .x_signal(border.signal_cloned())
            .y_signal(border.signal_cloned())
    }

    /// Set left & right borders.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::new().x(Border::new().color(GREEN_7)))
    ///     .s(Width::new(50))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn x(self, border: impl Borrow<Border>) -> Self {
        let border = border.borrow();
        self.left(border).right(border)
    }

    /// Set left & right borders depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::new().x_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dotted(),
    ///         || Border::new().color(PINK_0).dashed(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn x_signal(self, border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = Broadcaster::new(border);
        self.left_signal(border.signal_cloned())
            .right_signal(border.signal_cloned())
    }

    /// Set bottom and top borders.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::new().y(Border::new().color(GREEN_7)))
    ///     .s(Width::new(50))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn y(self, border: impl Borrow<Border>) -> Self {
        let border = border.borrow();
        self.top(border).bottom(border)
    }

    /// Set bottom and top borders depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::new().y_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dotted(),
    ///         || Border::new().color(PINK_0).dashed(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn y_signal(self, border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = Broadcaster::new(border);
        self.top_signal(border.signal_cloned())
            .bottom_signal(border.signal_cloned())
    }

    /// Set the top border.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::new().top(Border::new().color(GREEN_7)))
    ///     .s(Width::new(50))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn top(mut self, border: impl Borrow<Border>) -> Self {
        self.static_css_props
            .insert("border-top", border.borrow().to_cow_str());
        self
    }

    /// Set the top border depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::new().top_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dotted(),
    ///         || Border::new().color(PINK_0).dashed(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn top_signal(mut self, border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-top".into(), box_css_signal(border));
        self
    }

    /// Set the bottom border.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::new().bottom(Border::new().color(GREEN_7)))
    ///     .s(Width::new(50))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn bottom(mut self, border: impl Borrow<Border>) -> Self {
        self.static_css_props
            .insert("border-bottom", border.borrow().to_cow_str());
        self
    }

    /// Set the bottom border depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::new().bottom_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dotted(),
    ///         || Border::new().color(PINK_0).dashed(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn bottom_signal(mut self, border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-bottom".into(), box_css_signal(border));
        self
    }

    /// Set the right border.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::new().right(Border::new().color(GREEN_7)))
    ///     .s(Width::new(50))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn right(mut self, border: impl Borrow<Border>) -> Self {
        self.static_css_props
            .insert("border-right", border.borrow().to_cow_str());
        self
    }

    /// Set the right border depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::new().right_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dotted(),
    ///         || Border::new().color(PINK_0).dashed(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn right_signal(mut self, border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-right".into(), box_css_signal(border));
        self
    }

    /// Set the left border.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let button = Button::new()
    ///     .s(Borders::new().left(Border::new().color(GREEN_7)))
    ///     .s(Width::new(50))
    ///     .s(Background::new().color(BLUE_9))
    ///     .label("I have four borders");
    /// ```
    pub fn left(mut self, border: impl Borrow<Border>) -> Self {
        self.static_css_props
            .insert("border-left", border.borrow().to_cow_str());
        self
    }

    /// Set the left border depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Borders::new().left_signal(hovered_signal.map_bool(
    ///         || Border::new().color(GREEN_7).dotted(),
    ///         || Border::new().color(PINK_0).dashed(),
    ///     )))
    ///     .s(Background::new().color(BLUE_9))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("hover me");
    /// ```
    pub fn left_signal(mut self, border: impl Signal<Item = Border> + Unpin + 'static) -> Self {
        let border = border.map(|border| border.to_cow_str());
        self.dynamic_css_props
            .insert("border-left".into(), box_css_signal(border));
        self
    }
}

impl<'a> Style<'a> for Borders<'a> {
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

// ------ Border ------

/// Single border definition.
#[derive(Clone)]
pub struct Border {
    /// Width in pixels.
    width: u32,
    /// Style to apply.
    style: BorderStyle,
    /// Color with Hsluv standard.
    color: HSLuv,
}

impl Border {
    /// Construct by default a border with `1 pixel` width, `Solid` border style
    /// and `black` color.
    pub fn new() -> Self {
        Self {
            width: 1,
            style: BorderStyle::Solid,
            color: hsluv!(0, 0, 0),
        }
    }

    /// Set the width to apply for a border.
    /// ```no_run
    /// use zoon::*;
    /// let thick_border = Border::new().width(4);
    /// ```
    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    /// Set the color for a border.
    /// ```no_run
    /// use zoon::{named_color::*, *};
    /// let green_border = Border::new().color(GREEN_7);
    /// ```
    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.color = color;
        }
        self
    }

    /// Set the border style to solid.
    /// ```no_run
    /// use zoon::*;
    /// let solid_border = Border::new().solid();
    /// ```
    pub fn solid(mut self) -> Self {
        self.style = BorderStyle::Solid;
        self
    }

    /// Set the border style to dashed.
    /// ```no_run
    /// use zoon::*;
    /// let solid_border = Border::new().dashed();
    /// ```
    pub fn dashed(mut self) -> Self {
        self.style = BorderStyle::Dashed;
        self
    }

    /// Set the border style to dotted.
    /// ```no_run
    /// use zoon::*;
    /// let dotted_border = Border::new().dotted();
    /// ```
    pub fn dotted(mut self) -> Self {
        self.style = BorderStyle::Dotted;
        self
    }

    /// Convert the border properties as `Cow<'static, str>`.
    fn to_cow_str(&self) -> Cow<'static, str> {
        crate::format!(
            "{}px {} {}",
            self.width,
            self.style.as_str(),
            self.color.into_cow_str()
        )
        .into()
    }
}

// ------ BorderStyle ------

/// Border line styling.
#[derive(Clone, Copy)]
enum BorderStyle {
    Solid,
    Dashed,
    Dotted,
}

impl BorderStyle {
    fn as_str(&self) -> &str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }
}
