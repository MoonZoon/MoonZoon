use crate::css_property_name::CssPropertyName;
use crate::*;
use super::class_id_generator;
use std::rc::Rc;

// ------ ------
//   Element
// ------ ------

pub struct RawHtmlEl {
    class_id: Rc<String>,
    dom_builder: DomBuilder<web_sys::HtmlElement>,
}

impl RawHtmlEl {
    pub fn new(tag: &str) -> Self {
        let class_id = Rc::new(class_id_generator().next_class_id());
        Self {
            class_id: Rc::clone(&class_id),
            dom_builder: DomBuilder::new_html(tag)
                .class(class_id.as_str())
                .after_removed(move |_| class_id_generator().remove_class_id(&class_id)),
        }
    }
}

impl From<RawHtmlEl> for RawElement {
    fn from(raw_html_el: RawHtmlEl) -> Self {
        RawElement::El(raw_html_el)
    }
}

impl IntoDom for RawHtmlEl {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawHtmlEl {
    fn into_raw_element(self) -> RawElement {
        RawElement::El(self)
    }
}

// ------ ------
//  Attributes
// ------ ------

impl RawEl for RawHtmlEl {
    type WSElement = web_sys::HtmlElement;

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::WSElement>) -> DomBuilder<Self::WSElement>,
    ) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }

    fn style(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| dom_builder.style(CssPropertyName::new(name), value))
    }

    fn style_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.style_signal(
                name.into_cow_str_wrapper().into_css_property_name(),
                value.map(|value| value.into_option_cow_str_wrapper()),
            )
        })
    }

    fn class_id(&self) -> &str {
        &self.class_id
    }
}

impl RawHtmlEl {
    pub fn focused(mut self) -> Self {
        self.dom_builder = self.dom_builder.focused(true);
        self
    }
}
