use crate::*;

#[derive(Default)]
pub struct Transform {
    transformations: Vec<String>,
}

impl Transform {
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
            .push(crate::format!("scale({}%)", percent.into()));
        self
    }
}

impl<'a> Style<'a> for Transform {
    fn into_css_props_container(self) -> CssPropsContainer<'a> {
        let mut static_css_props = StaticCSSProps::default();
        if not(self.transformations.is_empty()) {
            let transform_value = self
                .transformations
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join(" ");
            static_css_props.insert("transform", transform_value.into());
        }
        CssPropsContainer::default().static_css_props(static_css_props)
    }
}
