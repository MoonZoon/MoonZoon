use crate::*;
use web_sys::{EventTarget, Node};

mod raw_html_el;
mod raw_svg_el;

pub use raw_html_el::RawHtmlEl;
pub use raw_svg_el::RawSvgEl;

pub trait UpdateRawEl<T: RawEl> {
    fn update_raw_el(self, updater: impl FnOnce(T) -> T) -> Self;
}

pub trait RawEl: Sized {
    type WSElement: AsRef<Node> + AsRef<EventTarget> + AsRef<JsValue> + AsRef<web_sys::Element>;

    fn update_dom_builder(self, updater: impl FnOnce(DomBuilder<Self::WSElement>) -> DomBuilder<Self::WSElement>) -> Self;

    fn attr(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.attribute(name, value)
        })
    }

    fn attr_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.attribute_signal(
                name.into_cow_str_wrapper(),
                value.map(|value| value.into_option_cow_str_wrapper()),
            )
        })
    }

    fn prop(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.property(name, JsValue::from_str(value))
        })
    }

    fn prop_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.property_signal(
                name.into_cow_str_wrapper(),
                value.map(|value| value.into_option_cow_str_wrapper()),
            )
        })
    }

    fn event_handler<E: StaticEvent>(
        self,
        handler: impl FnOnce(E) + Clone + 'static,
    ) -> Self {
        let handler = move |event: E| handler.clone()(event);
        self.update_dom_builder(|dom_builder| {
            dom_builder.event(handler)
        })
    }

    fn child<'a>(self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        if let Some(child) = child.into_option_element() {
            return self.update_dom_builder(|dom_builder| {
                dom_builder.child(child.into_raw_element().into_dom())
            })
        }
        self
    }

    fn child_signal<'a>(
        self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.child_signal(child.map(|child| {
                child
                    .into_option_element()
                    .map(|element| element.into_raw_element().into_dom())
            }))
        })
    }

    fn children<'a>(
        self,
        children: impl IntoIterator<Item = impl IntoElement<'a> + 'a>,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.children(
                children
                    .into_iter()
                    .map(|child| child.into_element().into_raw_element().into_dom()),
            )
        })
    }

    fn children_signal_vec<'a>(
        self,
        children: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.children_signal_vec(
                children.map(|child| child.into_element().into_raw_element().into_dom()),
            )
        })
    }

    fn style(self, name: &str, value: &str) -> Self;

    fn style_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self;
}
