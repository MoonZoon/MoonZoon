use zoon::*;

pub fn view() -> impl Element {
    Column::new()
        .s(Gap::new().y(5))
        .item(running_for_text())
        .item(companies_generated_text())
        .item(companies_filtered_text())
}

fn running_for_text() -> impl Element {
    Paragraph::new()
        .content("Running for ")
        .content(El::new().s(Font::new().weight(FontWeight::Bold)).child("123"))
        .content(" ms")
}

fn companies_generated_text() -> impl Element {
    Paragraph::new()
        .content(El::new().s(Font::new().weight(FontWeight::Bold)).child_signal(crate::all_companies().signal_vec_cloned().len()))
        .content(" companies generated in ")
        .content(El::new().s(Font::new().weight(FontWeight::Bold)).child("123"))
        .content(" ms")
}

fn companies_filtered_text() -> impl Element {
    Paragraph::new()
        .content(El::new().s(Font::new().weight(FontWeight::Bold)).child_signal(crate::filtered_companies().signal_vec_cloned().len()))
        .content(" companies filtered in ")
        .content(El::new().s(Font::new().weight(FontWeight::Bold)).child("123"))
        .content(" ms")
}
