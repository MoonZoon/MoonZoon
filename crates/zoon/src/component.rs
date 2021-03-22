use crate::element::{Element, RenderContext, IntoElement};
use crate::tracked_call_stack::__TrackedCallStack;
use crate::tracked_call::{__TrackedCall, TrackedCallId};
use crate::render;
use crate::runtime::LVARS;
use std::rc::Rc;
use std::cell::RefCell;
use crate::log;

// ------ Cmp ------

pub struct Cmp {
    element: Box<dyn Element>,
    pub component_data_id: Option<TrackedCallId>,
    // NoChange,
} 

impl<'a> Element for Cmp {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        // log!("CMP render: {:#?}", TrackedCallId::current());

        // let aaa = __TrackedCallStack::last();
        // log!("CMP LAST: {:#?}", aaa);

        // let xxx = __TrackedCallStack::parent();
        // log!("CMP PARENT: {:#?}", xxx);

        // log!("cmp render context: {:#?}", context);

        if let Some(component_data_id) = self.component_data_id {
            LVARS.with(move |l_vars| {
                let mut l_vars = l_vars.borrow_mut();

                let mut component_data = l_vars
                    .remove::<__ComponentData>(&component_data_id)
                    .unwrap();

                component_data.rcx = Some(rcx);
                component_data.parent_call = __TrackedCallStack::parent();
                component_data.parent_selected_index = component_data.parent_call.as_ref().map(|parent| {
                    parent.borrow().selected_index
                });

                // log!("PARENTTTTT CMP RENDER: {:#?}", component_data.parent_call);

                l_vars.insert(component_data_id, component_data);

            });
        }
        self.element.render(rcx)
        // match self {
        //     Cmp::Element(element) => {
        //         element.render(rcx)
        //     }
        //     Cmp::NoChange => {
        //         ()
        //     }
        // }
    }
}

// ------ IntoComponent ------

pub trait IntoComponent<'a> {
    fn into_component(self) -> Cmp;
}

impl<'a, T: 'static + IntoElement<'static>> IntoComponent<'a> for T {
    fn into_component(self) -> Cmp {
        Cmp {
            element: Box::new(self.into_element()),
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
}
