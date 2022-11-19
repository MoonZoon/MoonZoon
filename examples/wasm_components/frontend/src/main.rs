use anyhow::anyhow;
use zoon::{eprintln, named_color::*, println, *};

#[static_ref]
fn drop_zone_active() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn component_said() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

// WARNING: Experimental code that will be refactored. 
async fn load_and_use_component(file_list: web_sys::FileList) -> anyhow::Result<()> {
    let file = file_list
        .get(0)
        .ok_or_else(|| anyhow!("failed to get the dropped file"))?;
    let readable_stream = file.stream();

    let mut response_init = web_sys::ResponseInit::new();

    let headers = web_sys::Headers::new().map_err(|error| anyhow!("{error:#?}"))?;
    headers
        .append("Content-Type", "application/wasm")
        .map_err(|error| anyhow!("{error:#?}"))?;

    response_init.headers(&JsValue::from(headers));

    let response = web_sys::Response::new_with_opt_readable_stream_and_init(
        Some(&readable_stream),
        &response_init,
    )
    .map_err(|error| anyhow!("{error:#?}"))?;

    let response_promise = future_to_promise(future::ok(JsValue::from(response)));

    let import_object = js_sys::Object::new();
    let env_module = js_sys::Object::new();

    let log_result = Closure::<dyn Fn(f64)>::new(|result: f64| {
        println!("The result is: '{result}'");
    })
    .into_js_value();

    Reflect::set(&env_module, &"log_result".into(), &log_result)
        .map_err(|error| anyhow!("{error:#?}"))?;
    Reflect::set(&import_object, &"env".into(), &env_module.into())
        .map_err(|error| anyhow!("{error:#?}"))?;

    let result_object = JsFuture::from(js_sys::WebAssembly::instantiate_streaming(
        &response_promise,
        &import_object,
    ))
    .await
    .map_err(|error| anyhow!("{error:#?}"))?;

    let instance: js_sys::WebAssembly::Instance =
        Reflect::get(&result_object, &JsValue::from("instance"))
            .map_err(|error| anyhow!("{error:#?}"))?
            .dyn_into()
            .map_err(|error| anyhow!("{error:#?}"))?;

    let exports = instance.exports();

    let sum = Reflect::get(exports.as_ref(), &"sum".into())
        .map_err(|error| anyhow!("{error:#?}"))?
        .dyn_into::<js_sys::Function>()
        .map_err(|error| anyhow!("{error:#?}"))?;

    // @TODO see TODO 'components/calculator.main`
    // let some_bytes = Reflect::get(exports.as_ref(), &"some_bytes".into())
    //     .map_err(|error| anyhow!("{error:#?}"))?
    //     .dyn_into::<js_sys::Function>()
    //     .map_err(|error| anyhow!("{error:#?}"))?;

    let a = 1.2;
    let b = 3.4;
    let sum_a_b = sum
        .call2(&JsValue::NULL, &a.into(), &b.into())
        .map_err(|error| anyhow!("{error:#?}"))?
        .as_f64()
        .ok_or_else(|| anyhow!("failed to get `f64` from the `sum` function output"))?;

    let said = format!("{a} + {b} = {sum_a_b}");
    component_said().set(Some(said));

    Ok(())
}

fn root() -> impl Element {
    Column::new()
        .s(Width::exact(300))
        .s(Align::center())
        .s(Gap::new().y(20))
        .item(drop_zone())
        .item_signal(component_said().signal_cloned().map_some(|text| {
            Paragraph::new()
                .s(Align::new().center_x())
                .content("Component said: ")
                .content(
                    El::new()
                        .s(Font::new().weight(FontWeight::SemiBold))
                        .child(text),
                )
        }))
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
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::DragEnter| {
                        event.stop_propagation();
                        event.prevent_default();
                        drop_zone_active().set_neq(true);
                    },
                )
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::DragOver| {
                        event.stop_propagation();
                        event.prevent_default();
                        event.data_transfer().unwrap_throw().set_drop_effect("copy");
                    },
                )
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::DragLeave| {
                        event.stop_propagation();
                        event.prevent_default();
                        drop_zone_active().set_neq(false);
                    },
                )
                .event_handler_with_options(
                    EventOptions::new().preventable(),
                    |event: events::Drop| {
                        event.stop_propagation();
                        event.prevent_default();
                        drop_zone_active().set_neq(false);
                        let file_list = event.data_transfer().unwrap_throw().files().unwrap_throw();
                        Task::start(async move {
                            if let Err(error) = load_and_use_component(file_list).await {
                                eprintln!("{error:#}");
                            }
                        });
                    },
                )
        })
        .child(
            El::new()
                .s(Align::center())
                // @TODO the new ability shouldn't fire `dragleave` on moving to a child
                .pointer_handling(PointerHandling::none())
                .child("Drop Wasm component here"),
        )
}

fn main() {
    start_app("app", root);
}
