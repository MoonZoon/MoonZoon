use sdfu::mathtypes::Lerp;

use crate::math::CDF;

use std::f32::consts::PI;

pub trait Filter: Copy + Clone + Send {
    fn radius(&self) -> f32;
    fn evaluate(&self, p: f32) -> f32;
}

#[derive(Clone, Copy)]
pub struct BlackmanHarrisFilter {
    pub radius: f32,
}

impl BlackmanHarrisFilter {
    pub fn new(radius: f32) -> Self {
        BlackmanHarrisFilter { radius }
    }
}

impl Default for BlackmanHarrisFilter {
    fn default() -> Self {
        BlackmanHarrisFilter::new(1.5)
    }
}

const A0: f32 = 0.35875;
const A1: f32 = 0.48829;
const A2: f32 = 0.14128;
const A3: f32 = 0.01168;
const TWOPI: f32 = PI * 2.0;
const FOURPI: f32 = PI * 4.0;
const SIXPI: f32 = PI * 6.0;

impl Filter for BlackmanHarrisFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: f32) -> f32 {
        if p.abs() > self.radius {
            return 0.0;
        }
        let x = (p / self.radius).abs() * 0.5 + 0.5;
        A0 - A1 * (TWOPI * x).cos() + A2 * (FOURPI * x).cos() + A3 * (SIXPI * x).cos()
    }
}

#[derive(Clone, Copy)]
pub struct MitchellNetravaliFilter {
    pub radius: f32,
    pub b: f32,
    pub c: f32,
    pub samples: [f32; 16],
}

impl MitchellNetravaliFilter {
    pub fn new(radius: f32, b: f32, c: f32) -> Self {
        let mut samples = [0.0; 16];
        for (n, sample) in samples.iter_mut().enumerate() {
            *sample =
                MitchellNetravaliFilter::evaluate(radius, (n as f32 / 15.0) * 0.5 + 0.5, b, c);
        }
        MitchellNetravaliFilter {
            radius,
            b,
            c,
            samples,
        }
    }

    fn evaluate(radius: f32, p: f32, b: f32, c: f32) -> f32 {
        let x = (2.0 * p / radius).abs();
        if x >= 2.0 {
            return 0.0;
        }
        if x > 1.0 {
            ((-b - 6.0 * c) * x * x * x
                + (6.0 * b + 30.0 * c) * x * x
                + (-12.0 * b - 48.0 * c) * x
                + (8.0 * b + 24.0 * c))
                * (1.0 / 6.0)
        } else {
            ((12.0 - 9.0 * b - 6.0 * c) * x * x * x
                + (-18.0 + 12.0 * b + 6.0 * c) * x * x
                + (6.0 - 2.0 * b))
                * (1.0 / 6.0)
        }
    }
}

impl Default for MitchellNetravaliFilter {
    fn default() -> Self {
        MitchellNetravaliFilter::new(2.0, 1.0 / 3.0, 1.0 / 3.0)
    }
}

impl Filter for MitchellNetravaliFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: f32) -> f32 {
        MitchellNetravaliFilter::evaluate(self.radius, p, self.b, self.c)
    }
}

#[derive(Clone, Copy)]
pub struct BoxFilter {
    pub radius: f32,
}

impl BoxFilter {
    pub fn new(radius: f32) -> Self {
        BoxFilter { radius }
    }
}

impl Default for BoxFilter {
    fn default() -> Self {
        BoxFilter::new(0.5)
    }
}

impl Filter for BoxFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: f32) -> f32 {
        let x = p.abs();
        if x > self.radius {
            0.0
        } else {
            1.0
        }
    }
}

#[derive(Clone, Copy)]
pub struct LanczosSincFilter {
    pub radius: f32,
    pub tau: f32,
}

impl Default for LanczosSincFilter {
    fn default() -> Self {
        LanczosSincFilter::new(3.0, 3.0)
    }
}

impl LanczosSincFilter {
    pub fn new(radius: f32, tau: f32) -> Self {
        LanczosSincFilter { radius, tau }
    }

    fn sinc(x: f32) -> f32 {
        let x = x.abs();
        if x <= 0.00001 {
            1.0
        } else {
            let pix = std::f32::consts::PI * x;
            let sin = pix.sin();
            sin / pix
        }
    }
}

impl Filter for LanczosSincFilter {
    fn radius(&self) -> f32 {
        self.radius
    }

    fn evaluate(&self, p: f32) -> f32 {
        let x = p.abs();
        if x > self.radius {
            0.0
        } else {
            let lanczos = LanczosSincFilter::sinc(x / self.tau);
            LanczosSincFilter::sinc(x) * lanczos
        }
    }
}

const FILTER_TABLE_SIZE: usize = 512;

pub struct FilterImportanceSampler {
    inverse_cdf: [f32; FILTER_TABLE_SIZE],
}

impl FilterImportanceSampler {
    /// Create a new filter importance sampler. The filter must not have any negative
    /// lobe.
    pub fn new<F: Filter>(filter: &F) -> Self {
        let f_rad = filter.radius();

        let mut cdf = CDF::new();

        for n in 0..FILTER_TABLE_SIZE {
            let t = n as f32 / (FILTER_TABLE_SIZE - 1) as f32;
            let d = 0.0.lerp(f_rad, t);
            cdf.insert(d, filter.evaluate(d));
        }

        cdf.prepare();

        let mut inverse_cdf = [0.0; FILTER_TABLE_SIZE];

        for (n, inv_cdf_item) in inverse_cdf.iter_mut().enumerate() {
            let u = n as f32 / (FILTER_TABLE_SIZE - 1) as f32;
            let cdf_u = cdf.sample(u).unwrap();
            *inv_cdf_item = cdf_u.0;
        }

        FilterImportanceSampler { inverse_cdf }
    }

    /// Takes uniform sample on (0, 1) and importance samples it based on the
    /// filter's PDF, giving an output in the range (-filter.radius, filter.radius)
    pub fn sample(&self, u: f32) -> f32 {
        // rescale to (-1, 1)
        let u = 2.0 * (u - 0.5);

        let mult = if u < 0.0 { -1.0 } else { 1.0 };

        let u = u.abs().max(0.0).min(0.99999);

        let idx_full = u * (FILTER_TABLE_SIZE - 1) as f32;
        let idx = idx_full.floor() as usize;
        let t = idx_full.fract();

        mult * self.inverse_cdf[idx].lerp(self.inverse_cdf[idx + 1], t)
    }
}
