use crate::{TrackedCallId, runtime::{TRACKED_CALL_STACK}};
use crate::tracked_call::__TrackedCall;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct __TrackedCallStack(Vec<Rc<RefCell<__TrackedCall>>>);

impl __TrackedCallStack {
    pub fn push(tracked_call: Rc<RefCell<__TrackedCall>>) {
        TRACKED_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .push(tracked_call);
        });
    }
    
    pub fn pop() -> Option<Rc<RefCell<__TrackedCall>>> {
        TRACKED_CALL_STACK.with(|call_stack| {
            let mut call = call_stack
                .borrow_mut()
                .0
                .pop();

            if let Some(call) = &mut call {
                let mut call = call.borrow_mut();
                call.selected_index = 0;
            }
            call
        })
    }

    pub fn increment_last_selected_index() -> Option<usize> {
        TRACKED_CALL_STACK.with(|call_stack| {
            let mut call_stack = call_stack.borrow_mut();
            let selected_index = &mut call_stack
                .0
                .last_mut()?
                .borrow_mut()
                .selected_index;
            
            *selected_index += 1;
            Some(*selected_index)
        })
    }

    pub fn last_hash() -> Option<u64> {
        TRACKED_CALL_STACK.with(|call_stack| {
            Some(call_stack
                .borrow()
                .0
                .last()?
                .borrow()
                .hash)
        })
    }

    pub fn last_to_tracked_call_id() -> Option<TrackedCallId> {
        TRACKED_CALL_STACK.with(|call_stack| {
            Some(TrackedCallId::from_call(
                &call_stack
                    .borrow()
                    .0
                    .last()?
                    .borrow()
            ))
        })
    }

    // pub fn last() -> Option<__TrackedCall> {
    //     TRACKED_CALL_STACK.with(|call_stack| {
    //         call_stack
    //             .borrow()
    //             .0
    //             .last()
    //             .cloned()
    //     })
    // }

    pub fn parent() -> Option<Rc<RefCell<__TrackedCall>>> {
        TRACKED_CALL_STACK.with(|call_stack| {
            let call_stack = &call_stack.borrow().0;
            call_stack.get(call_stack.len() - 2).cloned()
        })
    }

    pub fn grand_parent() -> Option<Rc<RefCell<__TrackedCall>>> {
        TRACKED_CALL_STACK.with(|call_stack| {
            let call_stack = &call_stack.borrow().0;
            call_stack.get(call_stack.len() - 3).cloned()
        })
    }

    pub fn clear() {
        TRACKED_CALL_STACK.with(|call_stack| {
            call_stack.borrow_mut().0.clear()
        })
    }
}
