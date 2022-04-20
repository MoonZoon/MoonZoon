use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//    Element
// ------ ------

make_flags!(Url, Description);

type ImageRawEl = RawHtmlEl<web_sys::HtmlImageElement>;

pub struct Image<UrlFlag, DescriptionFlag> {
    raw_el: ImageRawEl,
    flags: PhantomData<(UrlFlag, DescriptionFlag)>,
}

impl Image<UrlFlagNotSet, DescriptionFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("img").class("image").dom_element_type(),
            flags: PhantomData,
        }
    }
}

impl Element for Image<UrlFlagSet, DescriptionFlagSet> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<UrlFlagSet, DescriptionFlagSet> IntoIterator for Image<UrlFlagSet, DescriptionFlagSet> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<UrlFlag, DescriptionFlag> UpdateRawEl<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(ImageRawEl) -> ImageRawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<UrlFlag, DescriptionFlag> Styleable<'_, ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}
impl<UrlFlag, DescriptionFlag> KeyboardEventAware<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}
impl<UrlFlag, DescriptionFlag> MouseEventAware<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}
impl<UrlFlag, DescriptionFlag> PointerEventAware<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}
impl<UrlFlag, DescriptionFlag> TouchEventAware<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}
impl<UrlFlag, DescriptionFlag> Hookable<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {
}
impl<UrlFlag, DescriptionFlag> AddNearbyElement<'_, ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}
impl<UrlFlag, DescriptionFlag> HasClassId<ImageRawEl> for Image<UrlFlag, DescriptionFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, UrlFlag, DescriptionFlag> Image<UrlFlag, DescriptionFlag> {
    pub fn url(mut self, url: impl IntoCowStr<'a> + 'a) -> Image<UrlFlagSet, DescriptionFlag>
    where
        UrlFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("src", &url.into_cow_str());
        self.into_type()
    }

    pub fn url_signal(
        mut self,
        url: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Image<UrlFlagSet, DescriptionFlag>
    where
        UrlFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr_signal("src", url);
        self.into_type()
    }

    pub fn description(
        mut self,
        description: impl IntoCowStr<'a> + 'a,
    ) -> Image<UrlFlag, DescriptionFlagSet>
    where
        DescriptionFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("alt", &description.into_cow_str());
        self.into_type()
    }

    pub fn description_signal(
        mut self,
        description: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Image<UrlFlag, DescriptionFlagSet>
    where
        DescriptionFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr_signal("alt", description);
        self.into_type()
    }

    fn into_type<NewUrlFlag, NewDescriptionFlag>(self) -> Image<NewUrlFlag, NewDescriptionFlag> {
        Image {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
