use crate::*;
use once_cell::sync::Lazy;

static NEARBY_ELEMENTS_Z_INDEX: Lazy<String> =
    Lazy::new(|| LayerIndex::NEARBY_ELEMENTS.to_string());

pub trait AddNearbyElement<'a>: RawElWrapper + Sized {
    #[track_caller]
    fn element_above(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        let container = element_above_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child(element)))
    }

    #[track_caller]
    fn element_above_signal(
        self,
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        let container = element_above_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child_signal(element)))
    }

    #[track_caller]
    fn element_below(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        let container = element_below_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child(element)))
    }

    #[track_caller]
    fn element_below_signal(
        self,
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        let container = element_below_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child_signal(element)))
    }

    #[track_caller]
    fn element_on_left(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        let container = element_on_left_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child(element)))
    }

    #[track_caller]
    fn element_on_left_signal(
        self,
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        let container = element_on_left_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child_signal(element)))
    }

    #[track_caller]
    fn element_on_right(self, element: impl IntoOptionElement<'a> + 'a) -> Self {
        let container = element_on_right_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child(element)))
    }

    #[track_caller]
    fn element_on_right_signal(
        self,
        element: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        let container = element_on_right_container();
        self.update_raw_el(|raw_el| raw_el.child(container.child_signal(element)))
    }
}

#[track_caller]
fn element_above_container() -> RawHtmlEl<web_sys::HtmlElement> {
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".above > *").style("pointer-events", "auto"));
    });
    RawHtmlEl::new("div")
        .class("nearby_element_container")
        .class("above")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("position", "absolute")
        .style("bottom", "100%")
        .style("left", "0")
        .style("width", "100%")
        .style("pointer-events", "none")
        .style("z-index", &NEARBY_ELEMENTS_Z_INDEX)
}

#[track_caller]
fn element_below_container() -> RawHtmlEl<web_sys::HtmlElement> {
    run_once!(|| {
        global_styles().style_group(StyleGroup::new(".below > *").style("pointer-events", "auto"));
    });
    RawHtmlEl::new("div")
        .class("nearby_element_container")
        .class("below")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("position", "absolute")
        .style("top", "100%")
        .style("left", "0")
        .style("width", "100%")
        .style("pointer-events", "none")
        .style("z-index", &NEARBY_ELEMENTS_Z_INDEX)
}

#[track_caller]
fn element_on_left_container() -> RawHtmlEl<web_sys::HtmlElement> {
    run_once!(|| {
        global_styles()
            .style_group(StyleGroup::new(".on_left > *").style("pointer-events", "auto"));
    });
    RawHtmlEl::new("div")
        .class("nearby_element_container")
        .class("on_left")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("position", "absolute")
        .style("right", "100%")
        .style("top", "0")
        .style("height", "100%")
        .style("pointer-events", "none")
        .style("z-index", &NEARBY_ELEMENTS_Z_INDEX)
}

#[track_caller]
fn element_on_right_container() -> RawHtmlEl<web_sys::HtmlElement> {
    run_once!(|| {
        global_styles()
            .style_group(StyleGroup::new(".on_right > *").style("pointer-events", "auto"));
    });
    RawHtmlEl::new("div")
        .class("nearby_element_container")
        .class("on_right")
        .style("display", "flex")
        .style("flex-direction", "column")
        .style("position", "absolute")
        .style("left", "100%")
        .style("top", "0")
        .style("height", "100%")
        .style("pointer-events", "none")
        .style("z-index", &NEARBY_ELEMENTS_Z_INDEX)
}
