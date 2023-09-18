use zoon::*;

trait_set! {
    pub trait PageCountSignal = Signal<Item = usize> + 'static + Unpin
}

pub struct Pagination<PCS: PageCountSignal> {
    current_page: Mutable<usize>,
    page_count: Broadcaster<PCS>,
}

impl<PCS: PageCountSignal> Element for Pagination<PCS> {
    fn into_raw_element(self) -> RawElement {
        self.view().into_raw_element()
    }
}

impl<PCS: PageCountSignal> Pagination<PCS> {
    #[track_caller]
    pub fn new(current_page: Mutable<usize>, page_count: PCS) -> Self {
        Self {
            current_page,
            page_count: page_count.broadcast(),
        }
    }

    fn view(&self) -> impl Element {
        Row::new()
            .s(Align::new().center_x())
            .s(Gap::new().x(20))
            .item(self.page_change_button(PageChange::Previous))
            .item(self.pagination_info())
            .item(self.page_change_button(PageChange::Next))
    }

    fn pagination_info(&self) -> impl Element {
        Paragraph::new()
            .content("Page ")
            .content(
                El::new()
                    .s(Font::new().weight(FontWeight::Bold))
                    .child(Text::with_signal(
                        self.current_page.signal_ref(|page| page + 1),
                    )),
            )
            .content(" of ")
            .content(
                El::new()
                    .s(Font::new().weight(FontWeight::Bold))
                    .child(Text::with_signal(self.page_count.signal())),
            )
    }

    fn page_change_button(&self, change: PageChange) -> impl Element {
        El::new()
            .s(Visible::with_signal(match change {
                PageChange::Previous => {
                    self.current_page.signal_ref(|page| *page > 0).left_either()
                }
                PageChange::Next => map_ref! {
                    let pages = self.page_count.signal(),
                    let current_page = self.current_page.signal() =>
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
                    .on_press(clone!((self.current_page => page) move || {
                        match change {
                            PageChange::Previous => *page.lock_mut() -= 1,
                            PageChange::Next => *page.lock_mut() += 1,
                        }
                    })),
            )
    }
}

#[derive(Clone, Copy)]
enum PageChange {
    Previous,
    Next,
}
