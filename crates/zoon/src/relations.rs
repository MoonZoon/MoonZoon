use crate::{block_call_stack::{__Block, __BlockCallStack}};
use std::collections::HashSet;
use crate::runtime::{RELATIONS, CACHES};

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
                __Block::SVar(_) => ()
            }
        }
    }

    pub fn remove_dependencies(block: &__Block) {
        RELATIONS.with(|relations| {
            relations
                .borrow_mut()
                .do_remove_dependencies(block)
        })
    }

    pub fn refresh_dependents(block: &__Block) {
        let dependents = RELATIONS.with(|relations| {
            relations
                .borrow_mut()
                .do_get_dependents(block)
        });
        for block in dependents {
            match block {
                __Block::Cache(id) => {
                    let creator = CACHES.with(|caches| {
                        caches
                            .borrow_mut()
                            .remove_return_creator(id)
                    });
                    if let Some(creator) = creator {
                        let data = creator();
                        CACHES.with(|caches| {
                            caches
                                .borrow_mut()
                                .insert(id, data, creator)
                        });
                        __Relations::refresh_dependents(&__Block::Cache(id));
                    }
                }
                __Block::SVar(_) => ()
            }
        }
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
