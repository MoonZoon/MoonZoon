use crate::state::State;
use crate::Node;

// -- modules --

pub mod button;
pub use button::Button;

pub mod column;
pub use column::Column;

pub mod el;
pub use el::El;

pub mod row;
pub use row::Row;

pub mod text;
pub use text::Text;

// ------ component_macro ------

#[macro_export]
macro_rules! component_macro {
    ( $name:tt, $component:expr ) => {
        // Replace $d with $ in the inner macro.
        $crate::with_dollar_sign! {
            ($d:tt) => {
                #[macro_export]
                macro_rules! $name {
                    ( $d ($d attribute:expr),* $d (,)?) => {
                        {
                            #[allow(unused_mut)]
                            let mut component = $component;
                            $d ( $d attribute.apply_to_component(&mut component); )*
                            component
                        }
                    }
                }
            }
        }
    }
}

// ------ Component ------

pub trait Component {
    fn new() -> Self 
        where Self: Default
    {
        Self::default()
    }

    fn with(mut self, attribute: impl ApplyToComponent<Self>) -> Self
        where Self: Sized
    {
        attribute.apply_to_component(&mut self);
        self
    }

    fn with_iter(mut self, attribute: impl ApplyToComponentForIterator<Self>) -> Self
        where Self: Sized
    {
        attribute.apply_to_component(&mut self);
        self
    }

    fn render(&mut self, rcx: RenderContext);
}

// ------ RenderContext ------

#[derive(Copy, Clone)]
pub struct RenderContext {
    pub index: u32,
    pub state_node: State<Node>,
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
}

// ------ ApplyToComponent ------

pub trait ApplyToComponent<T: Component> {
    fn apply_to_component(self, component: &mut T);
}

impl<T: Component, ATTR: ApplyToComponent<T>> ApplyToComponent<T> for Option<ATTR> {
    fn apply_to_component(self, component: &mut T) {
        if let Some(attribute) = self {
            attribute.apply_to_component(component);
        }
    }
}

impl<T: Component, ATTR: ApplyToComponent<T>> ApplyToComponent<T> for Vec<ATTR> {
    fn apply_to_component(self, component: &mut T) {
        for attribute in self {
            attribute.apply_to_component(component);
        }
    }
}

// -- ApplyToComponentForIterator --

pub trait ApplyToComponentForIterator<T: Component> {
    fn apply_to_component(self, component: &mut T);
}

impl<T, ATTR, I> ApplyToComponentForIterator<T> for I 
    where 
        T: Component, 
        ATTR: ApplyToComponent<T>, 
        I: Iterator<Item = ATTR>
{
    fn apply_to_component(self, component: &mut T) {
        for attribute in self {
            attribute.apply_to_component(component);
        }
    }
}

// ------ IntoComponent ------

pub trait IntoComponent<'a> {
    type CMP: Component;
    fn into_component(self) -> Self::CMP; 
}

impl<'a, T: Component> IntoComponent<'a> for T {
    type CMP = T;
    fn into_component(self) -> Self::CMP {
        self
    }
}

impl<'a> IntoComponent<'a> for String {
    type CMP = Text<'a>;
    fn into_component(self) -> Self::CMP {
        Text::default().with(self)
    }
}

impl<'a> IntoComponent<'a> for &'a str {
    type CMP = Text<'a>;
    fn into_component(self) -> Self::CMP {
        Text::default().with(self)
    }
}




