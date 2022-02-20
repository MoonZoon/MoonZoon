use crate::hitable::{Hitable, WHit, WShadingPoint};
use crate::material::MaterialHandle;
use crate::math::{f32x4, Wec3};
use crate::ray::WRay;
use crate::setup::SDF_DETAIL_SCALE;

use sdfu::*;

const MAX_MARCHES: u32 = 256;
const MAX_VIS_MARCHES: u32 = 100;

pub struct TracedSDF<S> {
    sdf: S,
    material: MaterialHandle,
}

impl<S> TracedSDF<S> {
    pub fn new(sdf: S, material: MaterialHandle) -> Self {
        TracedSDF { sdf, material }
    }
}

impl<S: SDF<f32x4, Wec3> + Send + Sync> Hitable for TracedSDF<S> {
    // return 1.0 for not occluded, 0.0 for occluded
    fn occluded(&self, start: Wec3, end: Wec3, _time: f32x4) -> f32x4 {
        let dir = end - start;
        let max_dist = dir.mag();
        let dir = dir / max_dist;

        let dist = self.sdf.dist(start).abs();

        let nan_mask = dist.cmp_nan(dist);
        let gt_mask = dist.cmp_gt(max_dist);
        let mut gt_nan_mask = gt_mask | nan_mask;

        let mut hit_mask = dist.cmp_lt(f32x4::from(0.0001));
        let mut t = dist;
        for _march in 0..MAX_VIS_MARCHES {
            let gt_mask = t.cmp_gt(max_dist);
            gt_nan_mask = gt_mask | nan_mask;
            if gt_nan_mask.move_mask() == 0b1111 {
                break;
            }

            let point = dir.mul_add(Wec3::broadcast(t), start);
            let dist = self.sdf.dist(point).abs();

            hit_mask = dist.cmp_lt(f32x4::from(0.0001 * SDF_DETAIL_SCALE).max(f32x4::from(0.00001 * SDF_DETAIL_SCALE) * t));

            let hit_gt_nan_mask = hit_mask | gt_nan_mask;
            if hit_gt_nan_mask.move_mask() == 0b1111 {
                break;
            }
            t = f32x4::merge(hit_gt_nan_mask, t, t + dist);
        }
        f32x4::merge(hit_mask & !gt_nan_mask, f32x4::ZERO, f32x4::ONE)
    }

    fn hit(&self, ray: &WRay, t_max: f32x4, hit_threshold_at: &dyn Fn(f32x4) -> f32x4) -> f32x4 {
        let dist = self.sdf.dist(ray.origin).abs();
        let mut t = dist;

        let nan_mask = t.cmp_nan(t);

        for _march in 0..MAX_MARCHES {
            let point = ray.point_at(t);
            let dist = self.sdf.dist(point).abs();

            let hit_mask = dist.cmp_lt(
                f32x4::from(0.00005 * SDF_DETAIL_SCALE)
                    .max(f32x4::from(0.05 * SDF_DETAIL_SCALE) * hit_threshold_at(t)));

            let gt_mask = t.cmp_gt(t_max);
            let hit_gt_nan_mask = hit_mask | nan_mask | gt_mask;

            t = f32x4::merge(hit_gt_nan_mask, t, t + dist);

            if hit_gt_nan_mask.move_mask() == 0b1111 {
                break;
            }
        }
        t
    }

    fn get_shading_info(
        &self,
        hit: WHit,
        half_pixel_size_at: &dyn Fn(f32x4) -> f32x4,
    ) -> (MaterialHandle, WShadingPoint) {
        let point = hit.point();

        let half_pixel_size = f32x4::from(0.0001).max(f32x4::from(SDF_DETAIL_SCALE) * half_pixel_size_at(hit.t));

        let normals = self.sdf.normals_fast(half_pixel_size);

        let normal = normals.normal_at(point);
        (
            self.material,
            WShadingPoint::new(hit, point, half_pixel_size, normal),
        )
    }
}

#[derive(Clone, Copy)]
pub struct MandelBox {
    iterations: usize,
    scale: f32x4,
    scale_vec: Wec3,
    box_fold: BoxFold,
    sphere_fold: SphereFold,
}

impl MandelBox {
    pub fn new(iterations: usize, box_fold: BoxFold, sphere_fold: SphereFold, scale: f32) -> Self {
        Self {
            iterations,
            box_fold,
            sphere_fold,
            scale: scale.into(),
            scale_vec: Wec3::broadcast(scale.into()),
        }
    }
}

impl SDF<f32x4, Wec3> for MandelBox {
    fn dist(&self, mut p: Wec3) -> f32x4 {
        let offset = p;
        let one = f32x4::ONE;
        let mut dr = one;
        for _ in 0..self.iterations {
            self.box_fold.box_fold(&mut p);
            self.sphere_fold.sphere_fold(&mut p, &mut dr);

            p = p.mul_add(self.scale_vec, offset);
            dr = (-dr).mul_add(self.scale, one);
        }

        let d = p.mag() / dr.abs();
        d
    }
}

#[derive(Clone, Copy)]
pub struct BoxFold {
    l: Wec3,
    neg_l: Wec3,
    two: Wec3,
}

impl BoxFold {
    pub fn new(side_length: f32) -> Self {
        let l = Wec3::broadcast(side_length.into());
        BoxFold {
            l,
            neg_l: -l,
            two: Wec3::broadcast(2.0.into()),
        }
    }

    pub fn box_fold(&self, point: &mut Wec3) {
        *point = point.clamped(self.neg_l, self.l).mul_add(self.two, -*point)
    }
}

#[derive(Clone, Copy)]
pub struct SphereFold {
    min_rad_sq: f32x4,
    fixed_rad_sq: f32x4,
}

impl SphereFold {
    pub fn new(min_radius: f32, fixed_radius: f32) -> Self {
        let min_rad_sq = (min_radius * min_radius).into();
        let fixed_rad_sq = (fixed_radius * fixed_radius).into();
        Self {
            min_rad_sq,
            fixed_rad_sq,
        }
    }

    pub fn sphere_fold(&self, point: &mut Wec3, dr: &mut f32x4) {
        let r2 = point.mag_sq();

        let mul = f32x4::ONE.max(self.fixed_rad_sq / self.min_rad_sq.max(r2));
        *point *= mul;
        *dr *= mul;
    }
}

#[derive(Clone, Copy)]
pub struct BrokenSphereFold {
    #[allow(dead_code)]
    rad_sq: f32x4,
}

impl BrokenSphereFold {
    #[allow(dead_code)]
    pub fn new(radius: f32) -> Self {
        Self {
            rad_sq: radius.into(),
        }
    }

    #[allow(dead_code)]
    pub fn sphere_fold(&self, point: &mut Wec3, dr: &mut f32x4) {
        let r2 = point.mag_sq();
        let mul = (self.rad_sq / r2)
            .max(self.rad_sq)
            .clamp(f32x4::ZERO, f32x4::ONE);
        *point *= mul;
        *dr *= mul;
    }
}
