use crate::BACKGROUND_COLOR;
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
            .item(pagination_info(current_page.clone(), page_count.clone()))
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
    page_count: Broadcaster<impl Signal<Item = usize> + 'static + Unpin>,
) -> impl Element {
    Paragraph::new()
        .content("Page ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(map_ref! {
                    let current_page = current_page.signal(),
                    let page_count = page_count.signal() =>
                    (current_page + 1).min(*page_count)
                }),
        )
        .content(" of ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(page_count.signal()),
        )
}

fn page_change_button(
    change: PageChange,
    current_page: Mutable<usize>,
    page_count: impl Signal<Item = usize> + 'static + Unpin,
) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Visible::with_signal(match change {
            PageChange::Previous => current_page.signal_ref(|page| *page > 0).left_either(),
            PageChange::Next => map_ref! {
                let pages = page_count,
                let current_page = current_page.signal() =>
                current_page + 1 < *pages
            }
            .right_either(),
        }))
        .s(Outline::inner().width(2))
        .s(RoundedCorners::all(3))
        .s(Padding::new().x(10).y(5))
        .s(Font::new().weight(FontWeight::Bold))
        .s(Shadows::new([Shadow::new().x(3).y(3)]))
        .s(Background::new().color_signal(hovered_signal.map_bool(
            || BACKGROUND_COLOR.also(|color| *color.lightness.get_or_insert(1.0) += 0.1),
            || BACKGROUND_COLOR,
        )))
        .label(match change {
            PageChange::Previous => "<",
            PageChange::Next => ">",
        })
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || match change {
            PageChange::Previous => *current_page.lock_mut() -= 1,
            PageChange::Next => *current_page.lock_mut() += 1,
        })
}
