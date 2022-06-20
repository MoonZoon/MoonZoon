use crate::*;
use std::{borrow::Cow, iter, marker::PhantomData};

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

impl<
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl + Into<RawElement>,
    > Element
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

impl<
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl + Into<RawElement>,
    > Element
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

impl<
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl + Into<RawElement>,
    > Element
    for TextInput<
        IdFlagSet,
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

impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > IntoIterator
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

impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > UpdateRawEl
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

impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > Styleable<'_>
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > KeyboardEventAware
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > Focusable
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
where
    RE::DomElement: AsRef<web_sys::HtmlElement>,
{
}
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > MouseEventAware
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > PointerEventAware
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > TouchEventAware
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > Hookable
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > AddNearbyElement<'_>
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > HasIds
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
impl<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        InputTypeFlag,
        ReadOnlyFlag,
        RE: RawEl,
    > SelectableTextContent
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
        self.raw_el = self.raw_el.id(id);
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
        self.raw_el = match placeholder.text {
            PlaceholderText::Static(text) => self.raw_el.attr("placeholder", &text),
            PlaceholderText::Dynamic(text) => self.raw_el.attr_signal("placeholder", text),
        };
        self.raw_el = self.raw_el.style_group(placeholder.style_group);
        self.into_type()
    }

    pub fn input_type(
        mut self,
        input_type: impl Into<InputType>,
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
        let input_type = input_type.into();
        self.raw_el = self.raw_el.attr("type", input_type.dom_type());
        self.raw_el = input_type.apply_to_raw_el(self.raw_el);
        self.into_type()
    }

    pub fn text(
        self,
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
        RE::DomElement: AsRef<web_sys::HtmlInputElement>,
    {
        self.raw_el
            .dom_element()
            .as_ref()
            .set_value(&text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        self,
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
        RE::DomElement: AsRef<web_sys::HtmlInputElement>,
    {
        let text = text.map(|text| text.into_option_cow_str().unwrap_or_default());
        let dom_element = self.raw_el.dom_element();
        let text_setter = Task::start_droppable(
            text.for_each_sync(move |text| dom_element.as_ref().set_value(&text)),
        );
        self.after_remove(move |_| drop(text_setter)).into_type()
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
        mut on_change: impl FnMut(String) + 'static,
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
        RE::DomElement: AsRef<web_sys::HtmlInputElement>,
    {
        let dom_element = self.raw_el.dom_element();
        self.raw_el = self
            .raw_el
            .event_handler(move |_: events::Input| on_change(dom_element.as_ref().value()));
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

    pub fn label_hidden_signal(
        mut self,
        label: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
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
        self.raw_el = self.raw_el.attr_signal("aria-label", label);
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

pub(crate) enum PlaceholderText<'a> {
    Static(Cow<'a, str>),
    Dynamic(Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>),
}

pub struct Placeholder<'a> {
    pub(crate) text: PlaceholderText<'a>,
    pub(crate) style_group: StyleGroup<'a>,
}

impl<'a> Placeholder<'a> {
    pub fn new(text: impl IntoCowStr<'a>) -> Self {
        Placeholder {
            text: PlaceholderText::Static(text.into_cow_str()),
            style_group: StyleGroup::new("::placeholder"),
        }
    }

    pub fn with_signal(
        text: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        let text = text.map(|text| Box::new(text) as Box<dyn IntoOptionCowStr<'static>>);
        Placeholder {
            text: PlaceholderText::Dynamic(Box::new(text)),
            style_group: StyleGroup::new("::placeholder"),
        }
    }

    pub fn s(mut self, style: impl Style<'a> + 'a) -> Self {
        self.style_group = style.merge_with_group(self.style_group);
        self
    }
}
