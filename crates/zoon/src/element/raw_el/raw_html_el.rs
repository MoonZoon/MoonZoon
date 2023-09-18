use super::CLASS_ID_GENERATOR;
use crate::*;

// ------ ------
//   Element
// ------ ------

pub struct RawHtmlEl<DomElement: Into<web_sys::HtmlElement> = web_sys::HtmlElement> {
    inner: Option<ElInner<DomElement>>,
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> ElementUnchecked
    for RawHtmlEl<DomElement>
{
    fn into_raw_unchecked(self) -> RawElOrText {
        self.dom_element_type::<web_sys::HtmlElement>().into()
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> Element for RawHtmlEl<DomElement> {}

struct ElInner<DomElement> {
    class_id: ClassId,
    dom_builder: DomBuilder<DomElement>,
}

impl RawHtmlEl<web_sys::HtmlElement> {
    #[track_caller]
    pub fn new(tag: &str) -> Self {
        <Self as RawEl>::new(tag)
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> RawHtmlEl<DomElement> {
    pub fn dom_element_type<T: Into<web_sys::HtmlElement> + JsCast>(self) -> RawHtmlEl<T> {
        let inner = self.inner.unwrap_throw();
        let element = inner.dom_builder.__internal_element().unchecked_into::<T>();
        let dom_builder = DomBuilder::new(element).__internal_transfer_callbacks(inner.dom_builder);
        RawHtmlEl {
            inner: Some(ElInner {
                class_id: inner.class_id,
                dom_builder,
            }),
        }
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Clone + JsCast> From<RawHtmlEl<DomElement>>
    for RawElOrText
{
    #[track_caller]
    fn from(raw_html_el: RawHtmlEl<DomElement>) -> Self {
        RawElOrText::RawHtmlEl(raw_html_el.dom_element_type::<web_sys::HtmlElement>())
    }
}

impl<DomElement: Into<web_sys::HtmlElement> + Into<web_sys::Node>> IntoDom
    for RawHtmlEl<DomElement>
{
    fn into_dom(self) -> Dom {
        self.inner.unwrap_throw().dom_builder.into_dom()
    }
}

// ------ ------
//     RawEl
// ------ ------

impl<DomElement> RawEl for RawHtmlEl<DomElement>
// Warning: "Global" bounds with `JsValue` or `JsCast` or `AsRef<web_sys::HtmlElement>` break Rust Analyzer (?)
where
    DomElement: AsRef<web_sys::Node>
        + AsRef<web_sys::EventTarget>
        + Into<web_sys::EventTarget>
        + AsRef<web_sys::Element>
        + Into<web_sys::Element>
        + Into<web_sys::HtmlElement>
        + JsCast
        + Clone
        + 'static,
{
    type DomElement = DomElement;

    #[track_caller]
    fn new(tag: &str) -> Self
    where
        DomElement: JsCast,
    {
        let class_id = CLASS_ID_GENERATOR.next_class_id();

        let mut dom_builder = DomBuilder::new_html(tag);
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            inner: Some(ElInner {
                class_id: class_id.clone(),
                dom_builder: dom_builder
                    .after_removed(move |_| CLASS_ID_GENERATOR.remove_class_id(class_id)),
            }),
        }
        .source_code_location()
    }

    fn new_dummy() -> Self {
        Self { inner: None }
    }

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::DomElement>) -> DomBuilder<Self::DomElement>,
    ) -> Self {
        let mut inner = self.inner.unwrap_throw();
        inner.dom_builder = updater(inner.dom_builder);
        self.inner = Some(inner);
        self
    }

    fn dom_element(&self) -> Self::DomElement {
        self.inner
            .as_ref()
            .unwrap_throw()
            .dom_builder
            .__internal_element()
    }

    fn class_id(&self) -> ClassId {
        self.inner.as_ref().unwrap_throw().class_id.clone()
    }

    #[track_caller]
    fn from_dom_element(dom_element: Self::DomElement) -> Self {
        let mut dom_builder = DomBuilder::new(dom_element);

        let class_id = CLASS_ID_GENERATOR.next_class_id();
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            inner: Some(ElInner {
                class_id: class_id.clone(),
                dom_builder: dom_builder
                    .after_removed(move |_| CLASS_ID_GENERATOR.remove_class_id(class_id)),
            }),
        }
        .source_code_location()
    }

    fn focus(self) -> Self
    where
        Self::DomElement: AsRef<web_sys::HtmlElement>,
    {
        self.update_dom_builder(|dom_builder| dom_builder.focused(true))
    }

    fn focus_signal(self, focus: impl Signal<Item = bool> + Unpin + 'static) -> Self
    where
        Self::DomElement: AsRef<web_sys::HtmlElement>,
    {
        self.update_dom_builder(|dom_builder| dom_builder.focused_signal(focus))
    }
}
