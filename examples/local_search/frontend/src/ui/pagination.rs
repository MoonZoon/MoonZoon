use zoon::*;

pub struct Pagination {
    raw_el: RawHtmlEl,
}

impl Element for Pagination {}

impl RawElWrapper for Pagination {
    type RawEl = RawHtmlEl;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl Pagination {
    pub fn new(
        current_page: Mutable<usize>,
        page_count: impl Signal<Item = usize> + 'static + Unpin,
    ) -> Self {
        let page_count = page_count.broadcast();

        let raw_el = Row::new()
            .s(Align::new().center_x())
            .s(Gap::new().x(20))
            .item(page_change_button(
                PageChange::Previous,
                current_page.clone(),
                page_count.signal(),
            ))
            .item(pagination_info(current_page.clone(), page_count.signal()))
            .item(page_change_button(
                PageChange::Next,
                current_page,
                page_count.signal(),
            ))
            .into_raw_el();

        Self { raw_el }
    }
}

#[derive(Clone, Copy)]
enum PageChange {
    Previous,
    Next,
}

fn pagination_info(
    current_page: Mutable<usize>,
    page_count: impl Signal<Item = usize> + 'static + Unpin,
) -> impl Element {
    Paragraph::new()
        .content("Page ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child(Text::with_signal(current_page.signal_ref(|page| page + 1))),
        )
        .content(" of ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child(Text::with_signal(page_count)),
        )
}

fn page_change_button(
    change: PageChange,
    current_page: Mutable<usize>,
    page_count: impl Signal<Item = usize> + 'static + Unpin,
) -> impl Element {
    El::new()
        .s(Visible::with_signal(match change {
            PageChange::Previous => current_page.signal_ref(|page| *page > 0).left_either(),
            PageChange::Next => map_ref! {
                let pages = page_count,
                let current_page = current_page.signal() =>
                current_page + 1 < *pages
            }
            .right_either(),
        }))
        .child(
            Button::new()
                .s(Outline::inner().width(2))
                .s(RoundedCorners::all(3))
                .s(Padding::new().x(10).y(5))
                .s(Font::new().weight(FontWeight::Bold))
                .s(Shadows::new([Shadow::new().x(3).y(3)]))
                .label(match change {
                    PageChange::Previous => "<",
                    PageChange::Next => ">",
                })
                .on_press(move || match change {
                    PageChange::Previous => *current_page.lock_mut() -= 1,
                    PageChange::Next => *current_page.lock_mut() += 1,
                }),
        )
}
