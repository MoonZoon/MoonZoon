use crate::*;
use std::{iter, borrow::Cow, marker::PhantomData};

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
    RE: RawEl,
> {
    raw_el: RE,
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
        RawHtmlEl<web_sys::HtmlInputElement>,
    >
{
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlInputElement>::new("input").class("text_input"),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl + Into<RawElement>> Element
    for TextInput<
        IdFlagSet,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagNotSet,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl + Into<RawElement>> Element
    for TextInput<
        IdFlagNotSet,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagSet,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    IntoIterator
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    UpdateRawEl
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
    type RawEl = RE;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    Styleable<'_>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    KeyboardEventAware
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    Focusable
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
    where RE::DomElement: AsRef<web_sys::HtmlElement>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    MouseEventAware
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    PointerEventAware
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    TouchEventAware
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    Hookable
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    AddNearbyElement<'_>
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    HasClassId
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag, ReadOnlyFlag, RE: RawEl>
    SelectableTextContent
    for TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
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
        RE: RawEl,
    >
    TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
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
        RE,
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
        RE,
    >
    where
        PlaceholderFlag: FlagNotSet,
    {
        let mut el_and_group = (self.raw_el.dom_element_type(), Some(StyleGroup::new("::placeholder")));
        for style_applicator in placeholder.style_applicators {
            el_and_group = style_applicator(el_and_group);
        }
        let (raw_el, style_group) = el_and_group;
        let raw_el = raw_el.dom_element_type();
        self.raw_el = match placeholder.text {
            PlaceholderText::Static(text) => {
                raw_el.attr("placeholder", &text)
            }
            PlaceholderText::Dynamic(text) => {
                raw_el.attr_signal("placeholder", text)
            }
        };
        self.raw_el = self.raw_el.style_group(style_group.unwrap_throw());
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
        RE,
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
        RE,
    >
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop("value", &text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        mut self,
        text: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> TextInput<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE,
    >
    where
        TextFlag: FlagNotSet,
    {
        let text = text.map(|text| text.into_option_cow_str().unwrap_or_default());
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
        RE,
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
        RE,
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
        RE,
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
        RE,
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
        RE,
    > {
        TextInput {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}

// ------ Placeholder ------

enum PlaceholderText<'a> {
    Static(Cow<'a, str>),
    Dynamic(Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>),
}

pub struct Placeholder<'a> {
    text: PlaceholderText<'a>,
    style_applicators: Vec<
        Box<
            dyn FnOnce((RawHtmlEl<web_sys::HtmlElement>, Option<StyleGroup<'a>>)) -> (RawHtmlEl<web_sys::HtmlElement>, Option<StyleGroup<'a>>)
                + 'a,
        >,
    >,
}

impl<'a> Placeholder<'a> {
    pub fn new(text: impl IntoCowStr<'a>) -> Self {
        Placeholder {
            text: PlaceholderText::Static(text.into_cow_str()),
            style_applicators: Vec::new(),
        }
    }

    pub fn with_signal(
        text: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        let text = text.map(|text| Box::new(text) as Box<dyn IntoOptionCowStr<'static>>);
        Placeholder {
            text: PlaceholderText::Dynamic(Box::new(text)),
            style_applicators: Vec::new(),
        }
    }

    pub fn s(mut self, style: impl Style<'a> + 'a) -> Self {
        self.style_applicators.push(Box::new(
            |(raw_html_el, style_group): (RawHtmlEl<web_sys::HtmlElement>, Option<StyleGroup<'a>>)| {
                style.apply_to_raw_el(raw_html_el, style_group)
            },
        ));
        self
    }
}
