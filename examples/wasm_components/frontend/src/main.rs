use anyhow::anyhow;
use wai_bindgen_wasmer::wasmer::{Imports, Module, Store};
use zoon::{eprintln, named_color::*, println, *};

// @TODO-WASMER: Uncomment once the Wasmer generator is updated.
// @TODO-WASMER: See the two longer Rust modules with generated code at the end of this file.
// wai_bindgen_wasmer::import!("components/calculator/calculator.wai");
// wai_bindgen_wasmer::export!("components/calculator/host.wai");

#[static_ref]
fn drop_zone_active() -> &'static Mutable<bool> {
    Mutable::new(false)
}

#[static_ref]
fn component_said() -> &'static Mutable<Option<String>> {
    Mutable::new(None)
}

async fn load_and_use_component(file_list: web_sys::FileList) -> anyhow::Result<()> {
    let file_bytes = file_list
        .get(0)
        .ok_or_else(|| anyhow!("failed to get the dropped file"))?
        .apply(|file| JsFuture::from(file.array_buffer()))
        .await
        .map_err(|error| anyhow!("{error:#?}"))?
        .apply_ref(js_sys::Uint8Array::new)
        .to_vec();

    struct Host;

    impl host::Host for Host {
        fn register_plugin(&mut self, plugin: host::Plugin) -> Result<(), host::Error> {
            println!("[host]: Plugin to registrate: {plugin:#?}");
            Err("testing error :)".to_owned())
        }

        fn log(&mut self, message: &str) {
            println!("[guest]: {message}");
        }
    }

    let mut store = Store::default();
    let module = Module::new(&store, file_bytes).await?;
    let mut imports = Imports::new();

    let init_host = host::add_to_imports(&mut store, &mut imports, Host);
    let (calculator, instance) = calculator::Calculator::instantiate(&mut store, &module, &mut imports).await?;
    init_host(&instance, &store)?;

    let init_data = calculator::InitData {
        instance_id: 3,
        host_name: "MoonZoon Wasm app",
    };
    calculator.init_plugin(&mut store, init_data)?;

    let mut new_component_said = String::new();

    let a = 1.2;
    let b = 3.4;
    let sum_a_b = calculator.sum(&mut store, a, b)?;
    new_component_said.push_str(&format!("\n{a} + {b} = {sum_a_b}"));

    let addends = [1.25, 2.5, 3.1, 60.];
    let addends_sum = calculator.sum_list(&mut store, &addends)?;
    new_component_said.push_str(&format!("\nSum {addends:?} = {addends_sum}"));

    component_said().set(Some(new_component_said));
    println!("Done!");
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
        // @TODO refactor with a new MoonZoon ability
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



// @TODO-WASMER: Remove once the generator is adapted to changes (i.e. once the compilation passes).
// @TODO-WASMER: See the todos at the top of this file.
//
// @TODO-WASMER: How to get the code:
// 1. cargo install wai-bindgen-cli
// 2. wai-bindgen wasmer --export frontend/components/calculator/host.wai
// 3. Copy the code from `bindings.rs` and then remove the file.
//
// Warning: The code patches implemented below probably aren't the most optimal ones!
#[allow(clippy::all)]
pub mod host {
  #[allow(unused_imports)]
  use wai_bindgen_wasmer::{anyhow, wasmer};
  pub type Error = String;
  #[derive(Clone)]
  pub struct Plugin<'a,> {
    pub name: &'a  str,
    pub version: Option<f32>,
  }
  impl<'a,> core::fmt::Debug for Plugin<'a,> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Plugin").field("name", &self.name).field("version", &self.version).finish()}
  }
  pub trait Host: Sized + Send + Sync + 'static{
    fn register_plugin(&mut self,plugin: Plugin<'_,>,) -> Result<(),Error>;
    
    fn log(&mut self,message: & str,) -> ();
    
  }
  pub struct LazyInitialized {
    memory: wasmer::Memory,
    func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
  }
  
  #[must_use = "The returned initializer function must be called
  with the instance and the store before starting the runtime"]
  pub fn add_to_imports<T>(store: &mut wasmer::Store, imports: &mut wasmer::Imports, data: T)
  -> impl FnOnce(&wasmer::Instance, &dyn wasmer::AsStoreRef) -> Result<(), anyhow::Error>
  where T: Host
  {
    #[derive(Clone)]struct EnvWrapper<T: Host> {
      data: T,
      lazy: std::rc::Rc<OnceCell<LazyInitialized>>,
    }
    unsafe impl<T: Host> Send for EnvWrapper<T> {}
    unsafe impl<T: Host> Sync for EnvWrapper<T> {}
    let lazy = std::rc::Rc::new(OnceCell::new());
    let env = EnvWrapper {
      data,
      lazy: std::rc::Rc::clone(&lazy),
    };
    let env = wasmer::FunctionEnv::new(&mut *store, env);
    let mut exports = wasmer::Exports::new();
    let mut store = store.as_store_mut();
    exports.insert(
    "register-plugin",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,arg0:i32,arg1:i32,arg2:i32,arg3:f32,arg4:i32| -> Result<(), wasmer::RuntimeError> {
      let func_canonical_abi_realloc = store
      .data()
      .lazy
      .get()
      .unwrap()
      .func_canonical_abi_realloc
      .clone();
      let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
      let _memory_view = _memory.view(&store);

      // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
      // let mut _bc = wai_bindgen_wasmer::BorrowChecker::new(unsafe {
      //   _memory_view.data_unchecked_mut()
      // });

      let data_mut = store.data_mut();
      let ptr0 = arg0;
      let len0 = arg1;

      // @TODO-WASMER: original code:
      // let param0 = Plugin{name:_bc.slice_str(ptr0, len0)?, version:match arg2 {
      // @TODO-WASMER: new code:
      let mut name_buffer = vec![0; len0 as usize];
      _memory_view.read(ptr0 as u64, &mut name_buffer)?;
      let name = std::str::from_utf8(&name_buffer).map_err(|error| wasmer::RuntimeError::new(error.to_string()))?;
      let param0 = Plugin{name, version:match arg2 {

        0 => None,
        1 => Some(arg3),
        _ => return Err(invalid_variant("option")),
      }, };
      let host = &mut data_mut.data;
      let result = host.register_plugin(param0, );
      match result {
        Ok(e) => { {
          let _memory_view = _memory.view(&store);

          // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
          // @TODO-WASMER: original code:
          // let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
          // caller_memory.store(arg4 + 0, wai_bindgen_wasmer::rt::as_i32(0i32) as u8)?;
          // @TODO-WASMER: new code:
          _memory_view.write_u8((arg4 + 0) as u64, wai_bindgen_wasmer::rt::as_i32(0i32) as u8)?;

          let () = e;
        } },
        Err(e) => { {
          let _memory_view = _memory.view(&store);

          // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
          // @TODO-WASMER: original code:
          // let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
          // caller_memory.store(arg4 + 0, wai_bindgen_wasmer::rt::as_i32(1i32) as u8)?;
          // @TODO-WASMER: new code:
          _memory_view.write_u8((arg4 + 0) as u64, wai_bindgen_wasmer::rt::as_i32(1i32) as u8)?;

          let vec1 = e;
          let ptr1 = func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 1, vec1.len() as i32)?;
          let _memory_view = _memory.view(&store);

          // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
          // @TODO-WASMER: original code:
          // let caller_memory = unsafe { _memory_view.data_unchecked_mut() };
          // caller_memory.store_many(ptr1, vec1.as_bytes())?;
          // caller_memory.store(arg4 + 8, wai_bindgen_wasmer::rt::as_i32(vec1.len() as i32))?;
          // caller_memory.store(arg4 + 4, wai_bindgen_wasmer::rt::as_i32(ptr1))?;
          // @TODO-WASMER: new code:
          _memory_view.write(ptr1 as u64, vec1.as_bytes())?;
          _memory_view.write((arg4 + 8) as u64, &wai_bindgen_wasmer::rt::as_i32(vec1.len() as i32).to_le_bytes())?;
          _memory_view.write((arg4 + 4) as u64, &wai_bindgen_wasmer::rt::as_i32(ptr1).to_le_bytes())?;

        } },
      };Ok(())
    }
    ));
    exports.insert(
    "log",
    wasmer::Function::new_typed_with_env(
    &mut store,
    &env,
    move |mut store: wasmer::FunctionEnvMut<EnvWrapper<T>>,arg0:i32,arg1:i32| -> Result<(), wasmer::RuntimeError> {
      let _memory: wasmer::Memory = store.data().lazy.get().unwrap().memory.clone();
      let _memory_view = _memory.view(&store);

      // // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
      // let mut _bc = wai_bindgen_wasmer::BorrowChecker::new(unsafe {
      //   _memory_view.data_unchecked_mut()
      // });

      let data_mut = store.data_mut();
      let ptr0 = arg0;
      let len0 = arg1;

      // @TODO-WASMER: original code:
      // let param0 = _bc.slice_str(ptr0, len0)?;
      // @TODO-WASMER: new code:
      let mut param0_buffer = vec![0; len0 as usize];
      _memory_view.read(ptr0 as u64, &mut param0_buffer)?;
      let param0 = std::str::from_utf8(&param0_buffer).map_err(|error| wasmer::RuntimeError::new(error.to_string()))?;

      let host = &mut data_mut.data;
      let result = host.log(param0, );
      let () = result;
      Ok(())
    }
    ));
    imports.register_namespace("host", exports);
    move |_instance: &wasmer::Instance, _store: &dyn wasmer::AsStoreRef| {
      let memory = _instance.exports.get_memory("memory")?.clone();
      let func_canonical_abi_realloc = _instance
      .exports
      .get_typed_function(
      &_store.as_store_ref(),
      "canonical_abi_realloc",
      )
      .unwrap()
      .clone();
      lazy.set(LazyInitialized {
        memory,
        func_canonical_abi_realloc,
      })
      .map_err(|_e| anyhow::anyhow!("Couldn't set lazy initialized data"))?;
      Ok(())
    }
  }
  use wai_bindgen_wasmer::once_cell::unsync::OnceCell;
  #[allow(unused_imports)]
  use wasmer::AsStoreMut as _;
  #[allow(unused_imports)]
  use wasmer::AsStoreRef as _;
  // @TODO-WASMER: `RawMem` is no longer needed.
  // use wai_bindgen_wasmer::rt::RawMem;
  use wai_bindgen_wasmer::rt::invalid_variant;
}





// @TODO-WASMER: Remove once the generator is adapted to changes (i.e. once the compilation passes).
// @TODO-WASMER: See the todos at the top of this file.
//
// @TODO-WASMER: How to get the code:
// 1. cargo install wai-bindgen-cli
// 2. wai-bindgen wasmer --import frontend/components/calculator/calculator.wai
// 3. Copy the code from `bindings.rs` and then remove the file.
//
// Warning: The code patches implemented below probably aren't the most optimal ones!
#[allow(clippy::all)]
pub mod calculator {
  #[allow(unused_imports)]
  use wai_bindgen_wasmer::{anyhow, wasmer};
  #[derive(Clone)]
  pub struct InitData<'a,> {
    pub instance_id: u32,
    pub host_name: &'a  str,
  }
  impl<'a,> core::fmt::Debug for InitData<'a,> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("InitData").field("instance-id", &self.instance_id).field("host-name", &self.host_name).finish()}
  }
  
  /// Auxiliary data associated with the wasm exports.
  #[derive(Default)]
  pub struct CalculatorData {
  }
  
  pub struct Calculator {
    #[allow(dead_code)]
    env: wasmer::FunctionEnv<CalculatorData>,
    func_canonical_abi_realloc: wasmer::TypedFunction<(i32, i32, i32, i32), i32>,
    func_init_plugin: wasmer::TypedFunction<(i32,i32,i32,), ()>,
    func_sum: wasmer::TypedFunction<(f64,f64,), f64>,
    func_sum_list: wasmer::TypedFunction<(i32,i32,), f64>,
    memory: wasmer::Memory,
  }
  impl Calculator {
    #[allow(unused_variables)]
    
    /// Adds any intrinsics, if necessary for this exported wasm
    /// functionality to the `ImportObject` provided.
    ///
    /// This function returns the `CalculatorData` which needs to be
    /// passed through to `Calculator::new`.
    fn add_to_imports(
    mut store: impl wasmer::AsStoreMut,
    imports: &mut wasmer::Imports,
    ) -> wasmer::FunctionEnv<CalculatorData> {
      let env = wasmer::FunctionEnv::new(&mut store, CalculatorData::default());
      env
    }
    
    /// Instantiates the provided `module` using the specified
    /// parameters, wrapping up the result in a structure that
    /// translates between wasm and the host.
    ///
    /// The `imports` provided will have intrinsics added to it
    /// automatically, so it's not necessary to call
    /// `add_to_imports` beforehand. This function will
    /// instantiate the `module` otherwise using `imports`, and
    /// both an instance of this structure and the underlying
    /// `wasmer::Instance` will be returned.
    // @TODO-WASMER: Needed to add `async`.
    pub async fn instantiate(
    mut store: impl wasmer::AsStoreMut,
    module: &wasmer::Module,
    imports: &mut wasmer::Imports,
    ) -> anyhow::Result<(Self, wasmer::Instance)> {
      let env = Self::add_to_imports(&mut store, imports);
      // @TODO-WASMER: Needed to add `.await`.
      let instance = wasmer::Instance::new(
      &mut store, module, &*imports).await?;
      
      Ok((Self::new(store, &instance, env)?, instance))
    }
    
    /// Low-level creation wrapper for wrapping up the exports
    /// of the `instance` provided in this structure of wasm
    /// exports.
    ///
    /// This function will extract exports from the `instance`
    /// and wrap them all up in the returned structure which can
    /// be used to interact with the wasm module.
    pub fn new(
    store: impl wasmer::AsStoreMut,
    _instance: &wasmer::Instance,
    env: wasmer::FunctionEnv<CalculatorData>,
    ) -> Result<Self, wasmer::ExportError> {
      let func_canonical_abi_realloc= _instance.exports.get_typed_function(&store, "canonical_abi_realloc")?;
      let func_init_plugin= _instance.exports.get_typed_function(&store, "init-plugin")?;
      let func_sum= _instance.exports.get_typed_function(&store, "sum")?;
      let func_sum_list= _instance.exports.get_typed_function(&store, "sum-list")?;
      let memory= _instance.exports.get_memory("memory")?.clone();
      Ok(Calculator{
        func_canonical_abi_realloc,
        func_init_plugin,
        func_sum,
        func_sum_list,
        memory,
        env,
      })
    }
    pub fn init_plugin(&self, store: &mut wasmer::Store,data: InitData<'_,>,)-> Result<(), wasmer::RuntimeError> {
      let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
      let _memory = &self.memory;
      let InitData{ instance_id:instance_id0, host_name:host_name0, } = data;
      let vec1 = host_name0;
      let ptr1 = func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 1, vec1.len() as i32)?;
      let _memory_view = _memory.view(&store);

      // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
      // @TODO-WASMER: original code:
      // unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr1, vec1.as_bytes())?;
      // @TODO-WASMER: new code:
      _memory_view.write(ptr1 as u64, vec1.as_bytes())?;
      
      self.func_init_plugin.call(store, wai_bindgen_wasmer::rt::as_i32(instance_id0), ptr1, vec1.len() as i32, )?;
      Ok(())
    }
    pub fn sum(&self, store: &mut wasmer::Store,a: f64,b: f64,)-> Result<f64, wasmer::RuntimeError> {
      let result0 = self.func_sum.call(store, a, b, )?;
      Ok(result0)
    }
    pub fn sum_list(&self, store: &mut wasmer::Store,addends: &[f64],)-> Result<f64, wasmer::RuntimeError> {
      let func_canonical_abi_realloc = &self.func_canonical_abi_realloc;
      let _memory = &self.memory;
      let vec0 = addends;
      let ptr0 = func_canonical_abi_realloc.call(&mut store.as_store_mut(), 0, 0, 8, (vec0.len() as i32) * 8)?;
      let _memory_view = _memory.view(&store);

      // @TODO-WASMER: `data_unchecked_mut()` is not usable in the browser
      // @TODO-WASMER: original code:
      // unsafe { _memory_view.data_unchecked_mut() }.store_many(ptr0, &vec0)?;
      // @TODO-WASMER: new code:
      _memory_view.write(ptr0 as u64, &vec0.iter().flat_map(|f| f.to_le_bytes()).collect::<Vec<_>>())?;

      let result1 = self.func_sum_list.call(store, ptr0, vec0.len() as i32, )?;
      Ok(result1)
    }
  }
  #[allow(unused_imports)]
  use wasmer::AsStoreMut as _;
  #[allow(unused_imports)]
  use wasmer::AsStoreRef as _;
  // @TODO-WASMER: `RawMem` is no longer needed.
  // use wai_bindgen_wasmer::rt::RawMem;
}
