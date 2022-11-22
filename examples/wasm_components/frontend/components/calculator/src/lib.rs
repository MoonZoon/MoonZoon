wai_bindgen_rust::export!("calculator.wai");
wai_bindgen_rust::import!("host.wai");

macro_rules! log {
    ($($arg:tt)*) => (host::log(&format!($($arg)*)))
}

struct Calculator;

impl calculator::Calculator for Calculator {
    fn init_plugin(data: calculator::InitData) {
        log!("calculator init-data: '{data:#?}'");
        let plugin = host::Plugin {
            name: "Calculator",
            version: None,
        };
        if let Err(error) = host::register_plugin(plugin) {
            log!("plugin registration failed: '{error}'");
        }
    }

    fn sum(a: f64, b: f64) -> f64 {
        let result = a + b;
        log!("sum result is {result}");
        result
    }

    fn sum_list(addends: Vec<f64>) -> f64 {
        let result = addends.iter().sum();
        log!("sum_array result is {result}");
        result
    }
}
