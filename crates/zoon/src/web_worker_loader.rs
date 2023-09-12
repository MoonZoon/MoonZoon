use crate::*;

pub struct WebWorkerLoader {
    path: String,
}

impl WebWorkerLoader {
    // @TODO add method `new_droppable`?

    pub fn new(crate_name: &str) -> Self {
        const FRONTEND_BUILD_ID: &str = env!("FRONTEND_BUILD_ID");
        const CACHE_BUSTING: &str = env!("CACHE_BUSTING");

        let current_href = window().location().href().unwrap_throw();

        let js_url = if CACHE_BUSTING == "true" {
            format!("/_api/web_workers/{crate_name}/pkg/{crate_name}_{FRONTEND_BUILD_ID}.js")
        } else {
            format!("/_api/web_workers/{crate_name}/pkg/{crate_name}.js")
        };
        let js_url = web_sys::Url::new_with_base(&js_url, &current_href)
            .expect_throw("Failed to create URL for Web Worker Javascript")
            .to_string();

        let wasm_url = if CACHE_BUSTING == "true" {
            format!("/_api/web_workers/{crate_name}/pkg/{crate_name}_bg_{FRONTEND_BUILD_ID}.wasm")
        } else {
            format!("/_api/web_workers/{crate_name}/pkg/{crate_name}_bg.wasm")
        };
        let wasm_url = web_sys::Url::new_with_base(&wasm_url, &current_href)
            .expect_throw("Failed to create URL for Web Worker Wasm")
            .to_string();

        let array: js_sys::Array = js_sys::Array::new();
        array.push(&format!(r#"importScripts("{js_url}");wasm_bindgen("{wasm_url}");"#).into());

        let blob = web_sys::Blob::new_with_str_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new().type_("application/javascript"),
        )
        .unwrap_throw();

        Self {
            path: web_sys::Url::create_object_url_with_blob(&blob).unwrap_throw(),
        }
    }

    pub fn new_from_frontend() -> Self {
        const FRONTEND_BUILD_ID: &str = env!("FRONTEND_BUILD_ID");
        const CACHE_BUSTING: &str = env!("CACHE_BUSTING");

        let current_href = window().location().href().unwrap_throw();

        let js_url = if CACHE_BUSTING == "true" {
            format!("/_api/pkg/frontend_{FRONTEND_BUILD_ID}.js")
        } else {
            format!("/_api/pkg/frontend.js")
        };
        let js_url = web_sys::Url::new_with_base(&js_url, &current_href)
            .expect_throw("Failed to create URL for Web Worker Javascript")
            .to_string();

        let array: js_sys::Array = js_sys::Array::new();
        array.push(
            &format!(
                r#"
            importScripts("{js_url}");
            self.onmessage = async event => {{
                const wasm_module = event.data[0];
                const wasm_memory = event.data[1];
                const closure_pointer_u32 = Number(event.data[2]);

                const {{ worker_entry_point }} = await wasm_bindgen(
                  wasm_module,
                  wasm_memory,  
                );

                worker_entry_point(Number(closure_pointer_u32));
              }}
        "#
            )
            .into(),
        );

        let blob = web_sys::Blob::new_with_str_sequence_and_options(
            &array,
            web_sys::BlobPropertyBag::new().type_("application/javascript"),
        )
        .unwrap_throw();

        Self {
            path: web_sys::Url::create_object_url_with_blob(&blob).unwrap_throw(),
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}
