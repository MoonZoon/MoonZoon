use bumpalo::collections::Vec as BumpVec;
use bumpalo::Bump;

use crate::film::ChannelSample;
use crate::hitable::WShadingPoint;
use crate::material::{MaterialHandle, BSDF};
use crate::math::{f32x4, Vec2u, Vec3, Wec3};
use crate::ray::Ray;
use crate::spectrum::{Srgb, WSrgb};
use crate::world::World;
use crate::setup::VOLUME_MARCHES_PER_SAMPLE;

pub trait Integrator: Send + Sync {
    #[allow(clippy::too_many_arguments)]
    fn integrate(
        &self,
        world: &World,
        samples_1d: &[f32x4; 3 + VOLUME_MARCHES_PER_SAMPLE],
        samples_2d: &[f32x4; 12 + 8 * VOLUME_MARCHES_PER_SAMPLE],
        depth: usize,
        material: MaterialHandle,
        intersection: WShadingPoint,
        bump: &Bump,
        spawned_rays: &mut BumpVec<Ray>,
        output_samples: &mut BumpVec<(Vec2u, ChannelSample)>,
    );

    fn requested_1d_sample_sets(&self) -> usize;
    fn requested_2d_sample_sets(&self) -> usize;
}

#[derive(Clone, Copy)]
pub struct PathTracingIntegrator {
    pub max_bounces: usize,
    pub volume_marches: usize,
}

impl Integrator for PathTracingIntegrator {
    fn requested_1d_sample_sets(&self) -> usize {
        (self.max_bounces + 1) * (3 + self.volume_marches)
    }

    fn requested_2d_sample_sets(&self) -> usize {
        (self.max_bounces + 1) * (12 + 8 * self.volume_marches)
    }

    fn integrate(
        &self,
        world: &World,
        samples_1d: &[f32x4; 3 + VOLUME_MARCHES_PER_SAMPLE],
        samples_2d: &[f32x4; 12 + 8 * VOLUME_MARCHES_PER_SAMPLE],
        depth: usize,
        material: MaterialHandle,
        mut intersection: WShadingPoint,
        bump: &Bump,
        spawned_rays: &mut BumpVec<Ray>,
        output_samples: &mut BumpVec<(Vec2u, ChannelSample)>,
    ) {
        let wo = -intersection.ray.dir;
        let material = world.materials.get(material);

        let bsdf = material.get_bsdf_at(&intersection, bump);

        let volume_transmission = if let Some(rho_t) = world.volume_params.coeff_extinction {
            (f32x4::from(-rho_t) * intersection.t).exp()
        } else {
            f32x4::ONE
        };

        intersection.ray.radiance +=
            bsdf.le(wo, &intersection) * intersection.ray.throughput * volume_transmission;

        if bsdf.receives_light() && world.lights.len() > 0 {
            // let light_idx =
            //     (samples_1d[0].as_ref()[0] * (world.lights.len() as f32)).floor() as usize;
            let lights_to_sample = (samples_1d[0] * f32x4::from(world.lights.len() as f32)).floor();
            let lights_to_sample = lights_to_sample.as_ref().iter().map(|i| *i as usize);

            let correction_factor =
                f32x4::from(world.lights.len() as f32 / lights_to_sample.len() as f32);

            for (i, light_idx) in lights_to_sample.enumerate() {
                let li = surface_sample_one_light(
                    world,
                    light_idx,
                    arrayref::array_ref![samples_2d, 0 + i * 2, 2],
                    &intersection,
                    bsdf,
                );

                intersection.ray.radiance +=
                    li * intersection.ray.throughput * correction_factor * volume_transmission;
            }
        }

        if let Some(rho_s) = world.volume_params.coeff_scattering {
            let rho_s = f32x4::from(rho_s);

            for march in 0..self.volume_marches {
                let lights_to_sample =
                    (samples_1d[march + 1] * f32x4::from(world.lights.len() as f32)).floor();
                let lights_to_sample = lights_to_sample.as_ref().iter().map(|i| *i as usize);

                let correction_factor = f32x4::from(
                    world.lights.len() as f32
                        / lights_to_sample.len() as f32
                        / self.volume_marches as f32,
                );

                for (i, light_idx) in lights_to_sample.enumerate() {
                    let (li, t) = volume_sample_one_light(
                        world,
                        light_idx,
                        arrayref::array_ref![samples_2d, 8 + 8 * march + i * 2, 2],
                        samples_1d[1],
                        intersection.ray.origin,
                        intersection.ray.dir,
                        intersection.t,
                        intersection.ray.time,
                    );

                    let transmission = if let Some(rho_t) = world.volume_params.coeff_extinction {
                        (f32x4::from(-rho_t) * t).exp()
                    } else {
                        f32x4::ONE
                    };

                    intersection.ray.radiance +=
                        li * intersection.ray.throughput * correction_factor * rho_s * transmission;
                }
            }
        }

        if bsdf.receives_light() {
            let se = bsdf.scatter(
                wo,
                &intersection,
                samples_1d[3],
                arrayref::array_ref![samples_2d, 8 + 8 * self.volume_marches, 4],
            );

            let ndl = se.wi.dot(intersection.normal).abs();

            let mut new_throughput =
                intersection.ray.throughput * volume_transmission * se.f * ndl / se.pdf;

            let roulette_factor = if depth > 2 {
                let roulette_factor =
                    (f32x4::ONE - intersection.ray.throughput.max_channel()).max(f32x4::from(0.05));

                new_throughput /= f32x4::ONE - roulette_factor;

                roulette_factor
            } else {
                f32x4::ZERO
            };

            let mut new_rays: [Ray; 4] = intersection.create_rays(se.wi).into();
            let throughputs: [Srgb; 4] = new_throughput.into();

            if depth == 0 {
                let normals: [Vec3; 4] = intersection.normal.into();
                for (ray, normal) in new_rays.iter().zip(normals.iter()) {
                    if ray.valid {
                        output_samples.push((ray.tile_coord, ChannelSample::Alpha(1.0)));
                        output_samples.push((ray.tile_coord, ChannelSample::WorldNormal(*normal)));
                    }
                }
            }

            for (((ray, new_throughput), roulette_factor), roulette_sample) in new_rays
                .iter_mut()
                .zip(throughputs.iter())
                .zip(roulette_factor.as_ref().iter())
                .zip(samples_1d[4].as_ref().iter())
            {
                if ray.valid {
                    if depth >= self.max_bounces || *roulette_sample < *roulette_factor {
                        output_samples.push((ray.tile_coord, ChannelSample::Color(ray.radiance)));
                    } else {
                        if !new_throughput.is_nan() {
                            ray.throughput = *new_throughput;
                        }

                        spawned_rays.push(*ray);
                    }
                }
            }
        } else {
            let final_rays: [Ray; 4] = intersection.ray.into();

            for ray in final_rays.iter() {
                if ray.valid {
                    let sample = if depth == 0 {
                        ChannelSample::Background(ray.radiance)
                    } else {
                        ChannelSample::Color(ray.radiance)
                    };

                    output_samples.push((ray.tile_coord, sample));
                }
            }
        }
    }
}

pub fn surface_sample_one_light(
    world: &World,
    light_idx: usize,
    samples: &[f32x4; 2],
    intersection: &WShadingPoint,
    bsdf: &dyn BSDF,
) -> WSrgb {
    let (end_point, li, pdf) = world.lights[light_idx].sample(samples, intersection.point);

    let wo = -intersection.ray.dir;
    let wi = end_point - intersection.point;
    let dist = wi.mag();
    let wi = wi / dist;

    // Offset from surface to avoid shadow acne
    let occlude_point = intersection.point
        + intersection.normal * intersection.normal.dot(wi).signum() * intersection.offset_by;

    // check occlusion
    let occluded = world
        .hitables
        .test_occluded(occlude_point, end_point, intersection.ray.time);

    let f = bsdf.f(wo, wi, intersection.normal) * intersection.normal.dot(wi).max(f32x4::ZERO);

    // volume transmission
    let transmission = if let Some(rho_t) = world.volume_params.coeff_extinction {
        (f32x4::from(-rho_t) * dist).exp()
    } else {
        f32x4::ONE
    };

    li * f * transmission * occluded / pdf
}

pub fn volume_sample_one_light(
    world: &World,
    light_idx: usize,
    light_samples: &[f32x4; 2],
    volume_sample: f32x4,
    ray_o: Wec3,
    ray_d: Wec3,
    max_distance: f32x4,
    time: f32x4,
) -> (WSrgb, f32x4) {
    let light = &world.lights[light_idx];

    let (vol_sample_dist, vol_sample_pdf) =
        light.sample_volume_scattering(volume_sample, ray_o, ray_d, max_distance);

    let sampled_point = ray_o + ray_d * vol_sample_dist;

    let (end_point, li, light_pdf) = light.sample(light_samples, sampled_point);

    let wi = end_point - sampled_point;
    let dist_point_to_light = wi.mag();
    // let wi = wi / dist;

    // check occlusion
    let occluded = world.hitables.test_occluded(sampled_point, end_point, time);

    let f = f32x4::ONE / (f32x4::from(4.0) * f32x4::PI);

    // volume transmission
    let transmission = if let Some(rho_t) = world.volume_params.coeff_extinction {
        (f32x4::from(-rho_t) * dist_point_to_light).exp()
    } else {
        f32x4::ONE
    };

    (
        li * f * transmission * occluded / (vol_sample_pdf * light_pdf),
        vol_sample_dist,
    )
}
