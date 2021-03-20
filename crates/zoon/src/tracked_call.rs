use std::{collections::{HashMap, hash_map::DefaultHasher}, hash::Hash};
use std::hash::Hasher;
use std::panic::Location;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::runtime::TRACKED_CALLS;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct __TrackedCallId {
    pub hash: u64,
}

impl __TrackedCallId {

    pub fn get_or_create() -> Self {
        let tracked_call = TrackedCall::new();

        let parent = tracked_call.parent;
        let call_site = tracked_call.call_site;
        let id = tracked_call.id;

        let id_exists = TRACKED_CALLS.with(|tracked_calls| {
            tracked_calls.borrow().contains_id(&id)
        });

        if !id_exists {
            TRACKED_CALLS.with(|tracked_calls| {
                let mut tracked_calls = tracked_calls.borrow_mut();
                tracked_calls.insert(id, tracked_call);

                if let Some(parent) = parent {
                    let children = &mut tracked_calls.get_mut(&parent).unwrap().children;
                    if let Some((selected_child_index, children)) = children.get_mut(&call_site) {
                        *selected_child_index += 1;
                        children.push(id);
                    } else {
                        children.insert(call_site, (1, vec![id]));
                    }
                }
            })
        } else {
            TRACKED_CALLS.with(|tracked_calls| {
                let mut tracked_calls = tracked_calls.borrow_mut();
                if let Some(parent) = parent {
                    let children = &mut tracked_calls.get_mut(&parent).unwrap().children;
                    let selected_child_index = &mut children.get_mut(&call_site).unwrap().0;
                    *selected_child_index += 1;
                }
            })
        }

        id
    }

    pub fn current() -> Self {
        __TrackedCallStack::last().expect("no current TrackedCallId")
    }
}

pub type SelectedChildIndex = usize;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrackedCall {
    id: __TrackedCallId,
    call_site: CallSite,
    index: Option<usize>,
    parent: Option<__TrackedCallId>,
    pub children: HashMap<CallSite, (SelectedChildIndex, Vec<__TrackedCallId>)>,
}

impl TrackedCall {
    #[track_caller]
    pub fn new() -> Self {
        let call_site = CallSite::here();

        let parent_tracked_call = __TrackedCallStack::last();
        let parent_data = parent_tracked_call.and_then(|parent| {
            TRACKED_CALLS.with(|tracked_calls| {
                tracked_calls.borrow().get(&parent).map(|tracked_call| {
                    let selected_child_index = tracked_call
                        .children
                        .get(&call_site)
                        .map(|(selected_child_index, _)| *selected_child_index)
                        .unwrap_or_default();
                    (tracked_call.id, selected_child_index)
                })
            })
        });

        let parent = parent_data.as_ref().map(|parent| parent.0);
        let index = parent_data.map(|parent| parent.1);
        
        let mut hasher = DefaultHasher::new();
        if let Some(parent) = parent {
            hasher.write_u64(parent.hash);
        }
        if let Some(index) = index {
            hasher.write_usize(index);
        }
        hasher.write_usize(call_site.location);
        let id = __TrackedCallId {
            hash: hasher.finish(),
        };

        Self {
            id,
            call_site,
            index,
            parent,
            children: HashMap::new(),
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
        Self::from(Location::caller())
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
