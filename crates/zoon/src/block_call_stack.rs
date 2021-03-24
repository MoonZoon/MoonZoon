use crate::runtime::BLOCK_CALL_STACK;
use crate::tracked_call::TrackedCallId;

pub type Id = u128;

#[derive(Default)]
pub struct __BlockCallStack(Vec<__Block>);

impl __BlockCallStack {
    pub fn push(block: __Block) {
        BLOCK_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .push(block)
        });
    }
    
    pub fn pop() -> Option<__Block> {
        BLOCK_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .pop()
        })
    }

    pub fn last() -> Option<__Block> {
        BLOCK_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow()
                .0
                .last()
                .cloned()
        })
    }
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub enum __Block {
    SVar(Id),
    Cache(Id),
    Cmp(TrackedCallId),
    CmpVar(TrackedCallId),
}
