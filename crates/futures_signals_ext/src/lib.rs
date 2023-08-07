use ::pin_project::pin_project;
pub use futures_signals::{
    self, map_mut, map_ref,
    signal::{
        self, always, channel, Broadcaster, Mutable, MutableSignal, ReadOnlyMutable, Receiver,
        Sender, Signal, SignalExt, SignalStream,
    },
    signal_map::{
        always as always_map, MapDiff, MutableBTreeMap, MutableSignalMap, SignalMap, SignalMapExt,
    },
    signal_vec::{
        always as always_vec, MutableSignalVec, MutableVec, SignalVec, SignalVecExt, VecDiff,
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
pub use signal_either::{IntoSignalEither, SignalEither};

mod mutable_ext;
pub use mutable_ext::MutableExt;

mod mutable_vec_ext;
pub use mutable_vec_ext::MutableVecExt;
