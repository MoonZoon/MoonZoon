use std::iter;
use zoon::*;

make_event!(BxSearchInput, "bx-search-input" => web_sys::CustomEvent);

pub struct Search {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
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

impl Element for Search {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into_raw_element()
    }
}

impl IntoIterator for Search {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
