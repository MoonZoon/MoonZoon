use crate::{web_sys::HtmlInputElement, *};
use std::iter;
use std::{borrow::Cow, marker::PhantomData};

// ------ ------
//    Element
// ------ ------

make_flags!(Id, OnChange, Placeholder, Text, Label, ReadOnly);

pub struct TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
        ReadOnlyFlag,
    )>,
}

impl
TextArea<
    IdFlagNotSet,
    OnChangeFlagNotSet,
    PlaceholderFlagNotSet,
    TextFlagNotSet,
    LabelFlagNotSet,
    ReadOnlyFlagNotSet,
>
{
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("textarea").class("text_area"),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, ReadOnlyFlag> Element
for TextArea<
    IdFlagSet,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlagNotSet,
    ReadOnlyFlag,
>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, ReadOnlyFlag> Element
for TextArea<
    IdFlagNotSet,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlagSet,
    ReadOnlyFlag,
>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
IntoIterator
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
UpdateRawEl<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
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

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
Styleable<'_, RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
KeyboardEventAware<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
Focusable
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
MouseEventAware<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
PointerEventAware<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
TouchEventAware<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
Hookable<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
    type WSElement = HtmlInputElement;
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
AddNearbyElement<'_>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
HasClassId<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag>
SelectableTextContent<RawHtmlEl>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
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
    ReadOnlyFlag,
>
TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
>
{
    pub fn id(
        mut self,
        id: impl IntoCowStr<'a>,
    ) -> TextArea<
        IdFlagSet,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlagSet,
        TextFlag,
        LabelFlag,
        ReadOnlyFlag,
    >
        where
            PlaceholderFlag: FlagNotSet,
    {
        let mut el_and_group = (self.raw_el, Some(StyleGroup::new("::placeholder")));
        for style_applicator in placeholder.style_applicators {
            el_and_group = style_applicator(el_and_group);
        }
        let (raw_el, style_group) = el_and_group;
        match placeholder.text {
            text_input::PlaceholderText::Static(text) => {
                self.raw_el = raw_el.attr("placeholder", &text);
            }
            text_input::PlaceholderText::Dynamic(text) => {
                self.raw_el = raw_el.attr_signal("placeholder", text);
            }
        }
        self.raw_el = self.raw_el.style_group(style_group.unwrap_throw());
        self.into_type()
    }

    pub fn text(
        mut self,
        text: impl IntoCowStr<'a>,
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlagSet,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagSet,
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
        NewReadOnlyFlag,
    >(
        self,
    ) -> TextArea<
        NewIdFlag,
        NewOnChangeFlag,
        NewPlaceholderFlag,
        NewTextFlag,
        NewLabelFlag,
        NewReadOnlyFlag,
    > {
        TextArea {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}



