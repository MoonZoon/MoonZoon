use ignore::WalkBuilder;
use std::{env, ffi::OsStr, fs::File, path::Path};

// https://doc.rust-lang.org/cargo/reference/build-scripts.html

macro_rules! instruction {
    ($($arg: tt)*) => {
        println!($($arg)*)
    }
}

// https://github.com/rust-lang/cargo/issues/985
// macro_rules! warning {
//     ($($arg: tt)*) => {
//         instruction!("cargo:warning={}", format!($($arg)*))
//     }
// }

fn main() {
    instruction!("cargo:rustc-env=TARGET={}", env::var("TARGET").unwrap());
    create_new_project_tar();
}

fn create_new_project_tar() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let file = File::create(Path::new(&out_dir).join("new_project.tar")).unwrap();
    let mut tar_builder = tar::Builder::new(file);

    let new_project_path = Path::new("new_project");
    let extra_ignored_files = [OsStr::new("Makefile.toml"), OsStr::new("Cargo.lock")];

    for entry in WalkBuilder::new(new_project_path).hidden(false).build() {
        let path = entry.unwrap().into_path();
        if path.is_dir() || extra_ignored_files.contains(&path.file_name().unwrap()) {
            continue;
        }
        let tar_path = path.strip_prefix(new_project_path).unwrap();
        tar_builder
            .append_file(&tar_path, &mut File::open(&path).unwrap())
            .unwrap();
    }
    tar_builder.finish().unwrap();
}
