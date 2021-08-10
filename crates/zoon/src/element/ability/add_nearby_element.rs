use crate::*;

pub trait AddNearbyElement<'a>: UpdateRawEl<RawHtmlEl> + Sized {
    fn element_above(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_above_container().child(element))
        })
    }

    fn element_above_signal(
        self, 
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_above_container().child_signal(element))
        })
    }

    fn element_below(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_below_container().child(element))
        })
    }

    fn element_on_below_signal(
        self, 
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_below_container().child_signal(element))
        })
    }

    fn element_on_left(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_on_left_container().child(element))
        })
    }

    fn element_on_left_signal(
        self, 
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_on_left_container().child_signal(element))
        })
    }

    fn element_on_right(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_on_right_container().child(element))
        })
    }

    fn element_on_right_signal(
        self, 
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.child(element_on_right_container().child_signal(element))
        })
    }
}

fn element_above_container() -> RawHtmlEl {
    RawHtmlEl::new("div")
        .style("position", "absolute")
        .style("bottom", "100%")
        .style("left", "0")
        .style("width", "100%")
        .style("pointer-events", "none")
        .attr("class", "above")
}

fn element_below_container() -> RawHtmlEl {
    RawHtmlEl::new("div")
        .style("position", "absolute")
        .style("top", "100%")
        .style("left", "0")
        .style("width", "100%")
        .style("pointer-events", "none")
        .attr("class", "below")
}

fn element_on_left_container() -> RawHtmlEl {
    RawHtmlEl::new("div")
        .style("position", "absolute")
        .style("right", "100%")
        .style("top", "0")
        .style("height", "100%")
        .style("pointer-events", "none")
        .attr("class", "on_left")
}

fn element_on_right_container() -> RawHtmlEl {
    RawHtmlEl::new("div")
        .style("position", "absolute")
        .style("left", "100%")
        .style("top", "0")
        .style("height", "100%")
        .style("pointer-events", "none")
        .attr("class", "on_right")
}
