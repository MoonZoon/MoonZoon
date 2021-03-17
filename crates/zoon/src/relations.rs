use crate::block_call_stack::{__Block, __BlockCallStack};
use std::collections::HashSet;
use crate::runtime::RELATIONS;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct Relation {
    block: __Block,
    dependency: __Block,  
}

#[derive(Default)]
pub struct __Relations(HashSet<Relation>);

impl __Relations {
    pub fn add_dependency(dependency: __Block) {
        if let Some(last_block) = __BlockCallStack::last() {
            match last_block {
                __Block::Cache(_) => {
                    Self::insert(last_block, dependency)
                }
                _ => ()
            }
        }
    }

    pub fn get_dependents(block: &__Block) -> Vec<__Block> {
        RELATIONS.with(|relations| {
            relations
                .borrow()
                .do_get_dependents(block)
        })
    }

    pub fn remove_dependencies(block: &__Block) {
        RELATIONS.with(|relations| {
            relations
                .borrow_mut()
                .do_remove_dependencies(block)
        })
    }

    fn insert(block: __Block, dependency: __Block) {
        RELATIONS.with(|relations| {
            relations
                .borrow_mut()
                .do_insert(block, dependency)
        })
    }
    
    fn do_get_dependents(&self, block: &__Block) -> Vec<__Block> {
        self.0.iter().filter_map(move |relation| {
            (&relation.dependency == block).then(|| relation.block)
        }).collect()
    }

    fn do_remove_dependencies(&mut self, block: &__Block) {
        self.0.retain(|relation| &relation.block != block);
    }

    fn do_insert(&mut self, block: __Block, dependency: __Block) {
        self.0.insert(Relation {
            block, dependency
        });
    }

}
