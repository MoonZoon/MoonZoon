use zoon::*;

const RECTANGLE_SIZE: u32 = 130;

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::new().y(10))
        .item(
            El::new()
                .s(Font::new().color(color!("Gray")))
                .child("Resize the browser window"),
        )
        .item(El::new().child("Row"))
        .item(row())
        .item(El::new().child("Grid"))
        .item(grid())
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
            .s(Background::new().color(color!("Green")))
            .child(El::new().s(Align::center()).child(index))
    })
}
