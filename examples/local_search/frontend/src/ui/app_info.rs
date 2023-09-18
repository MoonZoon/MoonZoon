use zoon::*;

pub struct AppInfo {
    raw_el: RawHtmlEl,
}

impl Element for AppInfo {}

impl RawElWrapper for AppInfo {
    type RawEl = RawHtmlEl;

    fn raw_el_mut(&mut self) -> &mut Self::RawEl {
        &mut self.raw_el
    }
}

impl AppInfo {
    pub fn new() -> Self {
        let raw_el = Column::new()
            .s(Gap::new().y(5))
            .item(running_for_text())
            .item(companies_generated_text())
            .item(companies_filtered_text())
            .into_raw_el();
        Self { raw_el }
    }
}

fn running_for_text() -> impl Element {
    Paragraph::new()
        .content("Running for ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child("123"),
        )
        .content(" ms")
}

fn companies_generated_text() -> impl Element {
    Paragraph::new()
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(crate::all_companies().signal_vec_cloned().len()),
        )
        .content(" companies generated in ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child("123"),
        )
        .content(" ms")
}

fn companies_filtered_text() -> impl Element {
    Paragraph::new()
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child_signal(crate::filtered_companies().signal_vec_cloned().len()),
        )
        .content(" companies filtered in ")
        .content(
            El::new()
                .s(Font::new().weight(FontWeight::Bold))
                .child("123"),
        )
        .content(" ms")
}
