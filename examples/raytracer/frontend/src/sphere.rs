use crate::animation::WSequenced;
use crate::hitable::{Hitable, WHit, WShadingPoint};
use crate::material::MaterialHandle;
use crate::math::{f32x4, Wec3};
use crate::ray::WRay;

pub struct Sphere<TR> {
    transform_seq: TR,
    radius: f32,
    material: MaterialHandle,
}

impl<TR> Sphere<TR> {
    pub fn new(transform_seq: TR, radius: f32, material: MaterialHandle) -> Self {
        Sphere {
            transform_seq,
            radius,
            material,
        }
    }
}

impl<TR: WSequenced<Wec3>> Hitable for Sphere<TR> {
    fn occluded(&self, start: Wec3, end: Wec3, time: f32x4) -> f32x4 {
        let dir = end - start;
        let dist = dir.mag();
        let dir = dir / dist;

        let origin = WSequenced::sample_at(&self.transform_seq, time);
        let oc = start - origin;
        let b = oc.dot(dir);
        let c = oc.mag_sq() - f32x4::from(self.radius * self.radius);
        let descrim = b * b - c;

        let desc_pos = descrim.cmp_gt(f32x4::ZERO);

        let desc_sqrt = descrim.sqrt();

        let t1 = -b - desc_sqrt;
        let t2 = -b + desc_sqrt;

        let min = t1.min(t2);
        let valid = min.cmp_gt(f32x4::from(0.001)) & t1.cmp_le(dist) & desc_pos;

        f32x4::merge(valid, f32x4::ZERO, f32x4::ONE)
    }

    fn hit(&self, ray: &WRay, t_max: f32x4, _hit_threshold_at: &dyn Fn(f32x4) -> f32x4) -> f32x4 {
        let origin = WSequenced::sample_at(&self.transform_seq, ray.time);
        let oc = ray.origin - origin;
        let b = oc.dot(ray.dir);
        let c = oc.mag_sq() - f32x4::from(self.radius * self.radius);
        let descrim = b * b - c;

        let desc_pos = descrim.cmp_gt(f32x4::ZERO);

        let miss = f32x4::from(std::f32::MAX);

        let desc_sqrt = descrim.sqrt();

        let t1 = -b - desc_sqrt;
        let t1_valid = t1.cmp_gt(f32x4::from(0.0001)) & t1.cmp_le(t_max) & desc_pos;

        let t2 = -b + desc_sqrt;
        let t2_valid = t2.cmp_gt(f32x4::from(0.0001)) & t2.cmp_le(t_max) & desc_pos;

        let take_t1 = t1.cmp_lt(t2) & t1_valid;

        let t = f32x4::merge(take_t1, t1, t2);

        f32x4::merge(t1_valid | t2_valid, t, miss)
    }

    fn get_shading_info(
        &self,
        hit: WHit,
        _half_pixel_size_at: &dyn Fn(f32x4) -> f32x4,
    ) -> (MaterialHandle, WShadingPoint) {
        let point = hit.point();
        let origin = WSequenced::sample_at(&self.transform_seq, hit.ray.time);
        let normal = (point - origin).normalized();
        (
            self.material,
            WShadingPoint::new(hit, point, f32x4::ZERO, normal),
        )
    }
}
