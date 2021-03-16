use crate::runtime::BLOCK_CALL_STACK;
use crate::log;

pub type Id = u128;

#[derive(Default)]
pub struct __BlockCallStack(Vec<__Block>);

impl __BlockCallStack {
    pub fn push(block: __Block) {
        log!("Push: {:#?}", block);
        BLOCK_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .push(block)
        });
    }
    
    pub fn pop() -> Option<__Block> {
        log!("Pop");
        BLOCK_CALL_STACK.with(|call_stack| {
            call_stack
                .borrow_mut()
                .0
                .pop()
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum __Block {
    SVar(Id),
    Cache(Id),
}
