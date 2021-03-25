use crate::{__Relations, element::{Element, IntoElement}};
use crate::render_context::RenderContext;
use crate::component_call_stack::__ComponentCallStack;
use crate::block_call_stack::__Block;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::tracked_call::{__TrackedCall, TrackedCallId};
use crate::render;
use crate::runtime::{C_VARS, EL_VARS, CMP_VARS};
use std::{mem, rc::Rc};
use std::cell::RefCell;
use std::collections::HashSet;
use crate::log;

pub fn rerender_component(id: TrackedCallId) {
    let component_data = C_VARS.with(|c_vars| {
        c_vars
            .borrow_mut()
            .remove::<__ComponentData>(&id)
    });

    let parent_call_from_macro = component_data.parent_call_from_macro.clone().unwrap();
    parent_call_from_macro.borrow_mut().selected_index = component_data.parent_selected_index_from_macro.unwrap();

    let parent_call = component_data.parent_call.clone().unwrap();
    parent_call.borrow_mut().selected_index = component_data.parent_selected_index.unwrap() - 1;

    let creator = Rc::clone(&component_data.creator);

    let rcx = component_data.rcx;

    C_VARS.with(|c_vars| {
        c_vars
            .borrow_mut()
            .insert(id, component_data)
    });

    if let Some(rcx) = rcx {
        __TrackedCallStack::push(parent_call_from_macro);
        let mut cmp = creator();
        __TrackedCallStack::pop();

        __TrackedCallStack::push(parent_call);
        cmp.render(rcx);
        __TrackedCallStack::pop();
    }


}

// ------ Cmp ------

pub struct Cmp {
    pub element: Option<Box<dyn Element>>,
    pub component_data_id: Option<TrackedCallId>,
} 

impl<'a> Element for Cmp {
    #[render]
    fn render(&mut self, mut rcx: RenderContext) {
        if let Some(component_data_id) = self.component_data_id {

            __ComponentCallStack::push(component_data_id);

            let children_to_remove = C_VARS.with(move |c_vars| {
                let mut c_vars = c_vars.borrow_mut();
    
                let mut component_data = c_vars.remove::<__ComponentData>(&component_data_id);
                component_data.rcx = Some(rcx);
                component_data.parent_call = __TrackedCallStack::parent();
                component_data.parent_selected_index = component_data.parent_call.as_ref().map(|parent| {
                    parent.borrow().selected_index
                });
    
                let children_to_remove = component_data.children_to_remove();
                c_vars.insert(component_data_id, component_data);
                children_to_remove
            });
            
            rcx.component_id = Some(component_data_id);
            if let Some(element) = &mut self.element {
                element.render(rcx);
            }

            __ComponentCallStack::pop();

            remove_children(children_to_remove);
        }
    }
}

fn remove_children(children: HashSet<ComponentChild>) {
    if children.is_empty() {
        return
    }
    // log!("{}", children.len());
    for child in children {
        match child {
            ComponentChild::ElVar(tracked_call_id) => {
                // log!("ElVar");
                let _ = EL_VARS.with(|el_vars| {
                    el_vars
                        .borrow_mut()
                        .el_vars
                        .remove(&tracked_call_id)
                });
            }
            ComponentChild::CmpVar(tracked_call_id) => {
                // log!("CmpVar");
                let _ = CMP_VARS.with(|cmp_vars| {
                    cmp_vars
                        .borrow_mut()
                        .cmp_vars
                        .remove(&tracked_call_id)
                });
            }
            ComponentChild::Cmp(tracked_call_id) => {
                // log!("Cmp");
                let _ = C_VARS.with(|c_vars| {
                    c_vars
                        .borrow_mut()
                        .c_vars
                        .remove(&tracked_call_id)
                });
                __Relations::remove_dependencies(&__Block::Cmp(tracked_call_id));
            }
        }
    }
}

// ------ IntoComponent ------

pub trait IntoComponent<'a> {
    fn into_component(self) -> Cmp;
}

impl<'a, T: 'static + IntoElement<'static>> IntoComponent<'a> for T {
    fn into_component(self) -> Cmp {
        Cmp {
            element: Some(Box::new(self.into_element())),
            component_data_id: None,
        }
        // Cmp::Element(Box::new(self.into_element()))
    }
}

// ------ __ComponentBody ------

pub struct __ComponentData {
    pub creator: Rc<dyn Fn() -> Cmp>,
    pub rcx: Option<RenderContext>,
    pub parent_call_from_macro: Option<Rc<RefCell<__TrackedCall>>>,
    pub parent_selected_index_from_macro: Option<usize>,
    pub parent_call: Option<Rc<RefCell<__TrackedCall>>>,
    pub parent_selected_index: Option<usize>, 
    pub previous_children: HashSet<ComponentChild>,
    pub children: HashSet<ComponentChild>,
    pub should_call_creator: bool,
}

impl __ComponentData {
    fn children_to_remove(&mut self) -> HashSet<ComponentChild>  {
        // @TODO optimize 
        let previous_children = mem::take(&mut self.previous_children);
        let children_to_remove = previous_children.difference(&self.children).cloned().collect();
        mem::swap(&mut self.previous_children, &mut self.children);
        children_to_remove
    }
}

impl Drop for __ComponentData {
    fn drop(&mut self) {
        // log!("DROP!");
        let children_to_remove = mem::take(&mut self.children);
        remove_children(children_to_remove);
    }
}

// ------ __ComponentChild ------

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComponentChild {
    ElVar(TrackedCallId),
    CmpVar(TrackedCallId),
    Cmp(TrackedCallId),
}
