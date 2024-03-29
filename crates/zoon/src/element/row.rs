use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty, Multiline);

pub struct Row<EmptyFlag, MultilineFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<(EmptyFlag, MultilineFlag)>,
}

impl<MultilineFlag, RE: RawEl> Element for Row<EmptyFlagNotSet, MultilineFlag, RE> {}

impl Row<EmptyFlagSet, MultilineFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<EmptyFlag, MultilineFlag, RE: RawEl> RawElWrapper for Row<EmptyFlag, MultilineFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Row<EmptyFlagSet, MultilineFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(StyleGroup::new(".row > .align_top").style("align-self", "start"))
                .style_group(StyleGroup::new(".row > .align_bottom").style("align-self", "end"))
                .style_group(StyleGroup::new(".row > .align_left").style("margin-right", "auto"))
                .style_group(StyleGroup::new(".row > .align_right").style("margin-left", "auto"))
                .style_group(
                    StyleGroup::new(".row > .center_x")
                        .style("margin-left", "auto")
                        .style("margin-right", "auto"),
                )
                .style_group(StyleGroup::new(".row > .center_y").style("align-self", "center"))
                .style_group(StyleGroup::new(".row > .exact_width").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".row > .fill_width").style("flex-grow", "1"))
                .style_group(
                    StyleGroup::new(".row.align_left_content").style("justify-content", "left"),
                )
                .style_group(
                    StyleGroup::new(".row.align_right_content").style("justify-content", "right"),
                )
                .style_group(
                    StyleGroup::new(".row.align_top_content")
                        .style_important("align-items", "start"),
                )
                .style_group(
                    StyleGroup::new(".row.align_bottom_content")
                        .style_important("align-items", "end"),
                )
                .style_group(
                    StyleGroup::new(".row.center_x_content").style("justify-content", "center"),
                )
                .style_group(
                    StyleGroup::new(".row.center_y_content")
                        .style_important("align-items", "center"),
                );
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("row")
                .style("display", "inline-flex")
                .style("align-items", "center"),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> Styleable<'_> for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> KeyboardEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> MouseEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> PointerEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> TouchEventAware for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> MutableViewport for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> ResizableViewport for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> AddNearbyElement<'_>
    for Row<EmptyFlag, MultilineFlag, RE>
{
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> HasIds for Row<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> SelectableTextContent
    for Row<EmptyFlag, MultilineFlag, RE>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, MultilineFlag, RE: RawEl> Row<EmptyFlag, MultilineFlag, RE> {
    pub fn item(
        mut self,
        item: impl IntoOptionElement<'a> + 'a,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Row<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    pub fn multiline(mut self) -> Row<EmptyFlag, MultilineFlagSet, RE>
    where
        MultilineFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .style("flex-wrap", "wrap")
            .style("flex-basis", "0")
            .style("flex-grow", "1");

        self.into_type()
    }

    pub fn multiline_signal(
        mut self,
        multiline: impl Signal<Item = impl Into<Option<bool>>> + 'static,
    ) -> Row<EmptyFlag, MultilineFlagSet, RE>
    where
        MultilineFlag: FlagNotSet,
    {
        let multiline = multiline
            .map(|multiline| multiline.into().unwrap_or_default())
            .broadcast();

        self.raw_el = self
            .raw_el
            .style_signal("flex-wrap", multiline.signal().map_true(|| "wrap"))
            .style_signal("flex-basis", multiline.signal().map_true(|| "0"))
            .style_signal("flex-grow", multiline.signal().map_true(|| "1"));

        self.into_type()
    }

    fn into_type<NewEmptyFlag, NewMultilineFlag>(self) -> Row<NewEmptyFlag, NewMultilineFlag, RE> {
        Row {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
