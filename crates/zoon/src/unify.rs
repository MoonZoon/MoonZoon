use crate::*;

pub trait Unify {
    type Unified;
    type UnifiedOption;

    fn unify(self) -> Self::Unified;
    fn unify_option(self) -> Self::UnifiedOption;
}

impl<T: Element> Unify for Option<T> {
    type Unified = Option<RawElOrText>;
    type UnifiedOption = Self::Unified;

    fn unify(self) -> Self::Unified {
        self.map(Element::into_raw)
    }
    fn unify_option(self) -> Self::UnifiedOption {
        self.unify()
    }
}

impl<T: Element> Unify for T {
    type Unified = RawElOrText;
    type UnifiedOption = Option<RawElOrText>;

    fn unify(self) -> Self::Unified {
        self.into_raw()
    }
    fn unify_option(self) -> Self::UnifiedOption {
        Some(self.into_raw())
    }
}
