use crate::*;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Id, OnChange, Placeholder, Text);

pub struct TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag)>,
}

impl TextInput<IdFlagNotSet, OnChangeFlagNotSet, PlaceholderFlagNotSet, TextFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("input").attr("class", "text_input"),
            flags: PhantomData,
        }
    }
}

impl<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag> Element for TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlag> {
    pub fn id(mut self, id: impl IntoCowStr<'a>) -> TextInput<IdFlagSet, OnChangeFlag, PlaceholderFlag, TextFlag>
    where
        IdFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("id", &id.into_cow_str());
        self.into_type()
    }

    pub fn text(mut self, text: impl IntoCowStr<'a>) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlagSet>
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop("value", &text.into_cow_str());
        self.into_type()
    }

    pub fn text_signal(
        mut self,
        text: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> TextInput<IdFlag, OnChangeFlag, PlaceholderFlag, TextFlagSet>
    where
        TextFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.prop_signal("value", text);
        self.into_type()
    }

    // pub fn on_change(
    //     mut self,
    //     on_press: impl FnOnce() + Clone + 'static,
    // ) -> TextInput<LabelFlag, OnPressFlagSet>
    // where
    //     OnPressFlag: FlagNotSet,
    // {
    //     self.raw_el = self
    //         .raw_el
    //         .event_handler(move |_: events::Click| (on_press.clone())());
    //     self.into_type()
    // }

    fn into_type<NewIdFlag, NewOnChangeFlag, NewPlaceholderFlag, NewTextFlag>(self) -> TextInput<NewIdFlag, NewOnChangeFlag, NewPlaceholderFlag, NewTextFlag> {
        TextInput {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
