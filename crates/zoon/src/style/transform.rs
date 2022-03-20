use crate::*;
use std::{borrow::Cow, mem};

#[derive(Default)]
pub struct Transform {
    transformations: Vec<String>,
    dynamic_css_props: DynamicCSSProps,
}

impl Transform {
    pub fn with_signal(
        transform: impl Signal<Item = impl Into<Option<Self>>> + Unpin + 'static,
    ) -> Self {
        let mut this = Self::default();
        let transform = transform.map(|transform| {
            transform
                .into()
                .map(|mut transform| transform.transformations_into_value())
        });
        this.dynamic_css_props
            .insert("transform".into(), box_css_signal(transform));
        this
    }

    pub fn move_up(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateY(-{}px)", distance.into()));
        self
    }

    pub fn move_down(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateY({}px)", distance.into()));
        self
    }

    pub fn move_left(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateX(-{}px)", distance.into()));
        self
    }

    pub fn move_right(mut self, distance: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("translateX({}px)", distance.into()));
        self
    }

    pub fn rotate(mut self, degrees: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("rotateZ({}deg)", degrees.into()));
        self
    }

    pub fn scale(mut self, percent: impl Into<f64>) -> Self {
        self.transformations
            .push(crate::format!("scale({})", percent.into() / 100.));
        self
    }

    fn transformations_into_value(&mut self) -> Cow<'static, str> {
        let transformations = mem::take(&mut self.transformations);
        if transformations.is_empty() {
            return "none".into();
        }
        transformations
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join(" ")
            .into()
    }
}

impl<'a> Style<'a> for Transform {
    fn apply_to_raw_el<E: RawEl>(
        mut self,
        mut raw_el: E,
        style_group: Option<StyleGroup<'a>>,
    ) -> (E, Option<StyleGroup<'a>>) {
        let mut static_css_props = StaticCSSProps::default();

        if self.dynamic_css_props.is_empty() {
            static_css_props.insert("transform", self.transformations_into_value());
        }

        if let Some(mut style_group) = style_group {
            for (name, css_prop_value) in static_css_props {
                style_group = if css_prop_value.important {
                    style_group.style(name, css_prop_value.value)
                } else {
                    style_group.style_important(name, css_prop_value.value)
                };
            }
            for (name, value) in self.dynamic_css_props {
                style_group = style_group.style_signal(name, value);
            }
            return (raw_el, Some(style_group));
        }
        for (name, css_prop_value) in static_css_props {
            raw_el = if css_prop_value.important {
                raw_el.style_important(name, &css_prop_value.value)
            } else {
                raw_el.style(name, &css_prop_value.value)
            };
        }
        for (name, value) in self.dynamic_css_props {
            raw_el = raw_el.style_signal(name, value);
        }
        (raw_el, None)
    }
}
