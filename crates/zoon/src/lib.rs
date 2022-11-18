#[cfg(feature = "connection")]
mod connection;

#[cfg(feature = "routing")]
pub mod routing;

#[cfg(feature = "web_storage")]
pub mod web_storage;

mod animation;
mod class_id;
pub mod console;
mod cow_str;
mod css_property;
pub mod dom;
mod dom_builder_ext;
mod either;
mod element;
mod event_options;
pub mod events_extra;
mod futures_signals_ext;
mod index_generator;
mod into_f64;
mod monotonic_ids;
mod mutable;
mod mutable_vec;
mod not;
mod resize_observer;
mod style;
mod task;
mod timer;
mod viewport;

pub use animation::*;
pub use class_id::ClassId;
pub use cow_str::{IntoCowStr, IntoOptionCowStr};
pub use css_property::VENDOR_PREFIXES;
pub use dom::{document, history, load_script, load_stylesheet, window};
pub use dom_builder_ext::DomBuilderExt;
pub use dominator::{self, events, traits::StaticEvent, Dom, DomBuilder};
pub use either::{Either, IntoEither};
pub use element::*;
pub use event_options::EventOptions;
pub use futures_channel::{self, oneshot};
pub use futures_signals::{
    self, map_mut, map_ref,
    signal::{
        self, always, channel, Broadcaster, MutableSignal, ReadOnlyMutable, Receiver, Sender,
        Signal, SignalExt, SignalStream,
    },
    signal_map::{MutableBTreeMap, MutableSignalMap, SignalMap, SignalMapExt},
    signal_vec::{always as always_vec, MutableSignalVec, SignalVec, SignalVecExt},
};
pub use futures_signals_ext::{SignalExtBool, SignalExtExt, SignalExtOption};
pub use futures_util::{self, future, FutureExt, Stream, StreamExt};
pub use gensym::gensym;
pub use hsluv::{hsluv, HSLuv};
pub use index_generator::IndexGenerator;
pub use into_f64::IntoF64;
pub use js_sys::{self, JsString, Reflect};
pub use lang::Lang;
pub use monotonic_ids::MonotonicIds;
pub use mutable::Mutable;
pub use mutable_vec::MutableVec;
pub use not::not;
pub use num_traits;
pub use once_cell;
pub use paste::paste;
pub use pin_project::pin_project;
pub use resize_observer::ResizeObserver;
pub use send_wrapper::SendWrapper;
pub use std::future::Future;
pub use strum;
pub use style::*;
pub use task::{Task, TaskHandle};
pub use timer::Timer;
pub use viewport::{Scene, Viewport};
pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use wasm_bindgen_futures::{self, JsFuture, future_to_promise};
pub use web_sys;

#[cfg(feature = "connection")]
pub use connection::{
    Connection, ExchangeMsgsError, MsgOptions, ReceiveDownMsgError, SendUpMsgError,
};

#[cfg(feature = "routing")]
pub use route_macro::route;
#[cfg(feature = "routing")]
pub use routing::{FromRouteSegments, RouteSegment, Router};

#[cfg(feature = "moonlight")]
pub use moonlight::{self, AuthToken, CorId, EntityId, Wrapper};

#[cfg(feature = "panic_hook")]
pub use console_error_panic_hook;

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

#[cfg(feature = "web_storage")]
pub use web_storage::{local_storage, session_storage, LocalStorage, SessionStorage, WebStorage};

#[cfg(feature = "serde_json")]
pub use serde_json;

#[cfg(feature = "serde-lite")]
pub use serde_lite::{self, Deserialize, Serialize};

#[cfg(feature = "serde")]
pub use serde::{self, de::DeserializeOwned, Deserialize, Serialize};

#[cfg(feature = "thiserror")]
pub use thiserror;

#[cfg(feature = "chrono")]
pub use chrono::{self, prelude::*, Duration};

#[cfg(feature = "serde-wasm-bindgen")]
pub use serde_wasm_bindgen;

// -- public_url --

pub static PUBLIC_URL: &str = "/_api/public/";

pub fn public_url(path: impl AsRef<str>) -> String {
    format!("{PUBLIC_URL}{}", path.as_ref())
}

#[macro_export]
macro_rules! public_url {
    ($($arg:tt)*) => {{
        $crate::public_url($crate::format!($($arg)*))
    }}
}

// ------ format! ------

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

// ------ make_flags! ------
pub trait FlagSet {}
pub trait FlagNotSet {}

#[macro_export]
macro_rules! make_flags {
    ($($flag:ident),*) => {
        $($crate::paste!{
            #[derive(Default)]
            pub struct [<$flag FlagSet>];
            #[derive(Default)]
            pub struct [<$flag FlagNotSet>];
            impl $crate::FlagSet for [<$flag FlagSet>] {}
            impl $crate::FlagNotSet for [<$flag FlagNotSet>] {}
        })*
    }
}

// ------ run_once! ------

#[macro_export]
macro_rules! run_once {
    ($f:expr) => {
        $crate::gensym! { $crate::run_once!($f) }
    };
    ($random_ident:ident, $f:expr) => {
        $crate::paste! {{
            static [<RUN_ONCE $random_ident:snake:upper>]: std::sync::Once = std::sync::Once::new();
            [<RUN_ONCE $random_ident:snake:upper>].call_once($f);
        }}
    };
}

// ------ element_vec! ------

#[macro_export]
macro_rules! element_vec {
    (  $($element:expr $(,)?)* ) => {
        {
            let mut elements = Vec::new();
            $(
                if let Some(element) = $element.into_option_element() {
                    elements.push(element.into_raw_element());
                }
            )*
            elements
        }
    };
}

// ------ start_app ------

pub fn start_app<'a, E: Element, I: IntoIterator<Item = E>>(
    browser_element_id: impl Into<Option<&'a str>>,
    view_root: impl FnOnce() -> I,
) {
    #[cfg(feature = "panic_hook")]
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let parent = browser_element_id
        .into()
        // @TODO we need a better error message
        .map(dominator::get_id)
        .unwrap_or_else(|| dominator::body().unchecked_into());

    for element in view_root() {
        dominator::append_dom(&parent, element.into_raw_element().into_dom());
    }
}
