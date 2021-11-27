use zoon::{*, named_color::*};

#[static_ref]
fn drag_rectangle() -> &'static Mutable<bool> {
    Default::default()
}

#[static_ref]
fn rectangle_offset() -> &'static Mutable<(i32, i32)> {
    Default::default()
}

fn root() -> impl Element {
    El::new()
        .s(Width::fill())
        .s(Height::fill())
        .update_raw_el(|raw_el| {
            let class_id = raw_el.class_id();
            raw_el
                .event_handler(|event: events::MouseMove| {
                    if drag_rectangle().get() {
                        rectangle_offset().update(|(x, y)| {
                            (x + event.movement_x(), y + event.movement_y())
                        });
                    }
                })
                .event_handler(|_: events::MouseUp| {
                    drag_rectangle().set_neq(false)
                })
                .event_handler(move |event: events::MouseLeave| {
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
        .s(Width::new(100))
        .s(Height::new(100))
        .s(Background::new().color(GRAY_5))
        .s(Align::center())
        .s(Transform::with_signal(rectangle_offset().signal().map(|(x, y)| {
            Transform::new().move_right(x).move_down(y)
        })))
        .update_raw_el(|raw_el| { 
            raw_el
                .event_handler(|_: events::MouseDown| {
                    drag_rectangle().set_neq(true)
                })
                // https://developer.mozilla.org/en-US/docs/Web/CSS/cursor
                .style_signal("cursor", drag_rectangle().signal().map_bool(
                    || "grabbing", 
                    || "grab",
                ))
        })
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
