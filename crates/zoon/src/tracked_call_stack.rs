use crate::runtime::{TRACKED_CALL_STACK, TRACKED_CALLS};
use crate::tracked_call::TrackedCallId;

#[derive(Default)]
pub struct __TrackedCallStack(Vec<TrackedCallId>);

impl __TrackedCallStack {
    pub fn push(tracked_call: TrackedCallId) {
        TRACKED_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .push(tracked_call)
        });
    }
    
    pub fn pop() -> Option<TrackedCallId> {
        let id = TRACKED_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .pop()
        });

        if let Some(id) = id.as_ref() {
            TRACKED_CALLS.with(|tracked_calls| {
                tracked_calls.borrow_mut().reset_indices(id);
            });
        }

        id
    }

    pub fn last() -> Option<TrackedCallId> {
        TRACKED_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow()
                .0
                .last()
                .cloned()
        })
    }
}
