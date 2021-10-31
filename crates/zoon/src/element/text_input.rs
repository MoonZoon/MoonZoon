use crate::{web_sys::HtmlInputElement, *};
use std::{borrow::Cow, marker::PhantomData};
use std::iter;

mod input_type;
pub use input_type::*;

// ------ ------
//    Element
// ------ ------

make_flags!(Id, OnChange, Placeholder, Text, Label, InputType, ReadOnly);

pub struct TextInput<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    InputTypeFlag,
    ReadOnlyFlag,
> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    )>,
}

impl
    TextInput<
        IdFlagNotSet,
        OnChangeFlagNotSet,
        PlaceholderFlagNotSet,
        TextFlagNotSet,
        LabelFlagNotSet,
        InputTypeFlagNotSet,
        ReadOnlyFlagNotSet,
    >
{
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("input").class("text_input"),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, InputTypeFlag, ReadOnlyFlag> Element
    for TextInput<
        IdFlagSet,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagNotSet,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, InputTypeFlag, ReadOnlyFlag> Element
    for TextInput<
        IdFlagNotSet,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagSet,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}


impl<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    InputTypeFlag,
    ReadOnlyFlag,
> IntoIterator for TextInput<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    InputTypeFlag,
    ReadOnlyFlag,
> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    UpdateRawEl<RawHtmlEl>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    Styleable<'_, RawHtmlEl>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    KeyboardEventAware<RawHtmlEl>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    Focusable
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    MouseEventAware<RawHtmlEl>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    Hookable<RawHtmlEl>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
    type WSElement = HtmlInputElement;
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    AddNearbyElement<'_>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag>
    HasClassId<RawHtmlEl>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
}

// ------ ------
//  Attributes
// ------ ------

impl<
        'a,
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
{
    pub fn id(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> TextInput<
        IdFlagSet,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    where
        IdFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("id", &id.into_cow_str());
        self.into_type()
    }

    pub fn placeholder(
        mut self,
        placeholder: Placeholder<'a>,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlagSet,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    where
        PlaceholderFlag: FlagNotSet,
    {
        let mut el_and_group = (self.raw_el, Some(StyleGroup::new("::placeholder")));
        for style_applicator in placeholder.style_applicators {
            el_and_group = style_applicator(el_and_group);
        }
        self.raw_el = el_and_group
            .0
            .attr("placeholder", &placeholder.text)
            .style_group(el_and_group.1.unwrap_throw());
        self.into_type()
    }

    pub fn input_type<T: InputTypeTrait>(
        mut self,
        input_type: T,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlagSet,
        ReadOnlyFlag,
    >
    where
        InputTypeFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("type", T::TYPE);
        self.raw_el = input_type.apply_to_raw_el(self.raw_el);
        self.into_type()
    }

    pub fn text(
        mut self,
        text: impl IntoCowStr<'a>,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop("value", &text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        mut self,
        text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop_signal("value", text);
        self.into_type()
    }

    pub fn read_only(
        mut self,
        read_only: bool,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlagSet,
    >
    where
        ReadOnlyFlag: FlagNotSet,
    {
        if read_only {
            self.raw_el = self.raw_el.attr("readonly", "");
        }
        self.into_type()
    }

    pub fn read_only_signal(
        mut self,
        read_only: impl Signal<Item = bool> + Unpin + 'static,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlagSet,
    >
    where
        ReadOnlyFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .attr_signal("readonly", read_only.map_true(|| ""));
        self.into_type()
    }

    pub fn on_change(
        mut self,
        on_change: impl FnOnce(String) + Clone + 'static,
    ) -> TextInput<
        IdFlag,
        OnChangeFlagSet,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    where
        OnChangeFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.event_handler(move |event: events::Input| {
            #[allow(deprecated)]
            (on_change.clone())(event.value().unwrap())
        });
        self.into_type()
    }

    pub fn label_hidden(
        mut self,
        label: impl IntoCowStr<'a>,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagSet,
        InputTypeFlag,
        ReadOnlyFlag,
    >
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("aria-label", &label.into_cow_str());
        self.into_type()
    }

    fn into_type<
        NewIdFlag,
        NewOnChangeFlag,
        NewPlaceholderFlag,
        NewTextFlag,
        NewLabelFlag,
        NewInputTypeFlag,
        NewReadOnlyFlag,
    >(
        self,
    ) -> TextInput<
        NewIdFlag,
        NewOnChangeFlag,
        NewPlaceholderFlag,
        NewTextFlag,
        NewLabelFlag,
        NewInputTypeFlag,
        NewReadOnlyFlag,
    > {
        TextInput {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}

// ------ Placeholder ------

pub struct Placeholder<'a> {
    text: Cow<'a, str>,
    style_applicators: Vec<
        Box<
            dyn FnOnce((RawHtmlEl, Option<StyleGroup<'a>>)) -> (RawHtmlEl, Option<StyleGroup<'a>>)
                + 'a,
        >,
    >,
}

impl<'a> Placeholder<'a> {
    pub fn new(text: impl IntoCowStr<'a>) -> Self {
        Placeholder {
            text: text.into_cow_str(),
            style_applicators: Vec::new(),
        }
    }

    pub fn s(mut self, style: impl Style<'a> + 'a) -> Self {
        self.style_applicators.push(Box::new(
            |(raw_html_el, style_group): (RawHtmlEl, Option<StyleGroup<'a>>)| {
                style.apply_to_raw_el(raw_html_el, style_group)
            },
        ));
        self
    }
}
