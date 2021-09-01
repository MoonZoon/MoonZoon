use crate::{web_sys::HtmlInputElement, *};
use std::borrow::Cow;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Id, OnChange, Placeholder, Text, Label, InputType);

pub struct TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag)>,
}

impl
    TextInput<
        IdFlagNotSet,
        OnChangeFlagNotSet,
        PlaceholderFlagNotSet,
        TextFlagNotSet,
        LabelFlagNotSet,
        InputTypeFlagNotSet,
    >
{
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("input").class("text_input"),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, InputTypeFlag> Element
    for TextInput<IdFlagSet, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlagNotSet, InputTypeFlag>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, InputTypeFlag> Element
    for TextInput<IdFlagNotSet, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlagSet, InputTypeFlag>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag> UpdateRawEl<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag>
{
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> Styleable<'_, RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> KeyboardEventAware<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> Focusable
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> MouseEventAware<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> Hookable<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
    type WSElement = HtmlInputElement;
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> AddNearbyElement<'_>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag> HasClassId<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag,InputTypeFlag>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag>
    TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag>
{
    pub fn id(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> TextInput<IdFlagSet, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag>
    where
        IdFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("id", &id.into_cow_str());
        self.into_type()
    }

    pub fn placeholder(
        mut self,
        placeholder: Placeholder<'a>,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlagSet, TextFlag, LabelFlag, InputTypeFlag>
    where
        PlaceholderFlag: FlagNotSet,
    {
        let mut el_and_group  = (self.raw_el, StyleGroup::new("::placeholder"));
        for style_applicator in placeholder.style_applicators {
            el_and_group = style_applicator(el_and_group);
        }
        self.raw_el = el_and_group.0
            .attr("placeholder", &placeholder.text)
            .style_group(el_and_group.1);
        self.into_type()
    }

    pub fn input_type(
        mut self,
        input_type: InputType,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlagSet>
    where
        InputTypeFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .attr("type", input_type.type_);
        self.into_type()
    }

    pub fn text(
        mut self,
        text: impl IntoCowStr<'a>,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlagSet, LabelFlag, InputTypeFlag>
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop("value", &text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        mut self,
        text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlagSet, LabelFlag, InputTypeFlag>
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop_signal("value", text);
        self.into_type()
    }

    pub fn on_change(
        mut self,
        on_change: impl FnOnce(String) + Clone + 'static,
    ) -> TextInput<IdFlag, OnChangeFlagSet, PlaceholderFlag, TextFlag, LabelFlag, InputTypeFlag>
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
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlagSet, InputTypeFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("aria-label", &label.into_cow_str());
        self.into_type()
    }

    fn into_type<NewIdFlag, NewOnChangeFlag, NewPlaceholderFlag, NewTextFlag, NewLabelFlag, NewInputTypeFlag>(
        self,
    ) -> TextInput<NewIdFlag, NewOnChangeFlag, NewPlaceholderFlag, NewTextFlag, NewLabelFlag, NewInputTypeFlag> {
        TextInput {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}

// ------ Placehholder ------

pub struct Placeholder<'a> {
    text: Cow<'a, str>,
    style_applicators: Vec<Box<dyn FnOnce((RawHtmlEl, StyleGroup<'a>)) -> (RawHtmlEl, StyleGroup<'a>) + 'a>>,
}

impl<'a> Placeholder<'a> {
    pub fn new(text: impl IntoCowStr<'a>) -> Self {
        Placeholder {
            text: text.into_cow_str(),
            style_applicators: Vec::new(),
        }
    }

    pub fn s(mut self, style: impl Style<'a> + 'a) -> Self {
        self.style_applicators.push(Box::new(|(raw_html_el, style_group): (RawHtmlEl, StyleGroup<'a>)| {
            style.apply_to_style_group(raw_html_el, style_group)
        }));
        self
    }
}

// ------ InputType ------

pub struct InputType {
    type_: &'static str,
}

impl Default for InputType {
    fn default() -> Self {
        Self {
            type_: "text",
        }
    }
}

impl InputType {
    pub fn password() -> Self {
        let mut this = Self::default();
        this.type_ = "password";
        this
    }
}
