use crate::runtime::COMPONENT_CALL_STACK;
use crate::tracked_call::TrackedCallId;

#[derive(Default)]
pub struct __ComponentCallStack(Vec<TrackedCallId>);

impl __ComponentCallStack {
    pub fn push(component_id: TrackedCallId) {
        COMPONENT_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .push(component_id)
        });
    }
    
    pub fn pop() -> Option<TrackedCallId> {
        COMPONENT_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .pop()
        })
    }

    pub fn last() -> Option<TrackedCallId> {
        COMPONENT_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow()
                .0
                .last()
                .cloned()
        })
    }
}
