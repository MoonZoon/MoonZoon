use crate::*;

pub trait OklchExt {
    fn new(l: impl Into<f64>, c: impl Into<f64>, h: impl Into<f64>, a: impl Into<f64>) -> OKLCH {
        OKLCH {
            l: l.into() as f32,
            c: c.into() as f32,
            h: h.into() as f32,
            alpha: a.into() as f32,
        }
    }
}

impl OklchExt for OKLCH {}
