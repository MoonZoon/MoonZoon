use std::iter;
use zoon::*;

pub struct Tile {
    raw_el: RawHtmlEl<web_sys::HtmlElement>,
}

#[allow(dead_code)]
impl Tile {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("bx-tile"),
        }
    }

    pub fn child<'a>(mut self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        self.raw_el = self.raw_el.child(child);
        self
    }

    pub fn child_signal<'a>(
        mut self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.child_signal(child);
        self
    }

    pub fn children<'a>(
        mut self,
        children: impl IntoIterator<Item = impl IntoOptionElement<'a> + 'a>,
    ) -> Self {
        self.raw_el = self.raw_el.children(children);
        self
    }

    pub fn children_signal_vec<'a>(
        mut self,
        children: impl SignalVec<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.raw_el = self.raw_el.children_signal_vec(children);
        self
    }
}

impl Element for Tile {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into_raw_element()
    }
}

impl IntoIterator for Tile {
    type Item = Self;
    type IntoIter = iter::Once<Self>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        iter::once(self)
    }
}
