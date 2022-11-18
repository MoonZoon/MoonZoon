use zoon::{*, named_color::*};

#[static_ref]
fn drop_zone_active() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn component_said() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

fn load_and_use_component(file_list: web_sys::FileList) {
    let imports = js_sys::Object::new();

    let file = file_list.get(0).unwrap_throw();
    let readable_stream = file.stream();

    let mut response_init = web_sys::ResponseInit::new();
    let headers = web_sys::Headers::new().unwrap_throw();
    headers.append("Content-Type", "application/wasm").unwrap_throw();
    response_init.headers(&JsValue::from(headers));
    let response = web_sys::Response::new_with_opt_readable_stream_and_init(Some(&readable_stream), &response_init).unwrap_throw();
    let response_promise =  future_to_promise(future::ok(JsValue::from(response)));

    let result_object = JsFuture::from(js_sys::WebAssembly::instantiate_streaming(&response_promise, &imports));

    Task::start(async move {
        let result_object = result_object.await.unwrap_throw();
        let instance: js_sys::WebAssembly::Instance = Reflect::get(&result_object, &JsValue::from("instance")).unwrap_throw().dyn_into().unwrap_throw();
        let exports = instance.exports();

        let say_something = Reflect::get(exports.as_ref(), &"say_something".into())
            .unwrap_throw()
            .dyn_into::<js_sys::Function>()
            .expect("the function `say_something` not found in the Wasm module");

        // let said = say_something.
    })
}

fn root() -> impl Element {
    Column::new()
    .s(Width::exact(300))
        .s(Align::center())
        .s(Gap::new().y(20))
        .item(drop_zone())
        .item_signal(
            component_said().signal_cloned().map_some(|text| {
                Paragraph::new()
                    .s(Align::new().center_x())
                    .content("Component said: ")
                    .content(El::new().s(Font::new().weight(FontWeight::SemiBold)).child(text))
            })
        )
}

fn drop_zone() -> impl Element {
    El::new()
        .s(Height::exact(200))
        .s(RoundedCorners::all(30))
        .s(Borders::all(Border::new().color(GREEN_5).width(2)))
        .s(Background::new().color_signal(drop_zone_active().signal().map_true(|| GREEN_9)))
        // @TODO refactor to an ability
        .update_raw_el(|raw_el| {
            raw_el
                .event_handler_with_options(EventOptions::new().preventable(), |event: events::DragEnter | {
                    event.stop_propagation();
                    event.prevent_default();
                    drop_zone_active().set_neq(true);
                })
                .event_handler_with_options(EventOptions::new().preventable(), |event: events::DragOver| {
                    event.stop_propagation();
                    event.prevent_default();
                    event.data_transfer().unwrap_throw().set_drop_effect("copy");
                })
                .event_handler_with_options(EventOptions::new().preventable(), |event: events::DragLeave| {
                    event.stop_propagation();
                    event.prevent_default();
                    drop_zone_active().set_neq(false);
                })
                .event_handler_with_options(EventOptions::new().preventable(), |event: events::Drop| {
                    event.stop_propagation();
                    event.prevent_default();
                    drop_zone_active().set_neq(false);
                    let file_list = event.data_transfer().unwrap_throw().files().unwrap_throw();
                    load_and_use_component(file_list);
                })
        })
        .child(
            El::new()
                .s(Align::center())
                // @TODO the new ability shouldn't fire `dragleave` on moving to a child
                .pointer_handling(PointerHandling::none())
                .child("Drop Wasm component here")
        )
}

fn main() {
    start_app("app", root);
}
