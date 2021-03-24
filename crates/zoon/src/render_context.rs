use crate::{TrackedCallId, el_var::ElVar};
use crate::Node;
use crate::component::rerender_component;
use crate::rerender;

// ------ RenderContext ------

#[derive(Copy, Clone, Debug)]
pub struct RenderContext {
    pub index: u32,
    pub node: ElVar<Node>,
    pub component_id: Option<TrackedCallId>,
}

impl RenderContext {
    pub fn inc_index(&mut self) -> &mut Self {
        self.index += 1;
        self
    } 

    pub fn reset_index(&mut self) -> &mut Self {
        self.index = 0;
        self
    } 

    pub fn rerender(&self) {
        if let Some(id) = self.component_id {
            return rerender_component(id)
        }
        rerender()
    }
}
