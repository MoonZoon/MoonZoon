use crate::render_context::RenderContext;

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

// ------ element_macro ------

#[macro_export]
macro_rules! element_macro {
    ( $name:tt, $element:expr ) => {
        // Replace $d with $ in the inner macro.
        $crate::with_dollar_sign! {
            ($d:tt) => {
                #[macro_export]
                macro_rules! $name {
                    ( $d ($d attribute:expr),* $d (,)?) => {
                        {
                            #[allow(unused_mut)]
                            let mut element = $element;
                            $d ( $d attribute.apply_to_element(&mut element); )*
                            element
                        }
                    }
                }
            }
        }
    }
}

// ------ Element ------

pub trait Element {
    fn new() -> Self 
        where Self: Default
    {
        Self::default()
    }

    fn with(mut self, attribute: impl ApplyToElement<Self>) -> Self
        where Self: Sized
    {
        attribute.apply_to_element(&mut self);
        self
    }

    fn with_iter(mut self, attribute: impl ApplyToElementForIterator<Self>) -> Self
        where Self: Sized
    {
        attribute.apply_to_element(&mut self);
        self
    }

    fn render(&mut self, rcx: RenderContext);
    
    fn force_render(&mut self, rcx: RenderContext) {
        self.render(rcx)
    }
}

// ------ ApplyToElement ------

pub trait ApplyToElement<T: Element> {
    fn apply_to_element(self, element: &mut T);
}

impl<T: Element, ATTR: ApplyToElement<T>> ApplyToElement<T> for Option<ATTR> {
    fn apply_to_element(self, element: &mut T) {
        if let Some(attribute) = self {
            attribute.apply_to_element(element);
        }
    }
}

impl<T: Element, ATTR: ApplyToElement<T>> ApplyToElement<T> for Vec<ATTR> {
    fn apply_to_element(self, element: &mut T) {
        for attribute in self {
            attribute.apply_to_element(element);
        }
    }
}

// -- ApplyToElementForIterator --

pub trait ApplyToElementForIterator<T: Element> {
    fn apply_to_element(self, element: &mut T);
}

impl<T, ATTR, I> ApplyToElementForIterator<T> for I 
    where 
        T: Element, 
        ATTR: ApplyToElement<T>, 
        I: Iterator<Item = ATTR>
{
    fn apply_to_element(self, element: &mut T) {
        for attribute in self {
            attribute.apply_to_element(element);
        }
    }
}

// ------ IntoElement ------

pub trait IntoElement<'a> {
    type EL: Element;
    fn into_element(self) -> Self::EL; 
}

impl<'a, T: Element> IntoElement<'a> for T {
    type EL = T;
    fn into_element(self) -> Self::EL {
        self
    }
}

impl<'a> IntoElement<'a> for String {
    type EL = Text<'a>;
    fn into_element(self) -> Self::EL {
        Text::default().with(self)
    }
}

impl<'a> IntoElement<'a> for &'a str {
    type EL = Text<'a>;
    fn into_element(self) -> Self::EL {
        Text::default().with(self)
    }
}




