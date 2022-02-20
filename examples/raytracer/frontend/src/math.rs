use crate::spectrum::WSrgb;

use std::f32::consts::PI;
pub use ultraviolet::f32x4;
pub use ultraviolet::Lerp;

pub type Vec3 = ultraviolet::Vec3;
pub type Wec3 = ultraviolet::Wec3;
pub type Vec2 = ultraviolet::Vec2;
pub type Wec2 = ultraviolet::Wec2;
pub type Vec2u = ultraviolet::int::Vec2u;

pub type Wat3 = ultraviolet::Wat3;

#[derive(Clone, Copy, Debug)]
pub struct Extent2u {
    pub w: u32,
    pub h: u32,
}

impl Extent2u {
    pub const fn new(w: u32, h: u32) -> Self {
        Self { w, h }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bounds2u {
    pub min: Vec2u,
    pub max: Vec2u,
}

impl Bounds2u {
    pub fn size(&self) -> Extent2u {
        Extent2u::new(self.max.x - self.min.x, self.max.y - self.min.y)
    }
}

#[derive(Clone, Copy)]
pub struct Transform {
    pub position: Wec3,
    // pub orientation: Quat,
}

pub trait OrthonormalBasis<M>: Sized {
    fn get_orthonormal_basis(&self) -> M;
}

impl OrthonormalBasis<Wat3> for Wec3 {
    fn get_orthonormal_basis(&self) -> Wat3 {
        let nor = *self;
        let ks = nor.z.signum();
        let ka = f32x4::ONE / (f32x4::ONE + nor.z.abs());
        let kb = -ks * nor.x * nor.y * ka;
        let uu = Wec3::new(f32x4::ONE - nor.x * nor.x * ka, ks * kb, -ks * nor.x);
        let vv = Wec3::new(kb, ks - nor.y * nor.y * ka * ks, -nor.y);
        Wat3::new(uu, vv, nor)
    }
}

pub trait RandomSample2d {
    type Sample;
    fn rand_in_unit_disk(samples: &Self::Sample) -> Self;
}

impl RandomSample2d for Wec2 {
    type Sample = [f32x4; 2];
    fn rand_in_unit_disk(samples: &Self::Sample) -> Self {
        // let r = samples[0].sqrt();
        // let theta = samples[1] * f32x4::TWO_PI;
        // let (s, c) = theta.sin_cos();
        // Wec2::new(r * c, r * s)
        concentric_circle_map(samples)
    }
}

pub trait RandomSample3d<T> {
    fn rand_in_unit_sphere(samples: &[T; 2]) -> Self;
    fn rand_on_unit_sphere(samples: &[T; 2]) -> Self;
    fn cosine_weighted_in_hemisphere(samples: &[T; 2]) -> Self;
    fn cosine_power_weighted(samples: &[T; 2], power: T) -> Self;
}

impl RandomSample3d<f32x4> for Wec3 {
    fn rand_in_unit_sphere(samples: &[f32x4; 2]) -> Self {
        let theta = samples[0] * f32x4::from(2f32 * PI);
        let phi = samples[1] * f32x4::from(2.0) - f32x4::ONE;
        let ophisq = (f32x4::ONE - phi * phi).sqrt();
        let (sin, cos) = theta.sin_cos();
        Wec3::new(ophisq * cos, ophisq * sin, phi)
    }

    fn rand_on_unit_sphere(samples: &[f32x4; 2]) -> Self {
        Self::rand_in_unit_sphere(samples).normalized()
    }

    // pdf: cos(theta) / pi
    // in return coord space: ret.z / pi
    fn cosine_weighted_in_hemisphere(samples: &[f32x4; 2]) -> Self {
        let xy = Wec2::rand_in_unit_disk(samples);
        let z = (f32x4::ONE - xy.mag_sq().min(f32x4::ONE)).sqrt();
        Wec3::new(xy.x, xy.y, z)
    }

    // pdf: (power+1)/ 2pi * cos^power(alpha)
    fn cosine_power_weighted(samples: &[f32x4; 2], power: f32x4) -> Self {
        let two = f32x4::from(2.0);
        let a = samples[0].powf(f32x4::ONE / (power + f32x4::ONE));
        let a2 = a * a;
        let b = (f32x4::ONE - a2).sqrt();
        let (s, c) = (two * samples[1]).sin_cos();
        Wec3::new(b * c, b * s, a)
    }
}

#[allow(dead_code)]
pub fn f0_from_ior(ior: f32x4) -> f32x4 {
    let f0 = (f32x4::ONE - ior) / (f32x4::ONE + ior);
    f0 * f0
}

pub fn f_schlick(cos: f32x4, f0: f32x4) -> f32x4 {
    f0 + (f32x4::ONE - f0) * (f32x4::ONE - cos).powi([5, 5, 5, 5])
}

#[allow(dead_code)]
pub fn f_schlick_c(cos: f32x4, f0: WSrgb) -> WSrgb {
    f0 + (WSrgb::one() - f0) * (f32x4::ONE - cos).powi([5, 5, 5, 5])
}

#[allow(dead_code)]
pub fn saturate(v: f32x4) -> f32x4 {
    v.min(f32x4::ONE).max(f32x4::ZERO)
}

pub struct CDF {
    items: Vec<(f32, f32)>,
    densities: Vec<f32>,
    weight_sum: f32,
    prepared: bool,
}

impl CDF {
    pub fn new() -> Self {
        CDF {
            items: Vec::new(),
            densities: Vec::new(),
            weight_sum: 0.0,
            prepared: false,
        }
    }

    pub fn insert(&mut self, item: f32, weight: f32) {
        self.items.push((item, weight));
        self.weight_sum += weight;
    }

    pub fn prepare(&mut self) {
        if self.prepared {
            return;
        }

        for (_, weight) in self.items.iter_mut() {
            *weight /= self.weight_sum;
        }

        let mut cum = 0.0;
        for (_, weight) in self.items.iter() {
            cum += *weight;
            self.densities.push(cum);
        }

        for (&(_, weight), density) in self.items.iter().zip(self.densities.iter_mut()).rev() {
            *density = 1.0;
            if weight > 0.0 {
                break;
            }
        }

        self.prepared = true;
    }

    pub fn sample(&self, x: f32) -> Option<(f32, f32)> {
        for (ret, density) in self.items.iter().zip(self.densities.iter()) {
            if *density >= x {
                return Some(*ret);
            }
        }
        None
    }
}

#[inline]
#[allow(dead_code)]
pub fn power_heuristic(n_samples_f: usize, f_pdf: f32, n_samples_g: usize, g_pdf: f32) -> f32 {
    let f = n_samples_f as f32 * f_pdf;
    let g = n_samples_g as f32 * g_pdf;
    f * f / (f * f + g * g)
}

pub fn concentric_circle_map(uv: &[f32x4; 2]) -> Wec2 {
    let two = f32x4::from(2.0);
    let a = uv[0].mul_add(two, -f32x4::ONE);
    let b = uv[1].mul_add(two, -f32x4::ONE);

    let zero_mask = a.cmp_eq(f32x4::ZERO) & b.cmp_eq(f32x4::ZERO);
    let b = f32x4::merge(zero_mask, f32x4::from(0.0001), b);

    let phi1 = f32x4::FRAC_PI_4 * b / a;
    let phi2 = (-f32x4::FRAC_PI_4 / b).mul_add(a, f32x4::FRAC_PI_2);

    let mask = (a * a).cmp_gt(b * b);

    let r = f32x4::merge(mask, a, b);
    let phi = f32x4::merge(mask, phi1, phi2);

    let (s, c) = phi.sin_cos();
    Wec2::new(r * c, r * s)
}
