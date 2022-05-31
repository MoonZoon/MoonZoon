use zoon::{named_color::*, *};

fn root() -> impl Element {
    El::new()
        .s(Borders::all(Border::new().color(GRAY_0)))
        .s(Width::exact(320))
        .s(Height::exact(320))
        .s(Align::center())
        .child(artboard())
}

fn artboard() -> impl Element {
    #[derive(Clone, Copy)]
    struct ViewBox {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    }

    let (view_box, view_box_signal) = Mutable::new_and_signal(ViewBox {
        x: -100.,
        y: -100.,
        width: 200.,
        height: 200.,
    });
    let (pan, pan_signal) = Mutable::new_and_signal(false);

    RawSvgEl::new("svg")
        .style("touch-action", "none")
        .style_signal("cursor", pan_signal.map_true(|| "grabbing"))
        .attr("width", "100%")
        .attr("height", "100%")
        .attr_signal("viewBox", view_box_signal.map(|view_box| {
            let ViewBox { x, y, width, height } = view_box;
            format!("{x} {y} {width} {height}")
        }))
        .use_dom_element(clone!((view_box) move |this, dom_element| {
            this.event_handler_with_options(EventOptions::new().preventable(), move |event: events_extra::WheelEvent| {
                event.prevent_default();
                let current_view_box = view_box.get();

                let (width, height) = {
                    // Note: It could be replaced with `.on_resize` + `Rc<Cell<width, height>>` 
                    // once ResizeObserver can reliably observe SVG elements (is there a workaround?)
                    let dom_rect = dom_element.get_bounding_client_rect();
                    (dom_rect.width(), dom_rect.height())
                };

                let origin_x = f64::from(event.offset_x());
                let origin_y = f64::from(event.offset_y());
                let zoom = event.delta_y().signum() * 0.2;
                let delta_view_box_width = current_view_box.width * zoom;
                let delta_view_box_height = current_view_box.height * zoom;

                view_box.set(ViewBox {
                    x: current_view_box.x - (delta_view_box_width / width * origin_x),
                    y: current_view_box.y - (delta_view_box_height / height * origin_y),
                    width: current_view_box.width + delta_view_box_width,
                    height: current_view_box.height + delta_view_box_height,
                });
            })
        }))
        .event_handler(clone!((pan) move |_: events_extra::PointerDown| {
            pan.set_neq(true);
        }))
        .event_handler(clone!((pan) move |_: events_extra::PointerUp| pan.set_neq(false)))
        .use_dom_element(clone!((pan) move |this, dom_element| {
            this.event_handler(move |event: events_extra::PointerMove| {
                if not(pan.get()) {
                    return;
                }
                let (width, height) = {
                    let dom_rect = dom_element.get_bounding_client_rect();
                    (dom_rect.width(), dom_rect.height())
                };
                view_box.update_mut(|view_box| {
                    // Warning: `movement_*` fails on iOS / touch screens.
                    // See workaround in the function `on_pointer_move_event`
                    // in the ability `PointerEventAware`. 
                    view_box.x -= f64::from(event.movement_x()) * (view_box.width / width);
                    view_box.y -= f64::from(event.movement_y()) * (view_box.height / height);
                });
            })
        }))
        .use_dom_element(|this, dom_element| {
            let dom_element = dom_element.unchecked_into();
            this.event_handler(move |event: events_extra::PointerLeave| {
                if let Some(target) = event.target() {
                    // we are leaving from the element itself, not only from its child
                    if target == dom_element {
                        pan.set_neq(false);
                    }
                }
            })
        })
        .children(circles())
}

fn circles() -> impl IntoIterator<Item = impl Element> {
    [
        RawSvgEl::new("circle")
            .attr("cx", "-30")
            .attr("cy", "-30")
            .attr("r", "10")
            .attr("fill", "cadetblue"),
        RawSvgEl::new("circle")
            .attr("cx", "30")
            .attr("cy", "30")
            .attr("r", "10")
            .attr("fill", "steelblue"),
        RawSvgEl::new("circle")
            .attr("cx", "30")
            .attr("cy", "-30")
            .attr("r", "10")
            .attr("fill", "lightblue"),
        RawSvgEl::new("circle")
            .attr("cx", "-30")
            .attr("cy", "30")
            .attr("r", "10")
            .attr("fill", "cornflowerblue"),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
