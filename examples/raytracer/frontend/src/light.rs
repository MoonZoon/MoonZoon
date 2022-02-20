use crate::math::{f32x4, OrthonormalBasis, Vec3, Wec3};
use crate::spectrum::{Srgb, WSrgb};
use sdfu::mathtypes::Lerp;

pub trait Light: Send + Sync {
    /// returns (sampled point, output radiance toward ref, pdf of sample wrt solid angle wrt ref point)
    fn sample(&self, samples: &[f32x4; 2], point: Wec3) -> (Wec3, WSrgb, f32x4);

    /// returns (distance along ray of sampled point, pdf of sample)
    fn sample_volume_scattering(
        &self,
        sample: f32x4,
        ray_o: Wec3,
        ray_d: Wec3,
        max_distance: f32x4,
    ) -> (f32x4, f32x4);
}

#[derive(Clone, Copy)]
pub struct SphereLight {
    pos: Wec3,
    emission: WSrgb,
    rad: f32x4,
}

impl SphereLight {
    pub fn new(pos: Vec3, rad: f32, emission: Srgb) -> Self {
        Self {
            pos: Wec3::splat(pos),
            emission: WSrgb::splat(emission),
            rad: f32x4::from(rad),
        }
    }
}

impl Light for SphereLight {
    /// returns (sampled point, output radiance toward ref, pdf of sample wrt solid angle wrt ref point)
    fn sample(&self, samples: &[f32x4; 2], p: Wec3) -> (Wec3, WSrgb, f32x4) {
        let dir_to_light = self.pos - p;
        let dist_to_light_sq = dir_to_light.mag_sq();
        let dist_to_light = dist_to_light_sq.sqrt();
        let dir_to_light = dir_to_light / dist_to_light;
        let basis = (-dir_to_light).get_orthonormal_basis();

        let r2 = self.rad * self.rad;

        let sin_theta_max_2 = r2 / dist_to_light_sq;
        let cos_theta_max = f32x4::ZERO.max(f32x4::ONE - sin_theta_max_2).sqrt();
        let cos_theta = (f32x4::ONE - samples[0]) + samples[0] * cos_theta_max;
        let sin_theta = f32x4::ZERO.max(f32x4::ONE - cos_theta * cos_theta).sqrt();
        let phi = samples[1] * f32x4::TWO_PI;

        let ds = dist_to_light * cos_theta
            - f32x4::ZERO
                .max(r2 - dist_to_light_sq * sin_theta * sin_theta)
                .sqrt();
        let cos_alpha =
            (dist_to_light_sq + r2 - ds * ds) / (f32x4::from(2.0) * dist_to_light * self.rad);
        let sin_alpha = f32x4::ZERO.max(f32x4::ONE - cos_alpha * cos_alpha).sqrt();

        let (sin_phi, cos_phi) = phi.sin_cos();

        let offset = basis.cols[0] * sin_alpha * cos_phi
            + basis.cols[1] * sin_alpha * sin_phi
            + basis.cols[2] * cos_alpha;

        let point = self.pos + offset * self.rad;

        let pdf = uniform_cone_pdf(cos_theta_max);

        (point, self.emission, pdf)
    }

    /// returns (radiance toward sampled point, distance along ray of sampled point, pdf of sample)
    fn sample_volume_scattering(
        &self,
        sample: f32x4,
        ray_o: Wec3,
        ray_d: Wec3,
        max_distance: f32x4,
    ) -> (f32x4, f32x4) {
        // equi-angular sampling from
        // "Importance Sampling Techniques for Path Tracing in Participating Media" by Kulla and Fajardo.

        // get coord of closest point to light along (infinite) ray
        let delta = (self.pos - ray_o).dot(ray_d);

        // get distance this point is from light
        let closest_point = ray_o + delta * ray_d;
        let d = (closest_point - self.pos).mag();

        // get angle of endpoints
        let theta_a = (-delta).atan2(d);
        let theta_b = (max_distance - delta).atan2(d);

        // take sample
        let t = d * theta_a.lerp(theta_b, sample).tan();
        let sample_dist = delta + t;
        let pdf = d / ((theta_b - theta_a) * (d.mul_add(d, t * t)));

        (sample_dist, pdf)
    }
}

fn uniform_cone_pdf(cos_theta_max: f32x4) -> f32x4 {
    f32x4::ONE / (f32x4::TWO_PI * (f32x4::ONE - cos_theta_max))
}
