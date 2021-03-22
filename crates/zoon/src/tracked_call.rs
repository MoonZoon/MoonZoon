use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::Hash};
use std::hash::Hasher;
use std::panic::Location;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::runtime::SUBSTITUTED_CALL_SITE;
use crate::log;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TrackedCallId {
    pub hash: u64,
    pub call_site: CallSite,
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
            call_site: call.call_site,
            index: call.index,
            parent_hash: call.parent_hash,
        }
    }
}

#[derive(Debug)]
pub struct __TrackedCall {
    pub hash: u64,
    pub call_site: CallSite,
    pub index: Option<usize>,
    pub parent_hash: Option<u64>,
    pub selected_indices: HashMap<CallSite, usize>,
}


impl __TrackedCall {
    #[track_caller]
    pub fn create() -> __TrackedCall {
        let call_site = CallSite::here();

        let index = __TrackedCallStack::increment_last_selected_index(call_site);
        let parent_hash = __TrackedCallStack::last_hash();

        let hash = {
            let mut hasher = DefaultHasher::new();
            if let Some(parent_hash) = parent_hash {
                hasher.write_u64(parent_hash);
            }
            if let Some(index) = index {
                hasher.write_usize(index);
            }
            hasher.write_usize(call_site.location);
            hasher.finish()
        };

        Self {
            hash,
            call_site,
            index,
            parent_hash,
            selected_indices: HashMap::new(),
        }
    }
}


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct CallSite {
    location: usize
} 

impl CallSite {

    #[track_caller]
    pub fn here() -> Self {
        // if let Some(call_site) = SUBSTITUTED_CALL_SITE.with(|call_site| call_site.take()) {
        //     log!("xxxxxxxxxxxxxxxxxxxxxxxxxxx");
        //     return call_site;
        // }
        let raw_location = Location::caller();
        // log!("Location: {:#?}", raw_location);
        Self::from(raw_location)
    }

}

impl From<&'static Location<'static>> for CallSite {
    fn from(location: &'static Location<'static>) -> Self {
        Self {
            // the pointer value for a given location is enough to differentiate it from all others
            location: location as *const _ as usize,
        }
    }
}
