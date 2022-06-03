use zoon::{named_color::*, *};

fn root() -> impl Element {
    Column::new()
        .s(Padding::all(20))
        .s(Width::fill().max(500))
        .s(Align::center())
        .s(Spacing::new(30))
        .item(title())
        .item(paragraph())
}

fn title() -> impl Element {
    Paragraph::with_tag(Tag::H1)
        .lang(Lang::Norwegian)
        .s(Font::new().size(28).center())
        .content(
            El::new()
                .s(Font::new().no_wrap())
                .child("The no-wrap sentence."),
        )
        .content(" I'm centered and ")
        .content(
            El::new()
                .s(Font::new().line(FontLine::new().underline().wavy()))
                .s(RoundedCorners::all(10))
                .s(Background::new().color(BLUE_9))
                .s(Padding::all(5).top(0))
                .child("wavy"),
        )
        .content(".")
}

fn paragraph() -> impl Element {
    Paragraph::new()
        .content(
            El::new()
                .s(Font::new().size(60).line_height(55))
                .s(Align::new().left())
                .s(Padding::new().right(10))
                .child("L")
        )
        .content(
            El::new()
                .s(Font::new().italic())
                .child("orem ipsum dolor sit amet, consectetuer adipiscing elit. Fusce wisi. Duis bibendum, lectus ut viverra rhoncus, dolor nunc faucibus libero, eget facilisis enim ipsum id lacus. Sed convallis magna eu sem. Morbi leo mi, nonummy eget tristique non, rhoncus non leo. Donec iaculis gravida nulla. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. ")
            )
        .content(
            Link::new()
                .s(Font::new().color(BLUE_4).line(FontLine::new().underline()))
                .label("Extra mega super big large looooooooong label link to the page example.com")
                .to("http://example.com")
        )
        .content(" Maecenas libero. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos hymenaeos. Nunc dapibus tortor vel mi dapibus sollicitudin.")
        .content(
            Image::new()
                .s(Align::new().right())
                .s(RoundedCorners::all(20))
                .s(Padding::all(15))
                .url([PUBLIC_URL, "lorem_picsum_1039-200x200.jpg"].concat())
                .description("paragraph image")
        )
        .content(" Aliquam in lorem sit amet leo accumsan lacinia. Pellentesque sapien. Donec ipsum massa, ullamcorper in, auctor et, scelerisque sed, est. Integer tempor. Aliquam erat volutpat. Phasellus enim erat, vestibulum vel, aliquam a, posuere eu, velit. Aliquam erat volutpat. Etiam commodo dui eget wisi.")
}

fn main() {
    start_app("app", root);
}
