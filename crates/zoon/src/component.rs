use crate::element::{Element, IntoElement};
use crate::render_context::RenderContext;
use crate::component_call_stack::__ComponentCallStack;
use crate::tracked_call_stack::__TrackedCallStack;
use crate::tracked_call::{__TrackedCall, TrackedCallId};
use crate::render;
use crate::runtime::C_VARS;
use std::rc::Rc;
use std::cell::RefCell;
use crate::log;

pub fn rerender_component(id: TrackedCallId) {
    let component_creator = C_VARS.with(|c_vars| {
        c_vars
            .borrow()
            .data::<__ComponentData>(&id)
            .clone()
    });
    if let Some(rcx) = component_creator.rcx {
        let parent_call_from_macro = component_creator.parent_call_from_macro.unwrap();
        parent_call_from_macro.borrow_mut().selected_index = component_creator.parent_selected_index_from_macro.unwrap();

        __TrackedCallStack::push(parent_call_from_macro);
        let mut cmp = (component_creator.creator)();
        __TrackedCallStack::pop();

        let parent_call = component_creator.parent_call.unwrap();
        parent_call.borrow_mut().selected_index = component_creator.parent_selected_index.unwrap() - 1;
       
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

            C_VARS.with(move |c_vars| {
                let mut c_vars = c_vars.borrow_mut();
    
                let mut component_data = c_vars.remove::<__ComponentData>(&component_data_id);
    
                component_data.rcx = Some(rcx);
                component_data.parent_call = __TrackedCallStack::parent();
                component_data.parent_selected_index = component_data.parent_call.as_ref().map(|parent| {
                    parent.borrow().selected_index
                });
    
                c_vars.insert(component_data_id, component_data);
            });
            
            rcx.component_id = Some(component_data_id);
            if let Some(element) = &mut self.element {
                element.render(rcx);
            }

            __ComponentCallStack::pop();

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

#[derive(Clone)]
pub struct __ComponentData {
    pub creator: Rc<dyn Fn() -> Cmp>,
    pub rcx: Option<RenderContext>,
    pub parent_call_from_macro: Option<Rc<RefCell<__TrackedCall>>>,
    pub parent_selected_index_from_macro: Option<usize>,
    pub parent_call: Option<Rc<RefCell<__TrackedCall>>>,
    pub parent_selected_index: Option<usize>, 
    pub children: Vec<ComponentChild>,
    pub should_call_creator: bool,
}

// ------ __ComponentChild ------

#[derive(Clone, Copy)]
pub enum ComponentChild {
    ElVar(TrackedCallId),
    CmpVar(TrackedCallId),
    Cmp(TrackedCallId),
}
