use wasm_bindgen::{prelude::*};

#[macro_export]
macro_rules! log {
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(input: &str);
}
