use crate::{web_sys::HtmlDivElement, *};
use std::iter;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Label, OnPress);

/// Create a customizable Button for your web page.
/// The button is actually built using a div to avoid default behaviors and
/// styling from regular Html buttons.
/// You can create a new button by using its constructor and chain styling.
///
/// # Example
/// ```no_run
/// use zoon::*;
/// let button = Button::new()
///     .s(Align::center())
///     .s(Padding::all(5))
///     .label("Click me");
/// ```
/// You can also create your button with specific events as well and update the styling the way you need by using [signals](https://crates.io/crates/futures-signals)
///
/// 1 - Create a signal for local state management.
///
/// 2 - The variable **`hovered`** gets updated when the user hovers the button
///
/// 3 - The variable **`hovered_signal`** is the notification send by signal
/// that will actually determine the background color for the button.
///
/// # Example
/// ```no_run
/// use zoon::{named_color::*, *};
/// let (hovered, hover_signal) = Mutable::new_and_signal(false);
///
/// let button = Button::new()
///     .s(Background::new()
///         .color_signal(hover_signal.map_bool(|| GREEN_7, || GREEN_8)))
///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
///     .label("Hover me");
/// ```
/// It is possible to style a button with different properties such as `width` ,
/// `height` or `font`. Colors are available with macros as well.
///
/// # Example
/// ```no_run
/// use zoon::*;
/// let (hovered, hover_signal) = Mutable::new_and_signal(false);
///
/// let button = Button::new()
///     .s(Width::new(40))
///     .s(Height::new(40))
///     .s(Font::new().size(30).center().color_signal(
///         hover_signal
///             .map_bool(|| hsluv!(10.5, 37.7, 48.8), || hsluv!(12.2, 34.7, 68.2)),
///     ))
///     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
///     .label("Hover me");
/// ```
/// It is also possible to create a button with local state for dynamic updates.
///
/// # Example
/// ```no_run
/// use std::borrow::Cow;
/// use zoon::{named_color::*, *};
///
/// let click_count = Mutable::new(0);
///
/// // Create a title that gets updated when counting changes.
/// let title = click_count.signal().map(|count| {
///     if count == 0 {
///         return Cow::from("Click me!");
///     }
///     Cow::from(format!("Clicked {}x", count))
/// });
///
/// // Create a row with a button.
/// // The button's label is dynamic and changing according to the title signal.
/// // Clicking the button will increment the click_count value.
/// let raw_with_a_button = Row::new().item(
///     Button::new()
///         .label_signal(title)
///         .on_press(move || click_count.update(|count| count + 1)),
/// );
/// ```
pub struct Button<LabelFlag, OnPressFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(LabelFlag, OnPressFlag)>,
}

impl Button<LabelFlagNotSet, OnPressFlagNotSet> {
    pub fn new() -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".button > *")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(
                    StyleGroup::new(".button > .align_top")
                        .style("margin-bottom", "auto"),
                )
                .style_group(
                    StyleGroup::new(".button > .align_bottom")
                        .style("margin-top", "auto"),
                )
                .style_group(
                    StyleGroup::new(".button > .align_left")
                        .style("align-self", "flex-start"),
                )
                .style_group(
                    StyleGroup::new(".button > .align_right")
                        .style("align-self", "flex-end"),
                )
                .style_group(
                    StyleGroup::new(".button > .exact_height")
                        .style("flex-shrink", "0"),
                )
                .style_group(
                    StyleGroup::new(".button > .fill_height")
                        .style("flex-grow", "1"),
                );
        });
        Self {
            raw_el: RawHtmlEl::new("div")
                .class("button")
                .attr("role", "button")
                .attr("tabindex", "0")
                .style("cursor", "pointer")
                .style("user-select", "none")
                .style("text-align", "center")
                .style("display", "inline-flex")
                .style("flex-direction", "column")
                .style("align-items", "center")
                .style("touch-action", "manipulation"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, OnPressFlag> IntoIterator for Button<LabelFlag, OnPressFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<LabelFlag, OnPressFlag> UpdateRawEl<RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
    fn update_raw_el(
        mut self,
        updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl,
    ) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, OnPressFlag> Styleable<'_, RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
}
impl<LabelFlag, OnPressFlag> KeyboardEventAware<RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
}
impl<LabelFlag, OnPressFlag> Focusable for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> MouseEventAware<RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
}
impl<LabelFlag, OnPressFlag> PointerEventAware<RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
}
impl<LabelFlag, OnPressFlag> TouchEventAware<RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
}
impl<LabelFlag, OnPressFlag> Hookable<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {
    type WSElement = HtmlDivElement;
}
impl<LabelFlag, OnPressFlag> AddNearbyElement<'_>
    for Button<LabelFlag, OnPressFlag>
{
}
impl<LabelFlag, OnPressFlag> HasClassId<RawHtmlEl>
    for Button<LabelFlag, OnPressFlag>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(
        mut self,
        label: impl IntoElement<'a> + 'a,
    ) -> Button<LabelFlagSet, OnPressFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Button<LabelFlagSet, OnPressFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn on_press(
        mut self,
        on_press: impl FnOnce() + Clone + 'static,
    ) -> Button<LabelFlag, OnPressFlagSet>
    where
        OnPressFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .event_handler(move |_: events::Click| (on_press.clone())());
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewOnPressFlag>(
        self,
    ) -> Button<NewLabelFlag, NewOnPressFlag> {
        Button {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
