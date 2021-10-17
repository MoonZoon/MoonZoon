use crate::*;
use dominator::traits::{MultiStr, OptionStr, AsStr};
use wasm_bindgen::intern;
use web_sys::CssStyleDeclaration;

// @TODO remove once .style is callable on SvgElement in Dominator

pub trait DomBuilderExt {
    fn style_signal<B, C, D, E>(self, name: B, value: E) -> Self
        where B: MultiStr + 'static,
              C: MultiStr,
              D: OptionStr<Output = C>,
              E: Signal<Item = D> + 'static;

    fn style<B, C>(self, name: B, value: C) -> Self
        where B: MultiStr,
              C: MultiStr;

    fn style_important<B, C>(self, name: B, value: C) -> Self
        where B: MultiStr,
              C: MultiStr;

    fn style_unchecked<B, C>(self, name: B, value: C) -> Self
        where B: AsStr,
              C: AsStr;
}

// https://github.com/Pauan/rust-dominator/blob/e9e9e61ed8bff32d2a3aed3c85e27d9256b76bf5/src/dom.rs
impl<A> DomBuilderExt for DomBuilder<A> where A: AsRef<web_sys::SvgElement> + Clone {
    #[inline]
    fn style_signal<B, C, D, E>(mut self, name: B, value: E) -> Self
        where B: MultiStr + 'static,
              C: MultiStr,
              D: OptionStr<Output = C>,
              E: Signal<Item = D> + 'static {

        set_style_signal(self.__internal_element().as_ref().style(), &mut self.callbacks, name, value, false);
        self
    }

    #[inline]
    fn style<B, C>(self, name: B, value: C) -> Self
        where B: MultiStr,
              C: MultiStr {
        set_style(&self.__internal_element().as_ref().style(), &name, value, false);
        self
    }

    #[inline]
    fn style_important<B, C>(self, name: B, value: C) -> Self
        where B: MultiStr,
              C: MultiStr {
        set_style(&self.__internal_element().as_ref().style(), &name, value, true);
        self
    }

    #[inline]
    fn style_unchecked<B, C>(self, name: B, value: C) -> Self
        where B: AsStr,
              C: AsStr {
        name.with_str(|name| {
            value.with_str(|value| {
                bindings::set_style(&self.__internal_element().as_ref().style(), intern(name), value, false);
            });
        });
        self
    }
}

fn set_style<A, B>(style: &CssStyleDeclaration, name: &A, value: B, important: bool)
    where A: MultiStr,
          B: MultiStr {

    let mut names = vec![];
    let mut values = vec![];

    fn try_set_style(style: &CssStyleDeclaration, names: &mut Vec<String>, values: &mut Vec<String>, name: &str, value: &str, important: bool) -> Option<()> {
        assert!(value != "");

        // TODO handle browser prefixes ?
        bindings::remove_style(style, name);

        bindings::set_style(style, name, value, important);

        let is_changed = bindings::get_style(style, name) != "";

        if is_changed {
            Some(())

        } else {
            names.push(String::from(name));
            values.push(String::from(value));
            None
        }
    }

    let okay = name.find_map(|name| {
        let name: &str = intern(name);

        value.find_map(|value| {
            // TODO should this intern ?
            try_set_style(style, &mut names, &mut values, &name, &value, important)
        })
    });

    if let None = okay {
        if cfg!(debug_assertions) {
            // TODO maybe make this configurable
            panic!("style is incorrect:\n  names: {}\n  values: {}", names.join(", "), values.join(", "));
        }
    }
}

// https://github.com/Pauan/rust-dominator/blob/e9e9e61ed8bff32d2a3aed3c85e27d9256b76bf5/src/bindings.rs
mod bindings {
    use web_sys::CssStyleDeclaration;
    use wasm_bindgen::{prelude::*, intern};

    pub(crate) fn get_style(style: &CssStyleDeclaration, name: &str) -> String {
        style.get_property_value(name).unwrap_throw()
    }
    
    pub(crate) fn remove_style(style: &CssStyleDeclaration, name: &str) {
        // TODO don't return String ?
        style.remove_property(name).unwrap_throw();
    }

    pub(crate) fn set_style(style: &CssStyleDeclaration, name: &str, value: &str, important: bool) {
        let priority = if important { intern("important") } else { intern("") };
        style.set_property_with_priority(name, value, priority).unwrap_throw();
    }
}
