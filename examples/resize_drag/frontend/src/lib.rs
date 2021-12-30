use zoon::{*, named_color::*};

#[static_ref]
fn drag_rectangle() -> &'static Mutable<bool> {
    Default::default()
}

#[static_ref]
fn drag_handle() -> &'static Mutable<bool> {
    Default::default()
}

#[static_ref]
fn rectangle_offset() -> &'static Mutable<(i32, i32)> {
    Default::default()
}

#[static_ref]
fn rectangle_size() -> &'static Mutable<(u32, u32)> {
    Mutable::new((150, 150))
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .on_pointer_up(|| {
            drag_rectangle().set_neq(false);
            drag_handle().set_neq(false);
        })
        .on_pointer_move_event(|event| {
            if drag_rectangle().get() {
                rectangle_offset().update(|(x, y)| {
                    (x + event.movement_x(), y + event.movement_y())
                });
            }
            else if drag_handle().get() {
                rectangle_size().update(|(width, height)| {
                    let new_width = width as i32 + event.movement_x();
                    let new_height = height as i32 + event.movement_y();
                    (u32::try_from(new_width).unwrap_or_default(), u32::try_from(new_height).unwrap_or_default())
                });
            }
        })
        .update_raw_el(|raw_el| {
            let class_id = raw_el.class_id();
            raw_el
                .event_handler(move |event: events_extra::PointerLeave| {
                    let target = event.dyn_target::<web_sys::Element>().unwrap_throw();
                    let classes = target.get_attribute("class").unwrap_throw();
                    class_id.map(|class_id| {
                        let class_id = class_id.unwrap_throw();
                        for class in classes.split_ascii_whitespace() {
                            if class == class_id {
                                drag_rectangle().set_neq(false);
                                return;
                            }
                        }
                    });
                })
        })
        .child(rectangle())
}

fn rectangle() -> impl Element {
    Stack::new()
        .s(Width::with_signal(rectangle_size().signal().map(|(width, _)| width).dedupe()))
        .s(Height::with_signal(rectangle_size().signal().map(|(_, height)| height).dedupe()))
        .s(Background::new().color_signal(drag_rectangle().signal().map_bool(
            || GREEN_9,
            || GRAY_5, 
        )))
        .s(Transform::with_signal(rectangle_offset().signal().map(|(x, y)| {
            Transform::new().move_right(x).move_down(y)
        })))
        .s(Cursor::with_signal(drag_rectangle().signal().map_bool(
            || CursorIcon::Grabbing, 
            || CursorIcon::Grab, 
        )))
        .update_raw_el(|raw_el| { 
            raw_el.style("touch-action", "none")
        })
        .layer(rectangle_content())
        .layer(handle())
}

fn rectangle_content() -> impl Element {
    El::new()
        .s(Clip::both())
        .on_pointer_down(|| drag_rectangle().set_neq(true))
        .child(rectangle_attributes())
}

fn rectangle_attributes() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Font::new().color(GRAY_2))
        .update_raw_el(|raw_el| {
            raw_el
                .style("user-select", "none")
        })
        .item(
            Row::new()
                .item("offset X: ")
                .item_signal(rectangle_offset().signal().map(|(x, _)| x).dedupe())
        )
        .item(
            Row::new()
                .item("offset Y: ")
                .item_signal(rectangle_offset().signal().map(|(_, y)| y).dedupe())
        )
        .item(
            Row::new()
                .item("width: ")
                .item_signal(rectangle_size().signal().map(|(width, _)| width).dedupe())
        )
        .item(
            Row::new()
                .item("height: ")
                .item_signal(rectangle_size().signal().map(|(_, height)| height).dedupe())
        )
}

fn handle() -> impl Element {
    El::new()
        .s(Width::new(40))
        .s(Height::new(40))
        .s(Align::new().bottom().right())
        .s(Background::new().color_signal(drag_handle().signal().map_bool(
            || YELLOW_4, 
            || GREEN_5,
        )))
        .s(Transform::new().move_down(10).move_right(10))
        .s(RoundedCorners::all_max())
        .s(Cursor::new(CursorIcon::UpLeftDownRightArrow))
        .on_pointer_down(|| drag_handle().set_neq(true))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
