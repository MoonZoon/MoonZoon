use ::pin_project::pin_project;
pub use futures_signals::{
    self, map_mut, map_ref,
    signal::{
        self, always, channel, BoxSignal, Broadcaster, LocalBoxSignal, Mutable, MutableSignal,
        ReadOnlyMutable, Receiver, Sender, Signal, SignalExt, SignalStream,
    },
    signal_map::{
        always as always_map, BoxSignalMap, LocalBoxSignalMap, MapDiff, MutableBTreeMap,
        MutableSignalMap, SignalMap, SignalMapExt,
    },
    signal_vec::{
        always as always_vec, BoxSignalVec, LocalBoxSignalVec, MutableSignalVec, MutableVec,
        SignalVec, SignalVecExt, VecDiff,
    },
};
pub use futures_util::{self, future, Future, FutureExt, Stream, StreamExt};

mod signal_ext_bool;
pub use signal_ext_bool::SignalExtBool;

mod signal_ext_ext;
pub use signal_ext_ext::SignalExtExt;

mod signal_ext_option;
pub use signal_ext_option::SignalExtOption;

mod signal_map_ext_ext;
pub use signal_map_ext_ext::SignalMapExtExt;

mod signal_either;
pub use signal_either::SignalEither;

mod mutable_ext;
pub use mutable_ext::MutableExt;

mod mutable_vec_ext;
pub use mutable_vec_ext::MutableVecExt;

mod mutable_b_tree_map_ext;
pub use mutable_b_tree_map_ext::MutableBTreeMapExt;

mod map_diff_ext;
pub use map_diff_ext::MapDiffExt;

#[macro_export]
macro_rules! match_to_signal_cloned_option {
    ($expression:expr, $pattern:pat $(if $guard:expr)? => $mutable:expr $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => $crate::SignalEither::Left($mutable.signal_cloned().map(Some)),
            _ => $crate::SignalEither::Right($crate::always(None))
        }
    };
}

#[macro_export]
macro_rules! match_to_option {
    ($expression:expr, $pattern:pat $(if $guard:expr)? => $mutable:expr $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => Some($mutable),
            _ => None
        }
    };
}

#[macro_export]
macro_rules! match_to_signal_vec_cloned {
    ($expression:expr, $pattern:pat $(if $guard:expr)? => $mutable_vec:expr $(,)?) => {
        match $expression {
            $pattern $(if $guard)? => $crate::SignalEither::Left($mutable_vec.signal_vec_cloned()),
            _ => $crate::SignalEither::Right($crate::always_vec(vec![]))
        }
    };
}
