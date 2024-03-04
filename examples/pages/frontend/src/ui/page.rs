use crate::*;

pub struct Page;

impl Page {
    pub fn new(page: impl Signal<Item = Option<impl Element>> + Unpin + 'static) -> impl Element {
        Column::new()
            .s(Align::new().center_x())
            .s(Padding::all(20))
            .s(Gap::both(20))
            .item(ui::Header::new())
            .item(El::new().child_signal(page))
    }
}
