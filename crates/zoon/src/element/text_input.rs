use crate::{web_sys::HtmlInputElement, *};
use std::borrow::Cow;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Id, OnChange, Placeholder, Text, Label);

pub struct TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag)>,
}

impl
    TextInput<
        IdFlagNotSet,
        OnChangeFlagNotSet,
        PlaceholderFlagNotSet,
        TextFlagNotSet,
        LabelFlagNotSet,
    >
{
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("input").class("text_input"),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag> Element
    for TextInput<IdFlagSet, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlagNotSet>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag> Element
    for TextInput<IdFlagNotSet, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlagSet>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> UpdateRawEl<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> Styleable<'_, RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> KeyboardEventAware<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> Focusable
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> MouseEventAware<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> Hookable<RawHtmlEl>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
    type WSElement = HtmlInputElement;
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag> AddNearbyElement<'_>
    for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
    TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
{
    pub fn id(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> TextInput<IdFlagSet, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag>
    where
        IdFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("id", &id.into_cow_str());
        self.into_type()
    }

    pub fn placeholder(
        mut self,
        placeholder: Placeholder<'a>,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlagSet, TextFlag, LabelFlag>
    where
        PlaceholderFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .attr("placeholder", &placeholder.text)
            .style_group(placeholder.style_group);
        self.into_type()
    }

    pub fn text(
        mut self,
        text: impl IntoCowStr<'a>,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlagSet, LabelFlag>
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop("value", &text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        mut self,
        text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlagSet, LabelFlag>
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop_signal("value", text);
        self.into_type()
    }

    pub fn on_change(
        mut self,
        on_change: impl FnOnce(String) + Clone + 'static,
    ) -> TextInput<IdFlag, OnChangeFlagSet, PlaceholderFlag, TextFlag, LabelFlag>
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
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlagSet>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("aria-label", &label.into_cow_str());
        self.into_type()
    }

    fn into_type<NewIdFlag, NewOnChangeFlag, NewPlaceholderFlag, NewTextFlag, NewLabelFlag>(
        self,
    ) -> TextInput<NewIdFlag, NewOnChangeFlag, NewPlaceholderFlag, NewTextFlag, NewLabelFlag> {
        TextInput {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}

// ------ Placehholder ------

pub struct Placeholder<'a> {
    text: Cow<'a, str>,
    style_group: StyleGroup<'a>,
}

impl<'a> Placeholder<'a> {
    pub fn new(text: impl IntoCowStr<'a>) -> Self {
        Placeholder {
            text: text.into_cow_str(),
            style_group: StyleGroup::new("::placeholder"),
        }
    }

    pub fn s(mut self, style: impl Style<'a>) -> Self {
        let new_container = style.into_css_props_container();
        let old_container = &mut self.style_group.css_props_container;
        old_container
            .static_css_props
            .extend(new_container.static_css_props.into_iter());
        old_container
            .dynamic_css_props
            .extend(new_container.dynamic_css_props.into_iter());
        old_container
            .task_handles
            .extend(new_container.task_handles.into_iter());
        self
    }
}
