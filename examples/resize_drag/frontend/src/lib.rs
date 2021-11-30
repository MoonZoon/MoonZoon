use zoon::{*, named_color::*};

#[static_ref]
fn drag_rectangle() -> &'static Mutable<bool> {
    Default::default()
}

#[static_ref]
fn rectangle_offset() -> &'static Mutable<(i32, i32)> {
    Default::default()
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
                    event.prevent_default();
                    if drag_rectangle().get() {
                        let current_x = event.x(); 
                        let current_y = event.y(); 
                        let (previous_x, previous_y) = pointer_position().replace((current_x, current_y));
                        rectangle_offset().update(|(x, y)| {
                            (x + (current_x - previous_x), y + (current_y - previous_y))
                        });
                    }
                })
                .event_handler(|event: events_extra::PointerUp| {
                    event.prevent_default();
                    drag_rectangle().set_neq(false)
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
    El::new()
        .s(Width::new(150))
        .s(Height::new(150))
        .s(Background::new().color_signal(drag_rectangle().signal().map_bool(
            || GREEN_9,
            || GRAY_5, 
        )))
        .s(Align::center())
        .s(Transform::with_signal(rectangle_offset().signal().map(|(x, y)| {
            Transform::new().move_right(x).move_down(y)
        })))
        .update_raw_el(|raw_el| { 
            raw_el
                .event_handler(|event: events_extra::PointerDown| {
                    event.prevent_default();
                    pointer_position().set_neq((event.x(), event.y()));
                    drag_rectangle().set_neq(true)
                })
                // https://developer.mozilla.org/en-US/docs/Web/CSS/cursor
                .style_signal("cursor", drag_rectangle().signal().map_bool(
                    || "grabbing", 
                    || "grab",
                ))
                .style("touch-action", "none")
        })
        .child(
            Column::new()
                .s(Align::center())
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
        )
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
