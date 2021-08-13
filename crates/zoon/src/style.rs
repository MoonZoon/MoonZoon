use crate::*;
use std::{
    borrow::Cow, 
    collections::BTreeMap, 
    sync::Arc,
    convert::TryFrom,
};
use web_sys::{CssStyleSheet, HtmlStyleElement, CssStyleRule, CssStyleDeclaration};
use once_cell::race::OnceBox;

mod align;
pub use align::Align;

mod background;
pub use background::Background;

mod borders;
pub use borders::{Borders, Border};

mod color;
pub use color::{Color, NamedColor, HSLuv, hsl, hsla};

mod clip;
pub use clip::Clip;

mod font;
pub use font::{Font, NamedWeight, FontWeight, FontFamily};

mod height;
pub use height::Height;

mod padding;
pub use padding::Padding;

mod rounded_corners;
pub use rounded_corners::RoundedCorners;

mod scrollbars;
pub use scrollbars::Scrollbars;

mod shadows;
pub use shadows::{Shadows, Shadow};

mod spacing;
pub use spacing::Spacing;

mod transform;
pub use transform::Transform;

mod width;
pub use width::Width;

// --

pub type StaticCSSProps<'a> = BTreeMap<&'a str, Cow<'a, str>>;
pub type DynamicCSSProps = BTreeMap<Cow<'static, str>, BoxedCssSignal>;

pub type BoxedCssSignal = Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>;

pub fn box_css_signal(
    signal: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
) -> BoxedCssSignal {
    Box::new(signal.map(|value| Box::new(value) as Box<dyn IntoOptionCowStr<'static>>))
}

pub fn px<'a>(px: impl IntoCowStr<'a>) -> Cow<'a, str> {
    [&px.into_cow_str(), "px"].concat().into()
}

// ------ Style ------

pub trait Style<'a>: Default {
    fn new() -> Self {
        Self::default()
    }

    fn into_css_props_container(self) -> CssPropsContainer<'a>;

    fn update_raw_el_styles<T: RawEl>(self, mut raw_el: T) -> T {
        let CssPropsContainer { 
            static_css_props,
            dynamic_css_props,
            task_handles,
         } = self.into_css_props_container();

        for (name, value) in static_css_props {
            raw_el = raw_el.style(name, &value);
        }
        for (name, value) in dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        if not(task_handles.is_empty()) {
            raw_el = raw_el.after_remove(move |_| drop(task_handles))
        }
        raw_el
    }
}

// ------ CssPropsContainer ------

#[derive(Default)]
pub struct CssPropsContainer<'a> {
    pub static_css_props: StaticCSSProps<'a>,
    pub dynamic_css_props: DynamicCSSProps,
    pub task_handles: Vec<TaskHandle>,
}

// ------ StyleGroup ------

pub struct StyleGroup<'a> {
    pub selector: Cow<'a, str>,
    pub css_props_container: CssPropsContainer<'a>,
}

impl<'a> StyleGroup<'a> {
    pub fn new(selector: impl IntoCowStr<'a>) -> Self {
        Self {
            selector: selector.into_cow_str(),
            css_props_container: CssPropsContainer::default()
        }
    }

    pub fn style(mut self, name: &'a str, value: &'a str) -> Self {
        self.css_props_container.static_css_props.insert(name, value.into());
        self
    }

    pub fn style_signal(
        mut self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        self.css_props_container.dynamic_css_props.insert(
            name.into_cow_str(), 
            box_css_signal(value),
        );
        self
    }
}

// ------ StyleGroupHandle ------

pub struct StyleGroupHandle {
    rule_id: u32,
    _task_handles: Vec<TaskHandle>,
}

impl Drop for StyleGroupHandle {
    fn drop(&mut self) {
        global_styles().remove_rule(self.rule_id);
    }
}

// ------ global_styles ------

pub fn global_styles() -> &'static GlobalStyles {
    static GLOBAL_STYLES: OnceBox<GlobalStyles> = OnceBox::new();
    GLOBAL_STYLES.get_or_init(|| Box::new(GlobalStyles::new()))
}

pub struct GlobalStyles {
    sheet: SendWrapper<CssStyleSheet>,
    rule_ids: MonotonicIds,
}

impl GlobalStyles {
    fn new() -> Self {
        let style_element: HtmlStyleElement = document().create_element("style").unwrap_throw().unchecked_into();
        document().head().unwrap_throw().append_child(&style_element).unwrap_throw();
        let sheet = style_element.sheet().unwrap_throw().unchecked_into();
        Self {
            sheet: SendWrapper::new(sheet),
            rule_ids: MonotonicIds::default(),
        }
    }

    pub fn push_style_group(&self, group: StyleGroup) {

    }

    #[must_use]
    pub fn push_style_group_droppable(&self, group: StyleGroup) -> StyleGroupHandle {
        let (rule_id_and_index, ids_lock) = self.rule_ids.add_new_id();

        let empty_rule = {
            let mut rule = group.selector.to_string();
            rule.push_str("{}");
            rule
        };
        self.sheet.insert_rule_with_index(&empty_rule, rule_id_and_index).unwrap_throw();
        let declaration = self
            .sheet
            .css_rules()
            .unwrap_throw()
            .item(rule_id_and_index)
            .unwrap_throw()
            .unchecked_into::<CssStyleRule>()
            .style();

        drop(ids_lock);

        for (name, value) in group.css_props_container.static_css_props {
            set_css_property(&declaration, name, &value);
        }

        let declaration = Arc::new(SendWrapper::new(declaration));
        let mut task_handles = group.css_props_container.task_handles;
        for (name, value_signal) in group.css_props_container.dynamic_css_props {
            let declaration = Arc::clone(&declaration);
            task_handles.push(Task::start_droppable(value_signal.for_each(move |value| {
                if let Some(value) = value.into_option_cow_str() {
                    set_css_property(&declaration, &name, &value);
                } else {
                    declaration.remove_property(&name).unwrap_throw();
                }
                async {}
            })));
        }

        StyleGroupHandle {
            rule_id: rule_id_and_index,
            _task_handles: task_handles,
        }
    }

    fn remove_rule(&self, id: u32) {
        let (rule_index, _ids_lock) = self.rule_ids.remove_id(id);
        self.sheet.delete_rule(u32::try_from(rule_index).unwrap_throw()).unwrap_throw();
    }
}

fn set_css_property(declaration: &CssStyleDeclaration, name: &str, value: &str) {
    declaration.set_property(name, &value).unwrap_throw();
    if not(declaration.get_property_value(name).unwrap_throw().is_empty()) {
        return;
    }
    for prefix in VENDOR_PREFIXES {
        let prefixed_name = &[prefix, name].concat();
        declaration.set_property(&prefixed_name, &value).unwrap_throw();
        if not(declaration.get_property_value(name).unwrap_throw().is_empty()) {
            return;
        }
    }
    panic!("invalid CSS property: `{}: {};`", name, value);
}
