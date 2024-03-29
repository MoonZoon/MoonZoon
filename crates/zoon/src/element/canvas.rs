use crate::*;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Width, Height);

pub struct Canvas<WidthFlag, HeightFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<(WidthFlag, HeightFlag)>,
}

impl<HeightFlag, RE: RawEl> Element for Canvas<WidthFlagSet, HeightFlag, RE> {}

impl Canvas<WidthFlagNotSet, HeightFlagNotSet, RawHtmlEl<web_sys::HtmlCanvasElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlCanvasElement>::new("canvas").class("canvas"),
            flags: PhantomData,
        }
    }
}

impl<WidthFlag, HeightFlag, RE: RawEl> RawElWrapper for Canvas<WidthFlag, HeightFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<WidthFlag, HeightFlag, RE: RawEl> Styleable<'_> for Canvas<WidthFlag, HeightFlag, RE> {}
impl<WidthFlag, HeightFlag, RE: RawEl> KeyboardEventAware for Canvas<WidthFlag, HeightFlag, RE> {}
impl<WidthFlag, HeightFlag, RE: RawEl> MouseEventAware for Canvas<WidthFlag, HeightFlag, RE> {}
impl<WidthFlag, HeightFlag, RE: RawEl> PointerEventAware for Canvas<WidthFlag, HeightFlag, RE> {}
impl<WidthFlag, HeightFlag, RE: RawEl> TouchEventAware for Canvas<WidthFlag, HeightFlag, RE> {}
impl<WidthFlag, HeightFlag, RE: RawEl> AddNearbyElement<'_> for Canvas<WidthFlag, HeightFlag, RE> {}
impl<WidthFlag, HeightFlag, RE: RawEl> HasIds for Canvas<WidthFlag, HeightFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, WidthFlag, HeightFlag, RE: RawEl> Canvas<WidthFlag, HeightFlag, RE> {
    /// Default: 300px
    pub fn width(mut self, width: u32) -> Canvas<WidthFlagSet, HeightFlag, RE>
    where
        WidthFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("width", &width.to_string());
        self.into_type()
    }

    /// Default: 150px
    pub fn height(mut self, height: u32) -> Canvas<WidthFlag, HeightFlagSet, RE>
    where
        HeightFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("height", &height.to_string());
        self.into_type()
    }

    fn into_type<NewWidthFlag, NewHeightFlag>(self) -> Canvas<NewWidthFlag, NewHeightFlag, RE> {
        Canvas {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
