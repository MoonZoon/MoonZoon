use crate::*;
use std::marker::PhantomData;

// ------ ------
//    Element
// ------ ------

make_flags!(Label, ForInput);

pub struct Label<LabelFlag, ForInputFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<(LabelFlag, ForInputFlag)>,
}

impl<ForInputFlag, RE: RawEl> Element for Label<LabelFlagSet, ForInputFlag, RE> {}

impl Label<LabelFlagNotSet, ForInputFlagNotSet, RawHtmlEl<web_sys::HtmlLabelElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::<web_sys::HtmlLabelElement>::new("label").class("label"),
            flags: PhantomData,
        }
    }
}

impl<LabelFlag, ForInputFlag, RE: RawEl> RawElWrapper for Label<LabelFlag, ForInputFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, ForInputFlag, RE: RawEl> Styleable<'_> for Label<LabelFlag, ForInputFlag, RE> {}
impl<LabelFlag, ForInputFlag, RE: RawEl> KeyboardEventAware for Label<LabelFlag, ForInputFlag, RE> {}
impl<LabelFlag, ForInputFlag, RE: RawEl> MouseEventAware for Label<LabelFlag, ForInputFlag, RE> {}
impl<LabelFlag, ForInputFlag, RE: RawEl> PointerEventAware for Label<LabelFlag, ForInputFlag, RE> {}
impl<LabelFlag, ForInputFlag, RE: RawEl> TouchEventAware for Label<LabelFlag, ForInputFlag, RE> {}
impl<LabelFlag, ForInputFlag, RE: RawEl> AddNearbyElement<'_>
    for Label<LabelFlag, ForInputFlag, RE>
{
}
impl<LabelFlag, ForInputFlag, RE: RawEl> HasIds for Label<LabelFlag, ForInputFlag, RE> {}
impl<LabelFlag, ForInputFlag, RE: RawEl> SelectableTextContent
    for Label<LabelFlag, ForInputFlag, RE>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, ForInputFlag, RE: RawEl> Label<LabelFlag, ForInputFlag, RE> {
    pub fn label(
        mut self,
        label: impl IntoElement<'a> + 'a,
    ) -> Label<LabelFlagSet, ForInputFlag, RE>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Label<LabelFlagSet, ForInputFlag, RE>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn for_input(mut self, id: impl IntoCowStr<'a>) -> Label<LabelFlag, ForInputFlagSet, RE>
    where
        ForInputFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.attr("for", &id.into_cow_str());
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewForInputFlag>(self) -> Label<NewLabelFlag, NewForInputFlag, RE> {
        Label {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
