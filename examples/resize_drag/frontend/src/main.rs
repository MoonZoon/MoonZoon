use zoon::*;

static DRAG_RECTANGLE: Lazy<Mutable<bool>> = lazy::default();
static DRAG_HANDLE: Lazy<Mutable<bool>> = lazy::default();
static RECTANGLE_OFFSET: Lazy<Mutable<(i32, i32)>> = lazy::default();
static RECTANGLE_SIZE: Lazy<Mutable<(u32, u32)>> = Lazy::new(|| (150, 150).into());

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .on_pointer_up(|| {
            DRAG_RECTANGLE.set_neq(false);
            DRAG_HANDLE.set_neq(false);
        })
        .on_pointer_move_event(|event| {
            if DRAG_RECTANGLE.get() {
                RECTANGLE_OFFSET.update(|(x, y)| (x + event.movement_x(), y + event.movement_y()));
            } else if DRAG_HANDLE.get() {
                RECTANGLE_SIZE.update(|(width, height)| {
                    let new_width = width as i32 + event.movement_x();
                    let new_height = height as i32 + event.movement_y();
                    (
                        u32::try_from(new_width).unwrap_or_default(),
                        u32::try_from(new_height).unwrap_or_default(),
                    )
                });
            }
        })
        .on_pointer_leave(|| DRAG_RECTANGLE.set_neq(false))
        .child(rectangle())
}

fn rectangle() -> impl Element {
    Stack::new()
        .s(Width::exact_signal(
            RECTANGLE_SIZE.signal().map(|(width, _)| width).dedupe(),
        ))
        .s(Height::exact_signal(
            RECTANGLE_SIZE.signal().map(|(_, height)| height).dedupe(),
        ))
        .s(Background::new().color_signal(
            DRAG_RECTANGLE
                .signal()
                .map_bool(|| color!("Green"), || color!("Gray")),
        ))
        .s(Transform::with_signal(
            RECTANGLE_OFFSET
                .signal()
                .map(|(x, y)| Transform::new().move_right(x).move_down(y)),
        ))
        .s(Cursor::with_signal(
            DRAG_RECTANGLE
                .signal()
                .map_bool(|| CursorIcon::Grabbing, || CursorIcon::Grab),
        ))
        .touch_native_handling(TouchHandling::none())
        .layer(rectangle_content())
        .layer(handle())
}

fn rectangle_content() -> impl Element {
    El::new()
        .s(Clip::both())
        .on_pointer_down(|| DRAG_RECTANGLE.set_neq(true))
        .child(rectangle_attributes())
}

fn rectangle_attributes() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Font::new().color(color!("White")))
        .text_content_selecting(TextContentSelecting::none())
        .item(
            Row::new()
                .item("offset X: ")
                .item_signal(RECTANGLE_OFFSET.signal().map(|(x, _)| x).dedupe()),
        )
        .item(
            Row::new()
                .item("offset Y: ")
                .item_signal(RECTANGLE_OFFSET.signal().map(|(_, y)| y).dedupe()),
        )
        .item(
            Row::new()
                .item("width: ")
                .item_signal(RECTANGLE_SIZE.signal().map(|(width, _)| width).dedupe()),
        )
        .item(
            Row::new()
                .item("height: ")
                .item_signal(RECTANGLE_SIZE.signal().map(|(_, height)| height).dedupe()),
        )
}

fn handle() -> impl Element {
    El::new()
        .s(Width::exact(40))
        .s(Height::exact(40))
        .s(Align::new().bottom().right())
        .s(Background::new().color_signal(
            DRAG_HANDLE
                .signal()
                .map_bool(|| color!("Green"), || color!("Yellow")),
        ))
        .s(Transform::new().move_down(10).move_right(10))
        .s(RoundedCorners::all_max())
        .s(Cursor::new(CursorIcon::UpLeftDownRightArrow))
        .on_pointer_down(|| DRAG_HANDLE.set_neq(true))
}

fn main() {
    start_app("app", root);
}
