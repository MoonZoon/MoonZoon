use crate::element::{Element, RenderContext, IntoElement};
use crate::tracked_call_stack::__TrackedCallStack;
use crate::tracked_call::{__TrackedCall, TrackedCallId};
use crate::render;
use crate::runtime::LVARS;
use std::rc::Rc;
use crate::log;

// ------ Cmp ------

pub struct Cmp<'a> {
    element: Box<dyn Element + 'a>,
    pub component_data_id: Option<TrackedCallId>,
    // NoChange,
} 

impl<'a> Element for Cmp<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        log!("CMP render: {:#?}", TrackedCallId::current());

        // let tracked_call_stack_last = __TrackedCallStack::parent();
        // log!("tracked_call_stack_last: {:#?}", tracked_call_stack_last);

        // log!("cmp render context: {:#?}", context);

        if let Some(component_data_id) = self.component_data_id {
            LVARS.with(move |l_vars| {
                let mut l_vars = l_vars.borrow_mut();

                let mut component_data = l_vars
                    .remove::<__ComponentData>(&component_data_id)
                    .unwrap();

                component_data.rcx = Some(rcx);
                component_data.current_tracked_call_id = Some(TrackedCallId::current());

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
    fn into_component(self) -> Cmp<'a>;
}

impl<'a, T: 'a + IntoElement<'a>> IntoComponent<'a> for T {
    fn into_component(self) -> Cmp<'a> {
        Cmp {
            element: Box::new(self.into_element()),
            component_data_id: None,
        }
        // Cmp::Element(Box::new(self.into_element()))
    }
}

// ------ __ComponentBody ------
#[derive(Clone)]
pub struct __ComponentData<'a> {
    pub creator: Rc<dyn Fn() -> Cmp<'a>>,
    pub tracked_call_stack_last: Option<__TrackedCall>,
    pub rcx: Option<RenderContext>,
    pub current_tracked_call_id: Option<TrackedCallId>,
}
