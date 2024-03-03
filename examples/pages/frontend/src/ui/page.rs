use crate::*;

pub struct Page;

impl Page {
    pub fn new(
        page: impl Signal<Item = impl IntoOptionElement<'static>> + Unpin + 'static,
    ) -> impl Element {
        Column::new()
            .s(Padding::all(20))
            .s(Gap::both(20))
            .item(ui::Header::new())
            .item(El::new().child_signal(page))
    }
}
