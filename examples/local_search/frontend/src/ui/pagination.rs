use zoon::*;

pub fn new(current_page: Mutable<usize>, pages_count: impl Signal<Item = usize> + 'static) -> impl Element {
    let pages_count = pages_count.broadcast();
    Row::new()
        .s(Align::new().center_x())
        .s(Gap::new().x(20))
        .item(prev_page_button(current_page.clone()))
        .item(pagination_info(current_page.clone(), pages_count.signal()))
        .item(next_page_button(current_page, pages_count.signal()))
}

fn pagination_info(current_page: Mutable<usize>, pages_count: impl Signal<Item = usize> + 'static + Unpin) -> impl Element {
    Paragraph::new()
        .content("Page ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child(Text::with_signal(
                    current_page.signal_ref(|page| page + 1),
                )),
        )
        .content(" of ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child(Text::with_signal(pages_count)),
        )
}

fn prev_page_button(current_page: Mutable<usize>) -> impl Element {
    El::new()
        .s(Visible::with_signal(
            current_page.signal_ref(|page| *page > 0),
        ))
        .child(
            Button::new()
                .s(Outline::inner().width(2))
                .s(RoundedCorners::all(3))
                .s(Padding::new().x(10).y(5))
                .s(Font::new().weight(FontWeight::Bold))
                .s(Shadows::new([Shadow::new().x(3).y(3)]))
                .label("<")
                .on_press(move || {
                    current_page.update(|page| page - 1)
                })
        )
}

fn next_page_button(current_page: Mutable<usize>, pages_count: impl Signal<Item = usize> + 'static) -> impl Element {
    let visible = map_ref! {
        let pages = pages_count,
        let current_page = current_page.signal() =>
        current_page + 1 < *pages
    };
    El::new()
        .s(Visible::with_signal(visible))
        .child(
            Button::new()
                .s(Outline::inner().width(2))
                .s(RoundedCorners::all(3))
                .s(Padding::new().x(10).y(5))
                .s(Font::new().weight(FontWeight::Bold))
                .s(Shadows::new([Shadow::new().x(3).y(3)]))
                .label(">")
                .on_press(move || {
                    current_page.update(|page| page + 1)
                })
        )
}
