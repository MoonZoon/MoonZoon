use std::{collections::{hash_map::DefaultHasher}, hash::Hash};
use std::hash::Hasher;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::log;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrackedCallId {
    pub hash: u64,
    pub index: Option<usize>,
    pub parent_hash: Option<u64>,
}

impl TrackedCallId {
    pub fn current() -> Self {
        __TrackedCallStack::last_to_tracked_call_id().expect("no current TrackedCalledId")
    }

    pub fn from_call(call: &__TrackedCall) -> Self {
        Self {
            hash: call.hash,
            index: call.index,
            parent_hash: call.parent_hash,
        }
    }
}

#[derive(Debug)]
pub struct __TrackedCall {
    pub hash: u64,
    // pub call_site: CallSite,
    pub index: Option<usize>,
    pub parent_hash: Option<u64>,
    pub selected_index: usize,
}


impl __TrackedCall {
    pub fn create() -> __TrackedCall {
        // let call_site = CallSite::here();

        let index = __TrackedCallStack::increment_last_selected_index();
        let parent_hash = __TrackedCallStack::last_hash();

        let hash = {
            let mut hasher = DefaultHasher::new();
            if let Some(parent_hash) = parent_hash {
                hasher.write_u64(parent_hash);
            }
            if let Some(index) = index {
                hasher.write_usize(index);
            }
            hasher.finish()
        };

        Self {
            hash,
            index,
            parent_hash,
            selected_index: 0,
        }
    }
}


// #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
// pub struct CallSite {
//     location: usize
// } 

// impl CallSite {

//     pub fn here() -> Self {
//         // if let Some(call_site) = SUBSTITUTED_CALL_SITE.with(|call_site| call_site.take()) {
//         //     log!("xxxxxxxxxxxxxxxxxxxxxxxxxxx");
//         //     return call_site;
//         // }
//         let raw_location = Location::caller();
//         // log!("Location: {:#?}", raw_location);
//         Self::from(raw_location)
//     }

// }

// impl From<&'static Location<'static>> for CallSite {
//     fn from(location: &'static Location<'static>) -> Self {
//         Self {
//             // the pointer value for a given location is enough to differentiate it from all others
//             location: location as *const _ as usize,
//         }
//     }
// }
