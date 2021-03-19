use crate::runtime::{TRACKED_CALL_STACK, TRACKED_CALLS};
use crate::tracked_call::__TrackedCallId;

#[derive(Default)]
pub struct __TrackedCallStack(Vec<__TrackedCallId>);

impl __TrackedCallStack {
    pub fn push(tracked_call: __TrackedCallId) {
        TRACKED_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .push(tracked_call)
        });
    }
    
    pub fn pop() -> Option<__TrackedCallId> {
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

    pub fn last() -> Option<__TrackedCallId> {
        TRACKED_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow()
                .0
                .last()
                .cloned()
        })
    }
}
