use anyhow::anyhow;
use wasmi::{core::F64, Caller, Engine, Extern, Func, Linker, Module, Store};
use zoon::{eprintln, named_color::*, println, *};

#[static_ref]
fn drop_zone_active() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn component_said() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

// WARNING: Both `Wasmi` host and Wasm components use low-level communication mechanisms with `unsafe`
// with direct memory manipulation. A library/bindgen should be used for communication in the future.
async fn load_and_use_component(file_list: web_sys::FileList) -> anyhow::Result<()> {
    let file_bytes = file_list
        .get(0)
        .ok_or_else(|| anyhow!("failed to get the dropped file"))?
        .apply(|file| JsFuture::from(file.array_buffer()))
        .await
        .map_err(|error| anyhow!("{error:#?}"))?
        .apply_ref(js_sys::Uint8Array::new)
        .to_vec();

    let engine = Engine::default();
    let module = Module::new(&engine, file_bytes.as_slice())?;

    type HostState = ();
    let mut store = Store::new(&engine, ());

    let host_log = Func::wrap(
        &mut store,
        |caller: Caller<'_, HostState>, ptr: u32, size: u32| {
            let result = (|| {
                let memory = caller
                    .get_export("memory")
                    .and_then(Extern::into_memory)
                    .ok_or_else(|| anyhow!("could not find memory \"memory\""))?;

                let mut message = vec![0; size as usize];
                memory
                    .read(caller, ptr as usize, message.as_mut_slice())
                    .map_err(wasmi::Error::Memory)?;
                let message = String::from_utf8(message)?;

                println!("{message}");
                Ok::<(), anyhow::Error>(())
            })();
            if let Err(error) = result {
                eprintln!("{error:#}");
            }
        },
    );

    let mut linker = Linker::<HostState>::new();
    linker.define("env", "host_log", host_log)?;

    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;

    let alloc = instance
        .get_export(&store, "alloc")
        .and_then(Extern::into_func)
        .ok_or_else(|| anyhow!("could not find function \"alloc\""))?
        .typed::<u32, u32>(&store)?;

    let free = instance
        .get_export(&store, "free")
        .and_then(Extern::into_func)
        .ok_or_else(|| anyhow!("could not find function \"free\""))?
        .typed::<(u32, u32), ()>(&store)?;

    let memory = instance
        .get_export(&mut store, "memory")
        .and_then(Extern::into_memory)
        .ok_or_else(|| anyhow!("could not find memory \"memory\""))?;

    let mut new_component_said = String::new();
    // call `sum`
    {
        let sum = instance
            .get_export(&store, "sum")
            .and_then(Extern::into_func)
            .ok_or_else(|| anyhow!("could not find function \"sum\""))?
            .typed::<(F64, F64), F64>(&mut store)?;

        let a = 1.2;
        let b = 3.4;
        let sum_a_b = sum.call(&mut store, (a.into(), b.into()))?;

        new_component_said.push_str(&format!("\n{a} + {b} = {sum_a_b}"));
    }
    // call `sum_array`
    {
        let sum_array = instance
            .get_export(&store, "sum_array")
            .and_then(Extern::into_func)
            .ok_or_else(|| anyhow!("could not find function \"sum_array\""))?
            .typed::<(u32, u32), F64>(&mut store)?;

        let addends = vec![1.25, 2.5, 3.1, 60.];
        let addends_serialized = bincode::serialize(&addends)?;
        let addends_size = addends_serialized.len() as u32;

        let ptr = alloc.call(&mut store, addends_size)?;
        memory
            .write(&mut store, ptr as usize, &addends_serialized)
            .map_err(wasmi::Error::Memory)?;
        let addends_sum = sum_array.call(&mut store, (ptr, addends_size))?;
        free.call(&mut store, (ptr, addends_size))?;

        new_component_said.push_str(&format!("\nSum {addends:?} = {addends_sum}"));
    }
    component_said().set(Some(new_component_said));
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
        // @TODO refactor with a new ability
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
