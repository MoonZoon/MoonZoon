pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use blocks::blocks;

pub mod element;
mod dom;
mod console;
mod hook;
mod l_var;
mod runtime;
mod l_var_map;

pub use element::*;
pub use dom::{Node, window, document}; 
pub use l_var::{LVar, CloneLVar};
pub use console::log;
pub use hook::{l_var, do_once};
pub use topo;
pub use topo::nested as render;
pub use topo::nested as cmp;

#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

const ELEMENT_ID: &str = "app";

fn runtime_run_once<E: Element>(root_cmp: impl Fn() -> E) {
    runtime::run_once(|| root(root_cmp));
}

#[macro_export]
macro_rules! start {
    ($root_cmp:expr) => {
        $crate::start($root_cmp);
    };
}

pub fn start<E: Element>(root_cmp: impl Fn() -> E + Copy) {
    log!("start");
    console_error_panic_hook::set_once();

    // for revision in 0..2 {
    //     log!("revision: {}", revision);
    //     root();
    // }

    log!("-------- revision: 0 --------");
    runtime_run_once(root_cmp);

    // log!("-------- revision: 1 --------");
    // runtime_run_once(root_cmp);

    // log!("-------- revision: 3 --------");
    // runtime_run_once(root_cmp);
}

#[topo::nested]
fn root<E: Element>(root_cmp: impl Fn() -> E) {
    log!("root");

    let node = l_var(|| Node {
        node_ws: web_sys::Node::from(document().get_element_by_id(ELEMENT_ID).expect("root element"))
    });

    let rcx = RenderContext { 
        index: 0,
        node 
    };

    root_cmp().render(rcx);
}
