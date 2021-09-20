use super::class_id_generator;
use crate::*;

// ------ ------
//   Element
// ------ ------

pub struct RawSvgEl {
    class_id: ClassId,
    dom_builder: DomBuilder<web_sys::SvgElement>,
}

impl RawSvgEl {
    pub fn new(tag: &str) -> Self {
        let class_id = class_id_generator().next_class_id();

        let mut dom_builder = DomBuilder::new_svg(tag);
        dom_builder = class_id.map(move |class_id| dom_builder.class(class_id.unwrap_throw()));

        Self {
            class_id: class_id.clone(),
            dom_builder: dom_builder
                .after_removed(move |_| class_id_generator().remove_class_id(class_id)),
        }
    }
}

impl From<RawSvgEl> for RawElement {
    fn from(raw_svg_el: RawSvgEl) -> Self {
        RawElement::SvgEl(raw_svg_el)
    }
}

impl IntoDom for RawSvgEl {
    fn into_dom(self) -> Dom {
        self.dom_builder.into_dom()
    }
}

impl Element for RawSvgEl {
    fn into_raw_element(self) -> RawElement {
        RawElement::SvgEl(self)
    }
}

// ------ ------
//  Attributes
// ------ ------

impl RawEl for RawSvgEl {
    type WSElement = web_sys::SvgElement;

    fn update_dom_builder(
        mut self,
        updater: impl FnOnce(DomBuilder<Self::WSElement>) -> DomBuilder<Self::WSElement>,
    ) -> Self {
        self.dom_builder = updater(self.dom_builder);
        self
    }

    fn style(self, _name: &str, _value: &str) -> Self {
        self.update_dom_builder(|_dom_builder| {
            todo!("implement `style` body in `raw_el.rs` once it's implemented for Element in Dominator or write `DomBuilderExt");
        })
    }

    fn style_signal<'a>(
        self,
        _name: impl IntoCowStr<'static>,
        _value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|_dom_builder| {
            todo!("implement `style_signal` body in `raw_el.rs` once it's implemented for Element in Dominator or write `DomBuilderExt");
        })
    }

    fn class_id(&self) -> ClassId {
        self.class_id.clone()
    }
}
