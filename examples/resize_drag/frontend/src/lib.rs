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

#[static_ref]
fn pointer_position() -> &'static Mutable<(i32, i32)> {
    Default::default()
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .update_raw_el(|raw_el| {
            let class_id = raw_el.class_id();
            raw_el
                .event_handler(|event: events_extra::PointerMove| {
                    event.prevent_default();  // ?
                    if drag_rectangle().get() {
                        let current_x = event.x(); 
                        let current_y = event.y(); 
                        let (previous_x, previous_y) = pointer_position().replace((current_x, current_y));
                        rectangle_offset().update(|(x, y)| {
                            (x + (current_x - previous_x), y + (current_y - previous_y))
                        });
                    }
                    else if drag_handle().get() {
                        let current_x = event.x(); 
                        let current_y = event.y(); 
                        let (previous_x, previous_y) = pointer_position().replace((current_x, current_y));
                        rectangle_size().update(|(width, height)| {
                            let new_width = width as i32 + (current_x - previous_x);
                            let new_height = height as i32 + (current_y - previous_y);
                            (u32::try_from(new_width).unwrap_or_default(), u32::try_from(new_height).unwrap_or_default())
                        });
                    }
                })
                .event_handler(|event: events_extra::PointerUp| {
                    event.prevent_default();
                    drag_rectangle().set_neq(false);
                    drag_handle().set_neq(false);
                })
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
            raw_el
                .event_handler(|event: events_extra::PointerDown| {
                        event.prevent_default();
                        pointer_position().set_neq((event.x(), event.y()));
                        drag_rectangle().set_neq(true)
                })
                .style("touch-action", "none")
        })
        .layer(rectangle_attributes())
        .layer(handle())
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
            Paragraph::new()
                .content("offset X: ")
                .content_signal(rectangle_offset().signal().map(|(x, _)| x).dedupe())
        )
        .item(
            Paragraph::new()
                .content("offset Y: ")
                .content_signal(rectangle_offset().signal().map(|(_, y)| y).dedupe())
        )
        .item(
            Paragraph::new()
                .content("width: ")
                .content_signal(rectangle_size().signal().map(|(width, _)| width).dedupe())
        )
        .item(
            Paragraph::new()
                .content("height: ")
                .content_signal(rectangle_size().signal().map(|(_, height)| height).dedupe())
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
        .update_raw_el(|raw_el| { 
            raw_el
                .event_handler(|event: events_extra::PointerDown| {
                    event.prevent_default();
                    pointer_position().set_neq((event.x(), event.y()));
                    drag_handle().set_neq(true);
                    if drag_rectangle().get() {
                        drag_rectangle().set_neq(false);
                    }
                })
                // .style("touch-action", "none") 
        })
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
