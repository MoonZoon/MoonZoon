use crate::*;
use std::marker::PhantomData;

// ------ ------
//   Element
// ------ ------

make_flags!(Empty, Multiline);

pub struct Stripe<EmptyFlag, MultilineFlag, RE: RawEl> {
    raw_el: RE,
    direction: Mutable<Direction>,
    flags: PhantomData<(EmptyFlag, MultilineFlag)>,
}

impl<MultilineFlag, RE: RawEl> Element for Stripe<EmptyFlagNotSet, MultilineFlag, RE> {}

impl Stripe<EmptyFlagSet, MultilineFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<EmptyFlag, MultilineFlag, RE: RawEl> RawElWrapper for Stripe<EmptyFlag, MultilineFlag, RE> {
    type RawEl = RE;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Stripe<EmptyFlagSet, MultilineFlagNotSet, RawHtmlEl<web_sys::HtmlElement>> {
    #[track_caller]
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                .style_group(
                    StyleGroup::new(".stripe_row > .align_top").style("align-self", "start"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row > .align_bottom").style("align-self", "end"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row > .align_left").style("margin-right", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row > .align_right").style("margin-left", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row > .center_x")
                        .style("margin-left", "auto")
                        .style("margin-right", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row > .center_y").style("align-self", "center"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row > .exact_width").style("flex-shrink", "0"),
                )
                .style_group(StyleGroup::new(".stripe_row > .fill_width").style("flex-grow", "1"))
                .style_group(
                    StyleGroup::new(".stripe_row.align_left_content")
                        .style("justify-content", "left"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row.align_right_content")
                        .style("justify-content", "right"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row.align_top_content")
                        .style_important("align-items", "start"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row.align_bottom_content")
                        .style_important("align-items", "end"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row.center_x_content")
                        .style("justify-content", "center"),
                )
                .style_group(
                    StyleGroup::new(".stripe_row.center_y_content")
                        .style_important("align-items", "center"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .align_top").style("margin-bottom", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .align_bottom").style("margin-top", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .align_left").style("align-self", "start"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .align_right").style("align-self", "end"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .center_x").style("align-self", "center"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .center_y")
                        .style("margin-top", "auto")
                        .style("margin-bottom", "auto"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .exact_height").style("flex-shrink", "0"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column > .fill_height").style("flex-grow", "1"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column.align_left_content")
                        .style("align-items", "start"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column.align_right_content")
                        .style("align-items", "end"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column.align_top_content")
                        .style_important("justify-content", "start"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column.align_bottom_content")
                        .style_important("justify-content", "end"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column.center_x_content")
                        .style("align-items", "center"),
                )
                .style_group(
                    StyleGroup::new(".stripe_column.center_y_content")
                        .style_important("justify-content", "center"),
                );
        });
        let direction = Mutable::new(Direction::default());
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class_signal("stripe_column", direction.signal().eq(Direction::Column))
                .class_signal("stripe_row", direction.signal().eq(Direction::Row))
                .style("display", "inline-flex")
                .style_signal(
                    "align-items",
                    direction.signal().eq(Direction::Row).map_true(|| "center"),
                )
                .style_signal(
                    "flex-direction",
                    direction
                        .signal()
                        .eq(Direction::Column)
                        .map_true(|| "column"),
                )
                .after_remove(clone!((direction) move |_| drop(direction))),
            direction,
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> Styleable<'_> for Stripe<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> KeyboardEventAware
    for Stripe<EmptyFlag, MultilineFlag, RE>
{
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> MouseEventAware for Stripe<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> PointerEventAware
    for Stripe<EmptyFlag, MultilineFlag, RE>
{
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> TouchEventAware for Stripe<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> MutableViewport for Stripe<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> ResizableViewport
    for Stripe<EmptyFlag, MultilineFlag, RE>
{
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> AddNearbyElement<'_>
    for Stripe<EmptyFlag, MultilineFlag, RE>
{
}
impl<EmptyFlag, MultilineFlag, RE: RawEl> HasIds for Stripe<EmptyFlag, MultilineFlag, RE> {}
impl<EmptyFlag, MultilineFlag, RE: RawEl> SelectableTextContent
    for Stripe<EmptyFlag, MultilineFlag, RE>
{
}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, MultilineFlag, RE: RawEl> Stripe<EmptyFlag, MultilineFlag, RE> {
    pub fn item(
        mut self,
        item: impl IntoOptionElement<'a> + 'a,
    ) -> Stripe<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.child(item);
        self.into_type()
    }

    pub fn item_signal(
        mut self,
        item: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Stripe<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.child_signal(item);
        self.into_type()
    }

    pub fn items(
        mut self,
        items: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Stripe<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.children(items);
        self.into_type()
    }

    pub fn items_signal_vec(
        mut self,
        items: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Stripe<EmptyFlagNotSet, MultilineFlag, RE> {
        self.raw_el = self.raw_el.children_signal_vec(items);
        self.into_type()
    }

    pub fn multiline_row(mut self) -> Stripe<EmptyFlag, MultilineFlagSet, RE>
    where
        MultilineFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .style_signal(
                "flex-wrap",
                self.direction
                    .signal()
                    .eq(Direction::Row)
                    .map_true(|| "wrap"),
            )
            .style_signal(
                "flex-basis",
                self.direction.signal().eq(Direction::Row).map_true(|| "0"),
            )
            .style_signal(
                "flex-grow",
                self.direction.signal().eq(Direction::Row).map_true(|| "1"),
            );

        self.into_type()
    }

    pub fn multiline_row_signal(
        mut self,
        multiline: impl Signal<Item = impl Into<Option<bool>>> + 'static,
    ) -> Stripe<EmptyFlag, MultilineFlagSet, RE>
    where
        MultilineFlag: FlagNotSet,
    {
        let multiline = map_ref! {
            let row = self.direction.signal().eq(Direction::Row),
            let multiline = multiline.map(|multiline| multiline.into().unwrap_or_default()) =>
            *row && *multiline
        }
        .broadcast();

        self.raw_el = self
            .raw_el
            .style_signal("flex-wrap", multiline.signal().map_true(|| "wrap"))
            .style_signal("flex-basis", multiline.signal().map_true(|| "0"))
            .style_signal("flex-grow", multiline.signal().map_true(|| "1"));

        self.into_type()
    }

    pub fn direction(self, direction: Direction) -> Stripe<EmptyFlag, MultilineFlag, RE> {
        self.direction.set_neq(direction);
        self.into_type()
    }

    pub fn direction_signal(
        mut self,
        direction: impl Signal<Item = Direction> + Unpin + 'static,
    ) -> Stripe<EmptyFlag, MultilineFlag, RE> {
        let self_direction = self.direction.clone();
        let direction_setter = Task::start_droppable(
            direction.for_each_sync(move |direction| self_direction.set_neq(direction)),
        );
        self.raw_el = self.raw_el.after_remove(move |_| drop(direction_setter));
        self.into_type()
    }

    fn into_type<NewEmptyFlag, NewMultilineFlag>(
        self,
    ) -> Stripe<NewEmptyFlag, NewMultilineFlag, RE> {
        Stripe {
            raw_el: self.raw_el,
            direction: self.direction,
            flags: PhantomData,
        }
    }
}

// ------ Direction ------

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Direction {
    #[default]
    Column,
    Row,
}
