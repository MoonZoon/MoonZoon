#[no_mangle]
pub extern "C" fn sum(a: f64, b: f64) -> f64 {
    let result = a + b;
    unsafe { log_result(result) };
    result
}

// @TODO make it extern with a proc_macro
// - https://radu-matei.com/blog/practical-guide-to-wasm-memory/#passing-arrays-to-rust-webassembly-modules
// - https://docs.rs/wasmi/latest/wasmi/index.html
// - https://github.com/paritytech/wasmi/issues/203
// - https://github.com/andrewdavidmackenzie/wasm_explore
// - https://nishtahir.com/interacting-with-wasm-memory/
// - https://github.com/ZuInnoTe/rust-wasm-dynamic-module-study
// - https://docs.rs/bincode/latest/bincode/
pub fn some_bytes() -> Vec<u8> {
    vec![1, 2, 3]
}

extern "C" {
    fn log_result(result: f64) -> f64;
}
