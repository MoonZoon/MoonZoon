use crate::*;

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ($crate::log(&$crate::format!($($arg)*)))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(input: &str);
}
