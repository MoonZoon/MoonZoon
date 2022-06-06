use ignore::Walk;
use std::env;

// https://doc.rust-lang.org/cargo/reference/build-scripts.html

macro_rules! instruction {
    ($($arg: tt)*) => {
        println!($($arg)*)
    }
}

// https://github.com/rust-lang/cargo/issues/985
macro_rules! warning {
    ($($arg: tt)*) => {
        instruction!("cargo:warning={}", format!($($arg)*))
    }
}

fn main() {
    instruction!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());

    for entry in Walk::new("../../examples/new_project") {
        let entry = entry.unwrap();
        if entry.file_name() == "Makefile.toml" {
            continue;
        }
        warning!("{:#?}", entry.path());
    }
}
