// #![no_std]

// rust-analyzer without imports like `crate::counter` can't find element macros
#![allow(macro_expanded_macro_exports_accessed_by_absolute_paths)]

use zoon::*;

mod app;

#[wasm_bindgen(start)]
pub fn start() {
    // start!(app)

    let rcx = RenderContext { 
        index: 0,
        node: None,
        component_id: None, 
    };

    dominator::append_dom(&dominator::get_id("app"), app::cmp::root().render());
}
