use crate::*;
use once_cell::race::OnceBox;
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    convert::TryFrom,
    mem,
    sync::Arc,
};
use web_sys::{CssStyleDeclaration, CssStyleRule, CssStyleSheet, HtmlStyleElement};

pub mod named_color;

mod align;
pub use align::Align;

mod background;
pub use background::Background;

mod borders;
pub use borders::{Border, Borders};

mod clip;
pub use clip::Clip;

mod font;
pub use font::{Font, FontFamily, FontWeight, FontLine};

mod height;
pub use height::Height;

mod layer_index;
pub use layer_index::LayerIndex;

mod padding;
pub use padding::Padding;

mod rounded_corners;
pub use rounded_corners::RoundedCorners;

mod scrollbars;
pub use scrollbars::Scrollbars;

mod shadows;
pub use shadows::{Shadow, Shadows};

mod spacing;
pub use spacing::Spacing;

mod transform;
pub use transform::Transform;

mod width;
pub use width::Width;

// --

pub struct CssPropValue<'a> {
    value: Cow<'a, str>,
    important: bool,
}

// ------ StaticCSSProps ------

#[derive(Default)]
pub struct StaticCSSProps<'a>(BTreeMap<&'a str, CssPropValue<'a>>);

impl<'a> StaticCSSProps<'a> {
    pub fn insert(&mut self, name: &'a str, value: impl IntoCowStr<'a>) {
        self.0.insert(name, CssPropValue {
            value: value.into_cow_str(),
            important: false
        });
    }

    pub fn insert_important(&mut self, name: &'a str, value: impl IntoCowStr<'a>) {
        self.0.insert(name, CssPropValue {
            value: value.into_cow_str(),
            important: true
        });
    }
}

impl<'a> IntoIterator for StaticCSSProps<'a> {
    type Item = (&'a str, CssPropValue<'a>);
    type IntoIter = std::collections::btree_map::IntoIter<&'a str, CssPropValue<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> Extend<(&'a str, CssPropValue<'a>)> for StaticCSSProps<'a> {
    fn extend<T: IntoIterator<Item=(&'a str, CssPropValue<'a>)>>(&mut self, iter: T) {
        self.0.extend(iter);
    }
}

// ------ DynamicCSSProps ------

pub type DynamicCSSProps = BTreeMap<Cow<'static, str>, BoxedCssSignal>;

// ------ StaticCssClasses ------

pub type StaticCssClasses<'a> = BTreeSet<Cow<'a, str>>;

// ------ BoxedCssSignal ------

pub type BoxedCssSignal = Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>;

// @TODO replace with a new function? https://github.com/Pauan/rust-signals/blob/master/CHANGELOG.md#0322---2021-06-13
pub fn box_css_signal(
    signal: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
) -> BoxedCssSignal {
    Box::new(signal.map(|value| Box::new(value) as Box<dyn IntoOptionCowStr<'static>>))
}

// ------ units ------

pub fn px<'a>(px: impl IntoCowStr<'a>) -> Cow<'a, str> {
    [&px.into_cow_str(), "px"].concat().into()
}

pub fn ch<'a>(ch: impl IntoCowStr<'a>) -> Cow<'a, str> {
    [&ch.into_cow_str(), "ch"].concat().into()
}

// ------ Style ------

pub trait Style<'a>: Default {
    fn new() -> Self {
        Self::default()
    }

    fn apply_to_raw_el<E: RawEl>(
        self,
        raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>);
}

// ------ StyleGroup ------

pub struct StyleGroup<'a> {
    pub selector: Cow<'a, str>,
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> StyleGroup<'a> {
    pub fn new(selector: impl IntoCowStr<'a>) -> Self {
        Self {
            selector: selector.into_cow_str(),
            static_css_props: StaticCSSProps::default(),
            dynamic_css_props: DynamicCSSProps::default(),
        }
    }

    pub fn style(mut self, name: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        self.static_css_props.insert(name, value.into());
        self
    }

    pub fn style_important(mut self, name: &'a str, value: impl Into<Cow<'a, str>>) -> Self {
        self.static_css_props.insert_important(name, value.into());
        self
    }

    pub fn style_signal(
        mut self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'static> + 'static> + Unpin + 'static,
    ) -> Self {
        self.dynamic_css_props
            .insert(name.into_cow_str(), box_css_signal(value));
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
        let style_element: HtmlStyleElement = document()
            .create_element("style")
            .unwrap_throw()
            .unchecked_into();
        document()
            .head()
            .unwrap_throw()
            .append_child(&style_element)
            .unwrap_throw();
        let sheet = style_element.sheet().unwrap_throw().unchecked_into();
        Self {
            sheet: SendWrapper::new(sheet),
            rule_ids: MonotonicIds::default(),
        }
    }

    pub fn style_group(&self, group: StyleGroup) -> &Self {
        let (_, task_handles) = self.style_group_inner(group, false);
        mem::forget(task_handles);
        self
    }

    #[must_use]
    pub fn style_group_droppable(&self, group: StyleGroup) -> StyleGroupHandle {
        let (rule_id, _task_handles) = self.style_group_inner(group, true);
        StyleGroupHandle {
            rule_id,
            _task_handles,
        }
    }

    // --

    fn style_group_inner(&self, group: StyleGroup, droppable: bool) -> (u32, Vec<TaskHandle>) {
        let (rule_id_and_index, ids_lock) = self.rule_ids.add_new_id();
        let empty_rule = [&group.selector, "{}"].concat();

        self.sheet
            .insert_rule_with_index(&empty_rule, rule_id_and_index)
            .unwrap_or_else(|_| {
                panic!("invalid CSS selector: `{}`", &group.selector);
            });

        let declaration = self
            .sheet
            .css_rules()
            .unwrap_throw()
            .item(rule_id_and_index)
            .unwrap_throw()
            .unchecked_into::<CssStyleRule>()
            .style();

        drop(ids_lock);

        for (name, css_prop_value) in group.static_css_props {
            set_css_property(&declaration, name, &css_prop_value.value, css_prop_value.important);
        }

        let declaration = Arc::new(SendWrapper::new(declaration));
        let mut task_handles = Vec::new();
        for (name, value_signal) in group.dynamic_css_props {
            let declaration = Arc::clone(&declaration);
            let task = value_signal.for_each_sync(move |value| {
                if let Some(value) = value.into_option_cow_str() {
                    // @TODO allow to set `important ` also in dynamic styles
                    set_css_property(&declaration, &name, &value, false);
                } else {
                    declaration.remove_property(&name).unwrap_throw();
                }
            });
            if droppable {
                task_handles.push(Task::start_droppable(task));
            } else {
                Task::start(task);
            }
        }
        (rule_id_and_index, task_handles)
    }

    fn remove_rule(&self, id: u32) {
        let (rule_index, _ids_lock) = self.rule_ids.remove_id(id);
        self.sheet
            .delete_rule(u32::try_from(rule_index).unwrap_throw())
            .unwrap_throw();
    }
}

fn set_css_property(declaration: &CssStyleDeclaration, name: &str, value: &str, important: bool) {
    let priority = if important { "important" } else { "" };
    declaration.set_property_with_priority(name, value, priority).unwrap_throw();
    if not(declaration
        .get_property_value(name)
        .unwrap_throw()
        .is_empty())
    {
        return;
    }
    for prefix in VENDOR_PREFIXES {
        let prefixed_name = [prefix, name].concat();
        declaration
            .set_property_with_priority(&prefixed_name, value, priority)
            .unwrap_throw();
        if not(declaration
            .get_property_value(&prefixed_name)
            .unwrap_throw()
            .is_empty())
        {
            return;
        }
    }
    panic!("invalid CSS property: `{}: {};`", name, value);
}
