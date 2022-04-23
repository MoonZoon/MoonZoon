use crate::css_property::{CssPropertyName, CssPropertyValue};
use crate::*;
use dominator::traits::AsStr;
use std::borrow::Cow;

// ------ ------
//  IntoCowStr
// ------ ------

pub trait IntoCowStr<'a> {
    fn into_cow_str(self) -> Cow<'a, str>;

    fn into_cow_str_wrapper(self) -> CowStrWrapper<'a>
    where
        Self: Sized,
    {
        CowStrWrapper(self.into_cow_str())
    }
}

//-- impls --

impl<'a> IntoCowStr<'a> for HSLuv {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.to_string().into()
    }
}

impl<'a> IntoCowStr<'a> for String {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> IntoCowStr<'a> for &'a String {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> IntoCowStr<'a> for &'a str {
    fn into_cow_str(self) -> Cow<'a, str> {
        self.into()
    }
}

impl<'a> IntoCowStr<'a> for Cow<'a, str> {
    fn into_cow_str(self) -> Cow<'a, str> {
        self
    }
}

macro_rules! make_into_cow_str_impls {
    ($($type:ty),*) => (
        $(
        impl<'a> IntoCowStr<'a> for $type {
            fn into_cow_str(self) -> Cow<'a, str> {
                self.to_string().into()
            }
        }
        )*
    )
}
make_into_cow_str_impls!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

// -------- --------
// IntoOptionCowStr
// -------- --------

pub trait IntoOptionCowStr<'a>: BoxedIntoOptionCowStr<'a> {
    fn into_option_cow_str(self) -> Option<Cow<'a, str>>;

    fn into_option_cow_str_wrapper(self) -> Option<CowStrWrapper<'a>>
    where
        Self: Sized,
    {
        self.into_option_cow_str()
            .map(|this| this.into_cow_str_wrapper())
    }
}

// "Helper trait" to allow implementation `IntoOptionCowStr` for `Box<dyn IntoOptionCowStr>`.
// Leveraging this "trick": https://stackoverflow.com/a/30353928
// @TODO remove once Rust feature `unsized_fn_params` is stable.
pub trait BoxedIntoOptionCowStr<'a> {
    fn boxed_into_option_cow_str(self: Box<Self>) -> Option<Cow<'a, str>>;
}

impl<'a, T: IntoOptionCowStr<'a>> BoxedIntoOptionCowStr<'a> for T {
    fn boxed_into_option_cow_str(self: Box<Self>) -> Option<Cow<'a, str>> {
        self.into_option_cow_str()
    }
}

// -- impls --

impl<'a, T: IntoCowStr<'a>> IntoOptionCowStr<'a> for T {
    fn into_option_cow_str(self) -> Option<Cow<'a, str>> {
        Some(self.into_cow_str())
    }
}

impl<'a, T: IntoCowStr<'a>> IntoOptionCowStr<'a> for Option<T> {
    fn into_option_cow_str(self) -> Option<Cow<'a, str>> {
        self.map(|this| this.into_cow_str())
    }
}

impl<'a> IntoOptionCowStr<'a> for Box<dyn IntoOptionCowStr<'a>> {
    fn into_option_cow_str(self) -> Option<Cow<'a, str>> {
        self.boxed_into_option_cow_str()
    }
}

// ------ ------
// CowStrWrapper
// ------ ------

pub struct CowStrWrapper<'a>(Cow<'a, str>);

//-- impls --

impl<'a> CowStrWrapper<'a> {
    pub fn into_css_property_name(self) -> CssPropertyName<'a> {
        CssPropertyName::new(self.0)
    }
}

impl<'a> CowStrWrapper<'a> {
    pub fn into_css_property_value(self) -> CssPropertyValue<'a> {
        CssPropertyValue::new(self.0)
    }
}

impl AsStr for CowStrWrapper<'_> {
    fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'a> From<CowStrWrapper<'a>> for JsValue {
    fn from(cow_str_wrapper: CowStrWrapper<'a>) -> Self {
        #[allow(deprecated)]
        JsValue::from_str(cow_str_wrapper.as_str())
    }
}
