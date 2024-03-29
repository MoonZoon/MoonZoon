use crate::*;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Url, Description);

pub struct Image<UrlFlag, DescriptionFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<(UrlFlag, DescriptionFlag)>,
}

impl<RE: RawEl> Element for Image<UrlFlagSet, DescriptionFlagSet, RE> {}

impl Image<UrlFlagNotSet, DescriptionFlagNotSet, RawHtmlEl<web_sys::HtmlImageElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlImageElement>::new("img").class("image"),
            flags: PhantomData,
        }
    }
}

impl<UrlFlag, DescriptionFlag, RE: RawEl> RawElWrapper for Image<UrlFlag, DescriptionFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<UrlFlag, DescriptionFlag, RE: RawEl> Styleable<'_> for Image<UrlFlag, DescriptionFlag, RE> {}
impl<UrlFlag, DescriptionFlag, RE: RawEl> KeyboardEventAware
    for Image<UrlFlag, DescriptionFlag, RE>
{
}
impl<UrlFlag, DescriptionFlag, RE: RawEl> MouseEventAware for Image<UrlFlag, DescriptionFlag, RE> {}
impl<UrlFlag, DescriptionFlag, RE: RawEl> PointerEventAware
    for Image<UrlFlag, DescriptionFlag, RE>
{
}
impl<UrlFlag, DescriptionFlag, RE: RawEl> TouchEventAware for Image<UrlFlag, DescriptionFlag, RE> {}
impl<UrlFlag, DescriptionFlag, RE: RawEl> AddNearbyElement<'_>
    for Image<UrlFlag, DescriptionFlag, RE>
{
}
impl<UrlFlag, DescriptionFlag, RE: RawEl> HasIds for Image<UrlFlag, DescriptionFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, UrlFlag, DescriptionFlag, RE: RawEl> Image<UrlFlag, DescriptionFlag, RE> {
    pub fn url(mut self, url: impl IntoCowStr<'a> + 'a) -> Image<UrlFlagSet, DescriptionFlag, RE>
    where
        UrlFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("src", &url.into_cow_str());
        self.into_type()
    }

    pub fn url_signal(
        mut self,
        url: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Image<UrlFlagSet, DescriptionFlag, RE>
    where
        UrlFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr_signal("src", url);
        self.into_type()
    }

    pub fn description(
        mut self,
        description: impl IntoCowStr<'a> + 'a,
    ) -> Image<UrlFlag, DescriptionFlagSet, RE>
    where
        DescriptionFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("alt", &description.into_cow_str());
        self.into_type()
    }

    pub fn description_signal(
        mut self,
        description: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Image<UrlFlag, DescriptionFlagSet, RE>
    where
        DescriptionFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr_signal("alt", description);
        self.into_type()
    }

    fn into_type<NewUrlFlag, NewDescriptionFlag>(
        self,
    ) -> Image<NewUrlFlag, NewDescriptionFlag, RE> {
        Image {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
