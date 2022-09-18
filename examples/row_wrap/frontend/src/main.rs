use zoon::{named_color, *};

const RECTANGLE_SIZE: u32 = 130;

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(10))
        .item(label("Row"))
        .item(row())
        .item(label("Grid"))
        .item(grid())
}

fn label(label: &str) -> impl Element {
    El::new().child(label)
}

fn row() -> impl Element {
    Row::new()
        .s(AlignContent::new().center_x())
        .s(Gap::both(15))
        .multiline()
        .items(rectangles())
}

fn grid() -> impl Element {
    Grid::new()
        .s(AlignContent::new().center_x())
        .s(Gap::both(15))
        .row_wrap_cell_width(RECTANGLE_SIZE)
        .cells(rectangles())
}

fn rectangles() -> impl Iterator<Item = impl Element> {
    (1..=7).map(|index| {
        El::new()
            .s(Width::exact(RECTANGLE_SIZE))
            .s(Height::exact(RECTANGLE_SIZE))
            .s(Background::new().color(named_color::GREEN_9))
            .child(El::new().s(Align::center()).child(index))
    })
}

fn main() {
    start_app("app", root);
}
