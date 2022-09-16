use crate::*;
use std::{iter, marker::PhantomData};

// ------ ------
//   Element
// ------ ------

make_flags!(Empty);

pub struct Grid<EmptyFlag, RE: RawEl> {
    raw_el: RE,
    flags: PhantomData<EmptyFlag>,
}

impl Grid<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    pub fn new() -> Self {
        Self::with_tag(Tag::Custom("div"))
    }
}

impl<RE: RawEl + Into<RawElement>> Element for Grid<EmptyFlagNotSet, RE> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<EmptyFlag, RE: RawEl> IntoIterator for Grid<EmptyFlag, RE> {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}

impl<EmptyFlag, RE: RawEl> UpdateRawEl for Grid<EmptyFlag, RE> {
    type RawEl = RE;

    fn update_raw_el(mut self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}

// ------ ------
//   Abilities
// ------ ------

impl ChoosableTag for Grid<EmptyFlagSet, RawHtmlEl<web_sys::HtmlElement>> {
    fn with_tag(tag: Tag) -> Self {
        run_once!(|| {
            global_styles()
                // @TODO check all (copy-pasted from `Row`)
                .style_group(
                    StyleGroup::new(".grid > .center_x")
                        .style("margin-left", "auto")
                        .style("margin-right", "auto"),
                )
                .style_group(
                    StyleGroup::new(".grid > .align_top").style("align-self", "flex-start"),
                )
                .style_group(
                    StyleGroup::new(".grid > .align_bottom").style("align-self", "flex-end"),
                )
                .style_group(StyleGroup::new(".grid > .align_right").style("margin-left", "auto"))
                .style_group(StyleGroup::new(".grid > .exact_width").style("flex-shrink", "0"))
                .style_group(StyleGroup::new(".grid > .fill_width").style("flex-grow", "1"))
                .style_group(
                    StyleGroup::new(".grid.align_left_content").style("justify-content", "left"),
                )
                .style_group(
                    StyleGroup::new(".grid.align_right_content").style("justify-content", "right"),
                )
                .style_group(
                    StyleGroup::new(".grid.align_top_content")
                        .style_important("align-items", "start"),
                )
                .style_group(
                    StyleGroup::new(".grid.align_bottom_content")
                        .style_important("align-items", "end"),
                )
                .style_group(
                    StyleGroup::new(".grid.center_x_content").style("justify-content", "center"),
                )
                .style_group(
                    StyleGroup::new(".grid.center_y_content")
                        .style_important("align-items", "center"),
                );
        });
        Self {
            raw_el: RawHtmlEl::new(tag.as_str())
                .class("grid")
                .style("display", "inline-grid")
                .style("align-items", "center"),
            flags: PhantomData,
        }
    }
}
impl<EmptyFlag, RE: RawEl> Styleable<'_> for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> KeyboardEventAware for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MouseEventAware for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> PointerEventAware for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> TouchEventAware for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> MutableViewport for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> ResizableViewport for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> Hookable for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> AddNearbyElement<'_> for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> HasIds for Grid<EmptyFlag, RE> {}
impl<EmptyFlag, RE: RawEl> SelectableTextContent for Grid<EmptyFlag, RE> {}

// ------ ------
//  Attributes
// ------ ------

impl<'a, EmptyFlag, RE: RawEl> Grid<EmptyFlag, RE> {
    pub fn row_wrap_cell_width(mut self, cell_width: U32Width) -> Grid<EmptyFlag, RE> {
        self.raw_el = self.raw_el.style(
            "grid-template-columns",
            &crate::format!("repeat(auto-fill, {cell_width}px)"),
        );
        self
    }

    pub fn row_wrap_cell_width_signal(
        mut self,
        cell_width: impl Signal<Item = impl Into<Option<U32Width>>> + Unpin + 'static,
    ) -> Grid<EmptyFlag, RE> {
        let cell_width = cell_width.map(|cell_width| {
            cell_width
                .into()
                .map(|cell_width| crate::format!("repeat(auto-fill, {cell_width}px)"))
        });
        self.raw_el = self
            .raw_el
            .style_signal("grid-template-columns", cell_width);
        self
    }

    pub fn cell(mut self, cell: impl IntoOptionElement<'a> + 'a) -> Grid<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child(cell);
        self.into_type()
    }

    pub fn cell_signal(
        mut self,
        cell: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Grid<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.child_signal(cell);
        self.into_type()
    }

    pub fn cells(
        mut self,
        cells: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Grid<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children(cells);
        self.into_type()
    }

    pub fn cells_signal_vec(
        mut self,
        cells: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Grid<EmptyFlagNotSet, RE> {
        self.raw_el = self.raw_el.children_signal_vec(cells);
        self.into_type()
    }

    fn into_type<NewEmptyFlag>(self) -> Grid<NewEmptyFlag, RE> {
        Grid {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
