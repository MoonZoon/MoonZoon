use crate::element::{Element, RenderContext, IntoElement};
use crate::tracked_call_stack::__TrackedCallStack;
use crate::tracked_call::__TrackedCall;
use crate::render;

// ------ Cmp ------

pub enum Cmp<'a> {
    Element(Box<dyn Element + 'a>),
    NoChange,
} 

impl<'a> Element for Cmp<'a> {
    #[render]
    fn render(&mut self, rcx: RenderContext) {
        match self {
            Cmp::Element(element) => {
                element.render(rcx)
            }
            Cmp::NoChange => {
                ()
            }
        }
    }
}

// ------ IntoComponent ------

pub trait IntoComponent<'a> {
    fn into_component(self) -> Cmp<'a>;
}

impl<'a, T: 'a + IntoElement<'a>> IntoComponent<'a> for T {
    fn into_component(self) -> Cmp<'a> {
        Cmp::Element(Box::new(self.into_element()))
    }
}
