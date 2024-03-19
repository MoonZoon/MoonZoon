use zoon::{
    strum::{AsRefStr, EnumIter, IntoEnumIterator},
    *,
};

#[derive(Clone, Copy, EnumIter, AsRefStr)]
#[strum(crate = "strum")]
enum RectangleAlignment {
    TopLeft,
    Top,
    TopRight,
    Right,
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    Center,
}

impl RectangleAlignment {
    fn to_align(&self) -> Align {
        match self {
            Self::TopLeft => Align::new().top().left(),
            Self::Top => Align::new().top().center_x(),
            Self::TopRight => Align::new().top().right(),
            Self::Right => Align::new().right().center_y(),
            Self::BottomRight => Align::new().bottom().right(),
            Self::Bottom => Align::new().bottom().center_x(),
            Self::BottomLeft => Align::new().bottom().left(),
            Self::Left => Align::new().left().center_y(),
            Self::Center => Align::center(),
        }
    }
}

static RECTANGLE_ALIGNMENT: Lazy<Mutable<Option<RectangleAlignment>>> = lazy::default();

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(AlignContent::center())
        .s(Align::center())
        .s(Gap::new().y(15))
        .item(
            Row::new()
                .item(container("Column", Column::new().items(rectangles())))
                .item(container("El", El::new().child(rectangle(1))))
                .item(container("Grid", Grid::new().cells(rectangles()))),
        )
        .item(container(
            "Stack",
            Stack::new()
                .layers(rectangles())
                .layers(RectangleAlignment::iter().map(align_switcher)),
        ))
        .item(
            Row::new()
                .item(container("Row", Row::new().items(rectangles())))
                .item(container(
                    "Paragraph",
                    Paragraph::new().contents(rectangles()),
                )),
        )
}

fn container<'a, T: Styleable<'a> + Element>(name: &str, element: T) -> impl Element {
    Column::new()
        .item(El::new().s(Align::new().center_x()).child(name))
        .item(
            element
                .s(Width::exact(278))
                .s(Height::exact(200))
                .s(Borders::all(Border::new().color(color!("Gray")).width(3)))
                .s(RoundedCorners::all(15)),
        )
}

fn rectangles() -> impl IntoIterator<Item = impl Element> {
    (1..=2).map(rectangle)
}

fn rectangle(index: i32) -> impl Element {
    let size = 40;
    El::new()
        .s(Width::exact(size))
        .s(Height::exact(size))
        .s(Background::new().color(color!("Green")))
        .s(RoundedCorners::all(10))
        .s(Align::with_signal_self(RECTANGLE_ALIGNMENT.signal_ref(
            |alignment| alignment.map(|alignment| alignment.to_align()),
        )))
        .child(El::new().s(Align::center()).child(index))
}

fn align_switcher(rectangle_alignment: RectangleAlignment) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(rectangle_alignment.to_align())
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("Blue"), || color!("DarkBlue"))))
        .s(Padding::all(5))
        .s(RoundedCorners::all(10))
        .label(rectangle_alignment.as_ref())
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(move || RECTANGLE_ALIGNMENT.set(Some(rectangle_alignment)))
}
