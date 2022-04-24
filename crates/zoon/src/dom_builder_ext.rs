use crate::*;
use dominator::traits::{MultiStr, OptionStr};
use std::mem::ManuallyDrop;
use wasm_bindgen::intern;
use web_sys::CssStyleDeclaration;

// @TODO Remove the file together with Dominator.

pub trait DomBuilderExt {
    fn style_signal<B, C, D, E>(self, name: B, value: E) -> Self
    where
        B: MultiStr + 'static,
        C: MultiStr,
        D: OptionStr<Output = C>,
        E: Signal<Item = D> + 'static;

    fn style<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr;

    fn style_important<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr;
}

impl<A> DomBuilderExt for DomBuilder<A>
where
    A: Into<web_sys::Element> + Clone + 'static,
{
    #[inline]
    fn style_signal<B, C, D, E>(self, name: B, value: E) -> Self
    where
        B: MultiStr + 'static,
        C: MultiStr,
        D: OptionStr<Output = C>,
        E: Signal<Item = D> + 'static,
    {
        let element = self.__internal_element().into();
        if element.has_type::<web_sys::HtmlElement>() {
            let builder = DomBuilder::new(element.unchecked_into::<web_sys::HtmlElement>());
            return self.__internal_transfer_callbacks(builder.style_signal(name, value));
        }
        if element.has_type::<web_sys::SvgElement>() {
            let builder = DomBuilder::new(element.unchecked_into::<web_sys::SvgElement>());
            return self.__internal_transfer_callbacks(DomBuilderExtSvg::style_signal(
                builder, name, value,
            ));
        }
        unimplemented!("only `HtmlElement` and `SvgElement` support styling");
    }

    #[inline]
    fn style<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr,
    {
        let element = self.__internal_element().into();
        if element.has_type::<web_sys::HtmlElement>() {
            let builder = DomBuilder::new(element.unchecked_into::<web_sys::HtmlElement>());
            return self.__internal_transfer_callbacks(builder.style(name, value));
        }
        if element.has_type::<web_sys::SvgElement>() {
            let builder = DomBuilder::new(element.unchecked_into::<web_sys::SvgElement>());
            return self
                .__internal_transfer_callbacks(DomBuilderExtSvg::style(builder, name, value));
        }
        unimplemented!("only `HtmlElement` and `SvgElement` support styling");
    }

    #[inline]
    fn style_important<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr,
    {
        let element = self.__internal_element().into();
        if element.has_type::<web_sys::HtmlElement>() {
            let builder = DomBuilder::new(element.unchecked_into::<web_sys::HtmlElement>());
            return self.__internal_transfer_callbacks(builder.style_important(name, value));
        }
        if element.has_type::<web_sys::SvgElement>() {
            let builder = DomBuilder::new(element.unchecked_into::<web_sys::SvgElement>());
            return self.__internal_transfer_callbacks(DomBuilderExtSvg::style_important(
                builder, name, value,
            ));
        }
        unimplemented!("only `HtmlElement` and `SvgElement` support styling");
    }
}

// @TODO remove once .style is callable on SvgElement in Dominator
// https://github.com/Pauan/rust-dominator/issues/47

trait DomBuilderExtSvg {
    fn style_signal<B, C, D, E>(self, name: B, value: E) -> Self
    where
        B: MultiStr + 'static,
        C: MultiStr,
        D: OptionStr<Output = C>,
        E: Signal<Item = D> + 'static;

    fn style<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr;

    fn style_important<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr;
}

// https://github.com/Pauan/rust-dominator/blob/e9e9e61ed8bff32d2a3aed3c85e27d9256b76bf5/src/dom.rs
impl<A> DomBuilderExtSvg for DomBuilder<A>
where
    A: AsRef<web_sys::SvgElement> + Clone + 'static,
{
    #[inline]
    fn style_signal<B, C, D, E>(self, name: B, value: E) -> Self
    where
        B: MultiStr + 'static,
        C: MultiStr,
        D: OptionStr<Output = C>,
        E: Signal<Item = D> + 'static,
    {
        let style = self.__internal_element().as_ref().style();
        let mut is_set = false;

        let set_style_task = Task::start_droppable(value.for_each_sync(move |value| {
            let value = value.into_option();

            if value.is_some() {
                is_set = true;
            } else if is_set {
                is_set = false;
            } else {
                return;
            }

            match value {
                Some(value) => {
                    // TODO should this intern or not ?
                    set_style(&style, &name, value, false);
                }
                None => {
                    name.each(|name| {
                        // TODO handle browser prefixes ?
                        bindings::remove_style(&style, intern(name));
                    });
                }
            }
        }));

        let set_style_task = ManuallyDrop::new(set_style_task);
        self.after_removed(move |_| drop(ManuallyDrop::into_inner(set_style_task)))
    }

    #[inline]
    fn style<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr,
    {
        set_style(
            &self.__internal_element().as_ref().style(),
            &name,
            value,
            false,
        );
        self
    }

    #[inline]
    fn style_important<B, C>(self, name: B, value: C) -> Self
    where
        B: MultiStr,
        C: MultiStr,
    {
        set_style(
            &self.__internal_element().as_ref().style(),
            &name,
            value,
            true,
        );
        self
    }
}

fn set_style<A, B>(style: &CssStyleDeclaration, name: &A, value: B, important: bool)
where
    A: MultiStr,
    B: MultiStr,
{
    let mut names = vec![];
    let mut values = vec![];

    fn try_set_style(
        style: &CssStyleDeclaration,
        names: &mut Vec<String>,
        values: &mut Vec<String>,
        name: &str,
        value: &str,
        important: bool,
    ) -> Option<()> {
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
            // @TODO nicer error
            panic!(
                "style is incorrect:\n  names: {}\n  values: {}",
                names.join(", "),
                values.join(", ")
            );
        }
    }
}

// https://github.com/Pauan/rust-dominator/blob/e9e9e61ed8bff32d2a3aed3c85e27d9256b76bf5/src/bindings.rs
mod bindings {
    use wasm_bindgen::{intern, prelude::*};
    use web_sys::CssStyleDeclaration;

    pub(crate) fn get_style(style: &CssStyleDeclaration, name: &str) -> String {
        style.get_property_value(name).unwrap_throw()
    }

    pub(crate) fn remove_style(style: &CssStyleDeclaration, name: &str) {
        // TODO don't return String ?
        style.remove_property(name).unwrap_throw();
    }

    pub(crate) fn set_style(style: &CssStyleDeclaration, name: &str, value: &str, important: bool) {
        let priority = if important {
            intern("important")
        } else {
            intern("")
        };
        style
            .set_property_with_priority(name, value, priority)
            .unwrap_throw();
    }
}
