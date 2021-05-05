pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use static_ref_macro::static_ref;

pub mod element;
mod dom;
mod console;
mod futures_signals_ext;

pub use futures_signals_ext::{MutableExt, MutableVecExt};
pub use element::*;
pub use dom::{window, document}; 
pub use console::log;
pub use once_cell;
pub use futures_signals::{
    self,
    map_mut,
    map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    signal_map::{MutableBTreeMap, SignalMap, SignalMapExt},
};
pub use dominator::{self, Dom, DomBuilder, events, traits::StaticEvent};
pub use enclose::enc as clone;
pub use paste;
pub use ufmt::{self, uDebug, uDisplay, uWrite, uwrite, uwriteln};
pub use lexical::{self, WriteIntegerOptions, WriteFloatOptions, NumberFormat};

use wee_alloc::WeeAlloc;

#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

pub trait FlagSet {}
pub trait FlagNotSet {}

#[macro_export]
macro_rules! make_flags {
    ($($flag:ident),*) => {
        $(paste::paste!{
            #[derive(Default)]
            pub struct [<$flag FlagSet>];
            #[derive(Default)]
            pub struct [<$flag FlagNotSet>];
            impl $crate::FlagSet for [<$flag FlagSet>] {}
            impl $crate::FlagNotSet for [<$flag FlagNotSet>] {}
        })*
    }
}

#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut text = String::new();
        $crate::ufmt::uwrite!(&mut text, $($arg)*).unwrap_throw();
        text
    }}
}

pub fn start_app<'a, E: Element>(browser_element_id: impl Into<Option<&'a str>>, view_root: impl FnOnce() -> E) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let parent = browser_element_id
        .into()
        .map(dominator::get_id)
        .unwrap_or_else(|| dominator::body().unchecked_into());

    dominator::append_dom(&parent, view_root().into_raw_element().into_dom());
}
