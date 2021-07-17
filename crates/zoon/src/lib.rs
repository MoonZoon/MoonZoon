#[cfg(feature = "connection")]
mod connection;

pub mod console;
mod cow_str;
mod css_property_name;
mod dom;
mod either;
mod element;
mod futures_signals_ext;
mod not;
mod router;
mod style;
mod task;
mod timer;
mod viewport;

pub use cow_str::{IntoCowStr, IntoOptionCowStr};
pub use dom::{document, history, window};
pub use dominator::{self, events, traits::StaticEvent, Dom, DomBuilder};
pub use either::{Either, IntoEither};
pub use element::*;
pub use futures_signals::{
    self, map_mut, map_ref,
    signal::{Mutable, Signal, SignalExt},
    signal_map::{MutableBTreeMap, SignalMap, SignalMapExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
};
pub use futures_signals_ext::{MutableExt, MutableVecExt, SignalExtMapBool};
pub use js_sys::{self, Reflect};
pub use not::not;
pub use paste;
pub use pin_project::pin_project;
pub use router::{FromRouteSegments, RouteSegment, Router, decode_uri_component, encode_uri_component};
pub use route_macro::route;
pub use send_wrapper::SendWrapper;
pub use std::future::Future;
pub use style::*;
pub use task::Task;
pub use timer::Timer;
pub use viewport::{Scene, Viewport};
pub use wasm_bindgen::{self, prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
pub use wasm_bindgen_futures::JsFuture;
pub use web_sys;

#[cfg(feature = "connection")]
pub use connection::{Connection, SendUpMsgError};

#[cfg(feature = "moonlight")]
pub use moonlight::{self, AuthToken, CorId};

#[cfg(feature = "panic_hook")]
pub use console_error_panic_hook;

#[cfg(feature = "static_ref")]
pub use once_cell;
#[cfg(feature = "static_ref")]
pub use static_ref_macro::static_ref;

#[cfg(feature = "clone")]
pub use enclose::enc as clone;

#[cfg(feature = "apply")]
pub use apply::{Also, Apply};

#[cfg(feature = "small_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(feature = "fast_alloc")]
compile_error!("Do you know a fast allocator working in Wasm?");

// #[cfg(feature = "tracing_alloc")]
// #[global_allocator]
// static GLOBAL_ALLOCATOR: wasm_tracing_allocator::WasmTracingAllocator<std::alloc::System> = wasm_tracing_allocator::WasmTracingAllocator(std::alloc::System);

#[cfg(feature = "fmt")]
pub use lexical::{self, NumberFormat, WriteFloatOptions, WriteIntegerOptions};
#[cfg(feature = "fmt")]
pub use ufmt::{self, uDebug, uDisplay, uWrite, uwrite, uwriteln};

#[cfg(not(feature = "fmt"))]
pub use std::format;

#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! format {
    ($($arg:tt)*) => {{
        let mut text = String::new();
        $crate::ufmt::uwrite!(&mut text, $($arg)*).unwrap_throw();
        text
    }}
}

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

pub fn start_app<'a, E: Element>(
    browser_element_id: impl Into<Option<&'a str>>,
    view_root: impl FnOnce() -> E,
) {
    #[cfg(feature = "panic_hook")]
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let parent = browser_element_id
        .into()
        // @TODO we need a better error message
        .map(dominator::get_id)
        .unwrap_or_else(|| dominator::body().unchecked_into());

    dominator::append_dom(&parent, view_root().into_raw_element().into_dom());
}
