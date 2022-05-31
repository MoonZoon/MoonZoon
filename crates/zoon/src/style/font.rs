use crate::*;
use std::borrow::Cow;

mod font_weight;
pub use font_weight::FontWeight;

mod font_family;
pub use font_family::FontFamily;

mod font_line;
pub use font_line::FontLine;

/// Styling to manage font.
#[derive(Default)]
pub struct Font<'a> {
    /// Static css properties used by zoon.
    static_css_props: StaticCSSProps<'a>,
    /// Customizable css properties which can be added.
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Font<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Define the font weight.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Font::new().weight(FontWeight::Bold))
    ///     .label("Click me");
    /// ```
    /// It can also be set manually.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Font::new().weight(FontWeight::Number(950)))
    ///     .label("Click me");
    /// ```
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.static_css_props
            .insert("font-weight", weight.number().into_cow_str());
        self
    }

    /// Define the font weight depending of signal's state..
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Font::new()
    ///         .weight_signal(hover_signal.map_bool(|| FontWeight::Bold, || FontWeight::Light)))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("Hover me");
    /// ```
    pub fn weight_signal(
        mut self,
        weight: impl Signal<Item = impl Into<Option<FontWeight>>> + Unpin + 'static,
    ) -> Self {
        let weight = weight.map(|weight| weight.into().map(|weight| weight.number()));
        self.dynamic_css_props
            .insert("font-weight".into(), box_css_signal(weight));
        self
    }

    /// Set the color.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let button = Button::new()
    ///     .s(Font::new().color(GREEN_7))
    ///     .label("Click me");
    /// ```
    /// Set the color with macro.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(Font::new().color(hsluv!(153.8, 99.1, 44.4)))
    ///     .label("Click me");
    /// ```
    pub fn color(mut self, color: impl Into<Option<HSLuv>>) -> Self {
        if let Some(color) = color.into() {
            self.static_css_props.insert("color", color.into_cow_str());
        }
        self
    }

    /// Set color depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::{named_color::*, *};
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Font::new().color_signal(hover_signal.map_bool(|| PINK_6, || BLUE_9)))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("Hover me");
    /// ```
    pub fn color_signal(
        mut self,
        color: impl Signal<Item = impl Into<Option<HSLuv>>> + Unpin + 'static,
    ) -> Self {
        let color = color.map(|color| color.into().map(|color| color.into_cow_str()));
        self.dynamic_css_props
            .insert("color".into(), box_css_signal(color));
        self
    }

    /// Set the font size.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(Font::new().size(350)).label("Click me");
    /// ```
    pub fn size(mut self, size: u32) -> Self {
        self.static_css_props.insert("font-size", px(size));
        self
    }

    /// Set font size depending of signal's state.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let (hovered, hover_signal) = Mutable::new_and_signal(false);
    /// let button = Button::new()
    ///     .s(Font::new().size_signal(hover_signal.map_bool(|| 350, || 150)))
    ///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
    ///     .label("Hover me");
    /// ```
    pub fn size_signal(
        mut self,
        size: impl Signal<Item = impl Into<Option<u32>>> + Unpin + 'static,
    ) -> Self {
        let size = size.map(|size| size.into().map(px));
        self.dynamic_css_props
            .insert("font-size".into(), box_css_signal(size));
        self
    }

    /// Set the text line height in pixels.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .s(Font::new().line_height(150))
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn line_height(mut self, line_height: u32) -> Self {
        self.static_css_props.insert("line-height", px(line_height));
        self
    }

    /// Set the font as Italic.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .s(Font::new().italic())
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn italic(mut self) -> Self {
        self.static_css_props.insert("font-style", "italic");
        self
    }

    /// Don't wrap the text according to its parent boundaries.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .s(Font::new().no_wrap())
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn no_wrap(mut self) -> Self {
        self.static_css_props.insert("white-space", "pre");
        self
    }

    /// Set the text to be wrapped withing its element and prevent overflow.
    ///
    /// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/word-break>.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use zoon::*;
    ///
    ///  let paragraph =  Paragraph::new()
    ///     .s(Width::exact(50))
    ///     .s(Font::new().wrap_anywhere())
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn wrap_anywhere(mut self) -> Self {
        // @TODO replace with the line below once `overflow-wrap: anywhere` works on
        // Safari https://developer.mozilla.org/en-US/docs/Web/CSS/overflow-wrap#browser_compatibility
        self.static_css_props.insert("word-break", "break-word");
        // self.static_css_props.insert("overflow-wrap", "anywhere");

        self.static_css_props.remove("white-space");
        self
    }

    /// Horizontally center the text inside its element.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///    .s(Font::new().center())
    ///    .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn center(mut self) -> Self {
        self.static_css_props.insert("text-align", "center");
        self
    }

    /// Set the family font with an array of [FontFamily].
    /// It is recommended to have few family fonts as
    /// fallback if the first ones are not available for web
    /// as documented here: <https://developer.mozilla.org/en-US/docs/Web/CSS/font-family>
    ///
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///    .s(Font::new().family([
    ///     FontFamily::new("Helvetica Neue"),
    ///     FontFamily::new("Helvetica"),
    ///     FontFamily::new("Arial"),
    ///     FontFamily::SansSerif,
    ///    ]))
    ///    .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn family(mut self, family: impl IntoIterator<Item = FontFamily<'a>>) -> Self {
        let font_family = family
            .into_iter()
            .map(|family| family.into_cow_str())
            .collect::<Cow<_>>()
            .join(", ");
        self.static_css_props.insert("font-family", font_family);
        self
    }

    /// Add underlining for the text with specific styling.
    /// [FontLine] has many options available.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let paragraph = Paragraph::new()
    ///     .s(Font::new().line(FontLine::new().underline().dashed()))
    ///     .content("Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit...");
    /// ```
    pub fn line(mut self, line: FontLine<'a>) -> Self {
        self.static_css_props
            .extend(line.static_css_props.into_iter());
        self.dynamic_css_props
            .extend(line.dynamic_css_props.into_iter());
        self
    }
}

impl<'a> Style<'a> for Font<'a> {
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
