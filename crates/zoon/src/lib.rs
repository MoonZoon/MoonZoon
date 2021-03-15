pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use blocks_macro::blocks;
pub use s_var_macro::s_var;
pub use update_macro::update;

pub mod element;
mod dom;
mod console;
mod hook;
mod l_var;
mod l_var_map;
mod s_var;
mod s_var_map;
mod runtime;

pub use element::*;
pub use dom::{Node, window, document}; 
pub use l_var::{LVar, CloneLVar};
pub use s_var::{SVar, CloneSVar, s_var};
pub use console::log;
pub use hook::{l_var, do_once};
pub use topo;
pub use topo::nested as render;
pub use topo::nested as cmp;
use runtime::ROOT_CMP;
pub use runtime::rerender;

#[macro_export]
macro_rules! with_dollar_sign {
    ($($body:tt)*) => {
        macro_rules! __with_dollar_sign { $($body)* }
        __with_dollar_sign!($);
    }
}

const ELEMENT_ID: &str = "app";

#[macro_export]
macro_rules! start {
    ($root_cmp:expr) => {
        $crate::start($root_cmp);
    };
}

pub fn start<E: Element + 'static>(root: fn() -> E) {
    let root = Box::new(move || Box::new(root()) as Box<dyn Element>);
    ROOT_CMP.with(move |app_root| {
        *app_root.borrow_mut() =  Some(root);
    });

    // log!("start");
    console_error_panic_hook::set_once();

    rerender();
}

#[topo::nested]
fn root() {
    // log!("root");

    let node = l_var(|| Node {
        node_ws: web_sys::Node::from(document().get_element_by_id(ELEMENT_ID).expect("root element"))
    });

    let rcx = RenderContext { 
        index: 0,
        node 
    };

    ROOT_CMP.with(|app_root| {
        if let Some(app_root) = app_root.borrow_mut().as_ref() {
            app_root().render(rcx);
        }
    });
}
