use crate::dominator::traits::MultiStr;
use std::borrow::Cow;

static VENDOR_PREFIXES: [&'static str; 4] = [
    "-webkit-",
    "-moz-",
    "-o-",
    "-ms-",
];

pub struct CssPropertyName<'a>(Cow<'a, str>);

impl<'a> CssPropertyName<'a> {
    pub fn new(name: impl Into<Cow<'a, str>>) -> Self {
        Self(name.into())
    }
}

impl MultiStr for CssPropertyName<'_> {
    #[inline]
    fn find_map<A, F: FnMut(&str) -> Option<A>>(&self, mut f: F) -> Option<A> {
        let result = f(&self.0);
        if result.is_some() {
            return result;
        }
        VENDOR_PREFIXES
            .iter()
            .find_map(|prefix| {
                f(&[prefix, self.0.as_ref()].concat())
            })
    }
}
