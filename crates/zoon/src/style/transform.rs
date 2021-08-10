use crate::*;
use euclid::{default::Transform3D, Angle, Vector3D};

#[derive(Default)]
pub struct Transform<'a> {
    matrix: Transform3D<f64>,
    static_css_props: StaticCSSProps<'a>,
    dynamic_css_props: DynamicCSSProps,
}

impl<'a> Transform<'a> {
    pub fn move_up(mut self, distance: impl Into<f64>) -> Self {
        self.matrix = self.matrix.then_translate(Vector3D::new(0., -distance.into(), 0.));
        self
    }

    pub fn move_down(mut self, distance: impl Into<f64>) -> Self {
        self.matrix = self.matrix.then_translate(Vector3D::new(0., distance.into(), 0.));
        self
    }

    pub fn move_left(mut self, distance: impl Into<f64>) -> Self {
        self.matrix = self.matrix.then_translate(Vector3D::new(-distance.into(), 0., 0.));
        self
    }

    pub fn move_right(mut self, distance: impl Into<f64>) -> Self {
        self.matrix = self.matrix.then_translate(Vector3D::new(distance.into(), 0., 0.));
        self
    }

    pub fn rotate(mut self, degrees: impl Into<f64>) -> Self {
        self.matrix = self.matrix.then_rotate(0., 0., 1., Angle::degrees(degrees.into()));
        self
    }

    pub fn scale(mut self, factor: impl Into<f64>) -> Self {
        let factor = factor.into();
        self.matrix = self.matrix.then_scale(factor, factor, factor);
        self
    }
}

impl<'a> Style<'a> for Transform<'a> {
    fn into_css_props(mut self) -> (StaticCSSProps<'a>, DynamicCSSProps) {
        let matrix_values = self
            .matrix
            .to_array()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        let css_matrix = ["matrix3d(", &matrix_values, ")"].concat();
        self.static_css_props.insert("transform", css_matrix.into());
        (self.static_css_props, self.dynamic_css_props)
    }
}
