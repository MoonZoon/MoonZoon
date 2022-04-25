use crate::{*, text_input::PlaceholderText};
use std::{marker::PhantomData, iter};

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
    RE: RawEl,
> {
    raw_el: RE,
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
    RawHtmlEl<web_sys::HtmlTextAreaElement>,
>
{
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlTextAreaElement>::new("textarea").class("text_area"),
            flags: PhantomData,
        }
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, ReadOnlyFlag, RE: RawEl + Into<RawElement>> Element
for TextArea<
    IdFlagSet,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlagNotSet,
    ReadOnlyFlag,
    RE,
>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, ReadOnlyFlag, RE: RawEl + Into<RawElement>> Element
for TextArea<
    IdFlagNotSet,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlagSet,
    ReadOnlyFlag,
    RE,
>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<OnChangeFlag, PlaceholderFlag, TextFlag, ReadOnlyFlag, RE: RawEl + Into<RawElement>> Element
for TextArea<
    IdFlagSet,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlagSet,
    ReadOnlyFlag,
    RE,
>
{
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
IntoIterator
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
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

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
UpdateRawEl
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE
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

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
Styleable<'_>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
KeyboardEventAware
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
Focusable
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
where
    RE::DomElement: AsRef<web_sys::HtmlElement>,
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
MouseEventAware
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
PointerEventAware
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
TouchEventAware
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
Hookable
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
AddNearbyElement<'_>
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
HasIds
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
>
{
}
impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag, LabelFlag, ReadOnlyFlag, RE: RawEl>
SelectableTextContent
for TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
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
    ReadOnlyFlag,
    RE: RawEl,
>
TextArea<
    IdFlag,
    OnChangeFlag,
    PlaceholderFlag,
    TextFlag,
    LabelFlag,
    ReadOnlyFlag,
    RE,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlagSet,
        TextFlag,
        LabelFlag,
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

    pub fn text(
        self,
        text: impl IntoCowStr<'a>,
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
        ReadOnlyFlag,
        RE,
    >
        where
            TextFlag: FlagNotSet,
            RE::DomElement: AsRef<web_sys::HtmlTextAreaElement>,
    {
        self.raw_el
            .dom_element()
            .as_ref()
            .set_value(&text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        self,
        text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlagSet,
        LabelFlag,
        ReadOnlyFlag,
        RE,
    >
        where
            TextFlag: FlagNotSet,
            RE::DomElement: AsRef<web_sys::HtmlTextAreaElement>,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlagSet,
        PlaceholderFlag,
        TextFlag,
        LabelFlag,
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
    ) -> TextArea<
        IdFlag,
        OnChangeFlag,
        PlaceholderFlag,
        TextFlag,
        LabelFlagSet,
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
        RE,
    > {
        TextArea {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}



