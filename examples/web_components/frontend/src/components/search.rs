use zoon::*;

make_event!(BxSearchInput, "bx-search-input" => web_sys::CustomEvent);

pub struct Search {
    raw_el: RawHtmlEl,
}

impl Element for Search {}

impl RawElWrapper for Search {
    type RawEl = RawHtmlEl;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

#[allow(dead_code)]
impl Search {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("bx-search"),
        }
    }

    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.raw_el = self.raw_el.prop("placeholder", placeholder);
        self
    }

    pub fn value_signal(
        mut self,
        value: impl Signal<Item = impl IntoCowStr<'static>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.prop_signal("value", value);
        self
    }

    pub fn on_change(mut self, mut on_change: impl FnMut(String) + 'static) -> Self {
        self.raw_el = self.raw_el.event_handler(move |event: BxSearchInput| {
            let value = Reflect::get(&event.event.detail(), &"value".into())
                .unwrap_throw()
                .as_string()
                .unwrap_throw();
            on_change(value);
        });
        self
    }
}
