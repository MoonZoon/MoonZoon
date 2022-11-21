use anyhow::anyhow;
use std::sync::{Arc, Mutex};
use wasmer::{imports, Function, FunctionEnv, FunctionEnvMut, Instance, Module, Store, WasmPtr};
use zoon::{eprintln, named_color::*, println, *};

#[static_ref]
fn drop_zone_active() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn component_said() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

type Size = i32;
type Ptr = i32;

// WARNING: Both `Wasmer` host and Wasm components use low-level communication mechanisms with `unsafe`
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

    let mut store = Store::default();
    let module = Module::new(&store, file_bytes).await?;

    #[derive(Clone, Default)]
    struct EnvData {
        instance: Arc<Mutex<Option<SendWrapper<Arc<Instance>>>>>,
    }

    let env_data = EnvData::default();
    let env = FunctionEnv::new(&mut store, env_data.clone());

    fn host_log(env: FunctionEnvMut<EnvData>, ptr: WasmPtr<u8>, size: Size) {
        let result: anyhow::Result<()> = (|| {
            let instance_lock = env
                .data()
                .instance
                .lock()
                .map_err(|error| anyhow!("failed to lock `Instance`: {error:#}"))?;
            let instance = instance_lock
                .as_ref()
                .ok_or_else(|| anyhow!("instance cannot be `None`"))?;
            let memory = instance.exports.get_memory("memory")?;

            let memory_view = memory.view(&env);
            let message = ptr.read_utf8_string(&memory_view, size.try_into()?)?;
            println!("{message}");
            Ok(())
        })();
        if let Err(error) = result {
            eprintln!("host_log failed: {error:#?}");
        }
    }

    let import_object = imports! {
        "env" => {
            "host_log" => Function::new_typed_with_env(&mut store, &env, host_log)
        }
    };

    let instance = Arc::new(Instance::new(&mut store, &module, &import_object).await?);
    env_data
        .instance
        .lock()
        .map_err(|error| anyhow!("failed to lock `Instance`: {error:#}"))?
        .replace(SendWrapper::new(instance.clone()));

    let memory = instance.exports.get_memory("memory")?;
    let alloc = instance
        .exports
        .get_typed_function::<Size, WasmPtr<u8>>(&store, "alloc")?;
    let free = instance
        .exports
        .get_typed_function::<(Ptr, Size), ()>(&store, "free")?;

    let mut new_component_said = String::new();
    // call `sum`
    {
        let sum = instance
            .exports
            .get_typed_function::<(f64, f64), f64>(&store, "sum")?;

        let a = 1.2;
        let b = 3.4;
        let sum_a_b = sum.call(&mut store, a, b)?;

        new_component_said.push_str(&format!("\n{a} + {b} = {sum_a_b}"));
    }
    // call `sum_array`
    {
        let sum_array = instance
            .exports
            .get_typed_function::<(Ptr, Size), f64>(&store, "sum_array")?;

        let addends = vec![1.25, 2.5, 3.1, 60.];
        let addends_serialized = bincode::serialize(&addends)?;
        let addends_size: Size = addends_serialized.len().try_into()?;

        let ptr = alloc.call(&mut store, addends_size)?;

        memory
            .view(&store)
            .write(ptr.offset().into(), &addends_serialized)?;
        let addends_sum = sum_array.call(&mut store, ptr.offset().try_into()?, addends_size)?;

        free.call(&mut store, ptr.offset().try_into()?, addends_size)?;

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
