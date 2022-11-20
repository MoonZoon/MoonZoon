fn host_log(message: String) {
    let message = message.into_bytes();
    unsafe { imported::host_log(message.as_ptr(), message.len()) };
}
mod imported {
    extern "C" {
        pub fn host_log(ptr: *const u8, size: usize);
    }
}

macro_rules! log {
    ($($arg:tt)*) => (host_log(format!($($arg)*)))
}

#[no_mangle]
pub extern "C" fn sum(a: f64, b: f64) -> f64 {
    let result = a + b;
    log!("sum result is {result}");
    result
}

#[no_mangle]
pub unsafe extern "C" fn sum_array(ptr: *const u8, size: usize) -> f64 {
    let encoded_data = core::slice::from_raw_parts(ptr, size);
    let Ok(array) = bincode::deserialize::<Vec<f64>>(encoded_data) else {
        let error_message = "deserialization_failed";
        log!("ERROR: '{error_message}'");
        panic!("{error_message}");
    };
    let result = array.into_iter().sum();
    log!("sum_array result is {result}");
    result
}

// -- helpers --

#[no_mangle]
pub unsafe extern "C" fn alloc(size: usize) -> *mut u8 {
    core::mem::ManuallyDrop::new(Vec::with_capacity(size)).as_mut_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8, size: usize) {
    Vec::from_raw_parts(ptr, size, size);
}
