use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//    Element
// ------ ------

make_flags!(Width, Height);

pub struct Canvas<WidthFlag, HeightFlag, CanvasRawEl = RawHtmlEl<web_sys::HtmlCanvasElement>> {
    raw_el: CanvasRawEl,
    flags: PhantomData<(WidthFlag, HeightFlag)>,
}

impl Canvas<WidthFlagNotSet, HeightFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("canvas").class("canvas").dom_element_type(),
            flags: PhantomData,
        }
    }
}

impl<HeightFlag> Element for Canvas<WidthFlagSet, HeightFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<WidthFlag, HeightFlag> IntoIterator for Canvas<WidthFlag, HeightFlag> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<WidthFlag, HeightFlag, CanvasRawEl: RawEl> UpdateRawEl for Canvas<WidthFlag, HeightFlag, CanvasRawEl> {
    type RawEl = CanvasRawEl;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<WidthFlag, HeightFlag> Styleable<'_> for Canvas<WidthFlag, HeightFlag> {}
impl<WidthFlag, HeightFlag> KeyboardEventAware for Canvas<WidthFlag, HeightFlag> {}
impl<WidthFlag, HeightFlag> MouseEventAware for Canvas<WidthFlag, HeightFlag> {}
impl<WidthFlag, HeightFlag> PointerEventAware for Canvas<WidthFlag, HeightFlag> {}
impl<WidthFlag, HeightFlag> TouchEventAware for Canvas<WidthFlag, HeightFlag> {}
impl<WidthFlag, HeightFlag> Hookable for Canvas<WidthFlag, HeightFlag> {
}
impl<WidthFlag, HeightFlag> AddNearbyElement<'_> for Canvas<WidthFlag, HeightFlag> {}
impl<WidthFlag, HeightFlag> HasClassId for Canvas<WidthFlag, HeightFlag> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, WidthFlag, HeightFlag> Canvas<WidthFlag, HeightFlag> {
    /// Default: 300px
    pub fn width(mut self, width: u32) -> Canvas<WidthFlagSet, HeightFlag>
    where
        WidthFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("width", &width.to_string());
        self.into_type()
    }

    /// Default: 150px
    pub fn height(mut self, height: u32) -> Canvas<WidthFlag, HeightFlagSet>
    where
        HeightFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("height", &height.to_string());
        self.into_type()
    }

    fn into_type<NewWidthFlag, NewHeightFlag>(self) -> Canvas<NewWidthFlag, NewHeightFlag> {
        Canvas {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
