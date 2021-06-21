use crate::*;

// ------ println ------

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ($crate::console::log(&$crate::format!($($arg)*)))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(input: &str);
}

// ------ eprintln ------

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ($crate::console::error(&$crate::format!($($arg)*)))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn error(input: &str);
}
