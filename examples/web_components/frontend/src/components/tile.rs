use zoon::*;

pub struct Tile {
    raw_el: RawHtmlEl,
}

impl Element for Tile {}

impl RawElWrapper for Tile {
    type RawEl = RawHtmlEl;
    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
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
