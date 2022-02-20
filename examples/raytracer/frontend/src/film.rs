use bumpalo::{collections::Vec as BumpVec, Bump};

use generic_array::{ArrayLength, GenericArray};

use rand::prelude::*;

use crate::camera::CameraHandle;
use crate::filter::{Filter, FilterImportanceSampler};
use crate::hitable::HitStore;
use crate::integrator::Integrator;
use crate::math::{f32x4, Bounds2u, Extent2u, Vec2, Vec2u, Vec3, Wec2};
use crate::ray::{Ray, WRay};
use crate::sampler::Samples;
use crate::spectrum::Srgb;
use crate::world::World;
use crate::setup::VOLUME_MARCHES_PER_SAMPLE;

use std::collections::hash_map::HashMap;
use std::ops::Range;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc, Mutex,
};

macro_rules! declare_channels {
    {
        $($name:ident => {
            storage: $storage:ident,
            init: $initialize:expr,
        }),+
    } => {
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub enum ChannelKind {
            $($name,)+
        }

        #[derive(Debug)]
        pub enum ChannelSample {
            $($name($storage),)+
        }

        pub enum ChannelTileStorage {
            $($name(Vec<$storage>),)+
        }

        impl ChannelTileStorage {
            fn new(kind: ChannelKind, res: Extent2u) -> Self {
                use ChannelKind::*;
                match kind {
                    $($name => Self::$name(vec![($initialize); (res.w * res.h) as usize]),)+
                }
            }

            fn add_sample(&mut self, idx: usize, sample: &ChannelSample) {
                match (self, sample) {
                    $((ChannelTileStorage::$name(ref mut buf), ChannelSample::$name(sample)) => {
                        buf[idx] += *sample;
                    },)+
                    _ => (),
                }
            }
        }

        pub enum ChannelStorage {
            $($name(Vec<$storage>),)+
        }

        impl ChannelStorage {
            fn new(kind: ChannelKind, res: Extent2u) -> Self {
                use ChannelKind::*;
                match kind {
                    $($name => Self::$name(vec![$initialize; (res.w * res.h) as usize]),)+
                }
            }

            pub fn kind(&self) -> ChannelKind {
                match *self {
                    $( ChannelStorage::$name(_) => ChannelKind::$name, )+
                }
            }

            pub fn copy_from_tile(&mut self, other: &ChannelTileStorage, full_res: Extent2u, tile_bounds: Bounds2u, samples: usize) -> Result<(), ()> {
                let extent = tile_bounds.size();
                match (self, other) {
                    $( (ChannelStorage::$name(this_buf), ChannelTileStorage::$name(tile_buf)) => {
                        for x in 0..extent.w {
                            for y in 0..extent.h {
                                let tile_idx = x + y * extent.w;
                                let this_idx = (tile_bounds.min.x + x) + (tile_bounds.min.y + y) * full_res.w;
                                let tile_samp_sum = tile_buf[tile_idx as usize];
                                this_buf[this_idx as usize] = tile_samp_sum / samples as f32;
                            }
                        }
                        Ok(())
                    }, )+
                    _ => Err(())
                }
            }
        }
    }
}

declare_channels! {
    Color => {
        storage: Srgb,
        init: Srgb::zero(),
    },
    Alpha => {
        storage: f32,
        init: 0f32,
    },
    Background => {
        storage: Srgb,
        init: Srgb::zero(),
    },
    WorldNormal => {
        storage: Vec3,
        init: Vec3::zero(),
    }
}

macro_rules! channel_storage_index {
    ($storage:expr, $channel:ident, $idx:expr) => {
        if let ChannelStorage::$channel(x) = &$storage[$idx] {
            x
        } else {
            panic!("Attempted to index into channel storage array with wrong channel type.");
        }
    };
}

pub struct Tile<N: ArrayLength<ChannelTileStorage>> {
    _index: usize,
    epoch: usize,
    channels: GenericArray<ChannelTileStorage, N>,
    raster_bounds: Bounds2u,
    raster_extent: Extent2u,
    screen_to_ndc_size: Vec2,
}

impl<N: ArrayLength<ChannelTileStorage>> Tile<N> {
    pub fn new<IC>(
        _index: usize,
        epoch: usize,
        channels: IC,
        res: Extent2u,
        raster_bounds: Bounds2u,
    ) -> Self
    where
        IC: std::iter::ExactSizeIterator<Item = ChannelKind>,
    {
        let screen_to_ndc_size = Vec2::new(1.0 / res.w as f32, 1.0 / res.h as f32);

        Tile {
            _index,
            epoch,
            channels: GenericArray::from_exact_iter(
                channels.map(|kind| ChannelTileStorage::new(kind, raster_bounds.size())),
            )
            .expect("Incorrect number of channels passed to tile creation"),
            raster_bounds,
            raster_extent: raster_bounds.size(),
            screen_to_ndc_size,
        }
    }

    pub fn add_sample(&mut self, tile_coord: Vec2u, sample: ChannelSample) {
        let idx = tile_coord.x + tile_coord.y * self.raster_extent.w;
        for channel in self.channels.iter_mut() {
            channel.add_sample(idx as usize, &sample);
        }
    }
}

pub struct Film<N: ArrayLength<ChannelStorage>> {
    channel_indices: HashMap<ChannelKind, usize>,
    channels: Mutex<GenericArray<ChannelStorage, N>>,
    progressive_epoch: usize,
    this_epoch_tiles_finished: AtomicUsize,
    res: Extent2u,
}

impl<'a, N: ArrayLength<ChannelStorage>> Film<N> {
    pub fn new(channels: &[ChannelKind], res: Extent2u) -> Result<Self, String> {
        let mut channel_indices = HashMap::new();
        for (i, kind) in channels.iter().enumerate() {
            if channel_indices.insert(*kind, i).is_some() {
                return Err(format!("Attempted to create multiple {:?} channels", *kind));
            }
        }
        Ok(Film {
            channel_indices,
            channels: Mutex::new(
                GenericArray::from_exact_iter(
                    channels.iter().map(|kind| ChannelStorage::new(*kind, res)),
                )
                .expect("Generic type length does not match the number of channels."),
            ),
            progressive_epoch: 0,
            this_epoch_tiles_finished: AtomicUsize::new(0),
            res,
        })
    }

    pub fn save_to<P: AsRef<std::path::Path>, IS: Into<String>>(
        &self,
        write_channels: &[ChannelKind],
        output_folder: P,
        base_name: IS,
        transparent_background: bool,
    ) -> Result<(), String> {
        use std::fs::DirBuilder;
        DirBuilder::new()
            .recursive(true)
            .create(output_folder.as_ref())
            .unwrap();

        let base_name = base_name.into();

        let channels = self.channels.lock().unwrap();

        for kind in write_channels.iter() {
            match *kind {
                ChannelKind::Color => {
                    let color_idx = self.channel_indices.get(&ChannelKind::Color);
                    let alpha_idx = self.channel_indices.get(&ChannelKind::Alpha);
                    let bg_idx = self.channel_indices.get(&ChannelKind::Background);

                    match (color_idx, alpha_idx, bg_idx, transparent_background) {
                        (Some(&color_idx), Some(&alpha_idx), _, true) => {
                            let color_buf = &channel_storage_index!(channels, Color, color_idx);
                            let alpha_buf = &channel_storage_index!(channels, Alpha, alpha_idx);
                            let mut img =
                                image::RgbaImage::new(self.res.w, self.res.h);
                            for (x, y, pixel) in img.enumerate_pixels_mut() {
                                let idx = x + (self.res.h - 1 - y) * self.res.w;
                                let col = color_buf[idx as usize];
                                let a = alpha_buf[idx as usize];
                                let rgb = (col).saturated().gamma_corrected(2.2);
                                *pixel = image::Rgba([
                                    (rgb.x * 255.0).min(255.0).max(0.0) as u8,
                                    (rgb.y * 255.0).min(255.0).max(0.0) as u8,
                                    (rgb.z * 255.0).min(255.0).max(0.0) as u8,
                                    (a * 255.0).min(255.0).max(0.0) as u8,
                                ]);
                            }
                            let filename = output_folder
                                .as_ref()
                                .join(format!("{}_color.png", base_name.clone()));
                            println!("Saving to {}...", filename.display());
                            img.save(filename).unwrap();
                        }
                        (Some(&color_idx), _, Some(&bg_idx), false) => {
                            let color_buf = channel_storage_index!(channels, Color, color_idx);
                            let bg_buf = channel_storage_index!(channels, Background, bg_idx);
                            let mut img =
                                image::RgbImage::new(self.res.w, self.res.h);
                            for (x, y, pixel) in img.enumerate_pixels_mut() {
                                let i = x + (self.res.h - 1 - y) * self.res.w;
                                let col = color_buf[i as usize];
                                let bg = bg_buf[i as usize];
                                let rgb = (col + bg).saturated().gamma_corrected(2.2);
                                *pixel = image::Rgb([
                                    (rgb.x * 255.0).min(255.0).max(0.0) as u8,
                                    (rgb.y * 255.0).min(255.0).max(0.0) as u8,
                                    (rgb.z * 255.0).min(255.0).max(0.0) as u8,
                                ]);
                            }
                            let filename = output_folder
                                .as_ref()
                                .join(format!("{}_color.png", base_name.clone()));
                            println!("Saving to {}...", filename.display());
                            img.save(filename).unwrap();
                        }
                        (Some(&color_idx), _, None, false) => {
                            let color_buf = channel_storage_index!(channels, Color, color_idx);
                            let mut img =
                                image::RgbImage::new(self.res.w, self.res.h);
                            for (x, y, pixel) in img.enumerate_pixels_mut() {
                                let idx = x + (self.res.h - 1 - y) * self.res.w;
                                let rgb = color_buf[idx as usize].gamma_corrected(2.2);
                                *pixel = image::Rgb([
                                    (rgb.x * 255.0).min(255.0).max(0.0) as u8,
                                    (rgb.y * 255.0).min(255.0).max(0.0) as u8,
                                    (rgb.z * 255.0).min(255.0).max(0.0) as u8,
                                ]);
                            }
                            let filename = output_folder
                                .as_ref()
                                .join(format!("{}_color.png", base_name.clone()));
                            println!("Saving to {}...", filename.display());
                            img.save(filename).unwrap();
                        }
                        _ => {
                            return Err(String::from(
                                "Attempted to write Color channel with insufficient channels",
                            ))
                        }
                    }
                }
                ChannelKind::Background => {
                    let idx = *self
                        .channel_indices
                        .get(&ChannelKind::Background)
                        .ok_or_else(|| {
                            String::from(
                                "Attempted to write Background channel but it didn't exist",
                            )
                        })?;
                    let buf = channel_storage_index!(channels, Background, idx);
                    let mut img = image::RgbImage::new(self.res.w, self.res.h);
                    for (x, y, pixel) in img.enumerate_pixels_mut() {
                        let idx = x + (self.res.h - 1 - y) * self.res.w;
                        let rgb = buf[idx as usize].saturated().gamma_corrected(2.2);
                        *pixel = image::Rgb([
                            (rgb.x * 255.0).min(255.0).max(0.0) as u8,
                            (rgb.y * 255.0).min(255.0).max(0.0) as u8,
                            (rgb.z * 255.0).min(255.0).max(0.0) as u8,
                        ]);
                    }
                    let filename = output_folder
                        .as_ref()
                        .join(format!("{}_background.png", base_name.clone()));
                    println!("Saving to {}...", filename.display());
                    img.save(filename).unwrap();
                }
                ChannelKind::WorldNormal => {
                    let idx = *self
                        .channel_indices
                        .get(&ChannelKind::WorldNormal)
                        .ok_or_else(|| {
                            String::from(
                                "Attempted to write WorldNormal channel but it didn't exist",
                            )
                        })?;
                    let buf = channel_storage_index!(channels, WorldNormal, idx);
                    let mut img = image::RgbImage::new(self.res.w, self.res.h);
                    for (x, y, pixel) in img.enumerate_pixels_mut() {
                        let idx = x + (self.res.h - 1 - y) * self.res.w;
                        let vec = buf[idx as usize];
                        let rgb = Srgb::from(vec * 0.5 + Vec3::new(0.5, 0.5, 0.5));
                        *pixel = image::Rgb([
                            (rgb.x * 255.0).min(255.0).max(0.0) as u8,
                            (rgb.y * 255.0).min(255.0).max(0.0) as u8,
                            (rgb.z * 255.0).min(255.0).max(0.0) as u8,
                        ]);
                    }
                    let filename = output_folder
                        .as_ref()
                        .join(format!("{}_normal.png", base_name.clone()));
                    println!("Saving to {}...", filename.display());
                    img.save(filename).unwrap();
                }
                ChannelKind::Alpha => {
                    let idx = *self
                        .channel_indices
                        .get(&ChannelKind::Alpha)
                        .ok_or_else(|| {
                            String::from("Attempted to write Alpha channel but it didn't exist")
                        })?;
                    let buf = channel_storage_index!(channels, Alpha, idx);
                    let mut img = image::GrayImage::new(self.res.w, self.res.h);
                    for (x, y, pixel) in img.enumerate_pixels_mut() {
                        let idx = x + (self.res.h - 1 - y) * self.res.w;
                        let a = buf[idx as usize];
                        *pixel = image::Luma([(a * 255.0).min(255.0).max(0.0) as u8]);
                    }
                    let filename = output_folder
                        .as_ref()
                        .join(format!("{}_alpha.png", base_name.clone()));
                    println!("Saving to {}...", filename.display());
                    img.save(filename).unwrap();
                }
            }
        }
        Ok(())
    }
}

impl<'a, N: ArrayLength<ChannelStorage> + ArrayLength<ChannelTileStorage>> Film<N> {
    #[allow(clippy::too_many_arguments)]
    pub fn render_frame_into<I, F>(
        &'a mut self,
        world: &World,
        camera: CameraHandle,
        integrator: &I,
        filter: &F,
        tile_size: Extent2u,
        frame: usize,
        time_range: Range<f32>,
        samples: usize,
    ) where
        F: Filter + Copy + Send,
        I: Integrator,
    {
        let camera = world.cameras.get(camera);
        let mut tiles = Vec::new();

        let rem = Vec2u::new((self.res.w) % tile_size.w, (self.res.h) % tile_size.h);
        {
            let mut idx = 0;
            let channels = self.channels.lock().unwrap();
            for tile_x in 0..((self.res.w + rem.x) / tile_size.w) {
                for tile_y in 0..((self.res.h + rem.y) / tile_size.h) {
                    let start = Vec2u::new(tile_x * tile_size.w, tile_y * tile_size.h);
                    let end = Vec2u::new(
                        (start.x + tile_size.w).min(self.res.w),
                        (start.y + tile_size.h).min(self.res.h),
                    );
                    let tile_bounds = Bounds2u {
                        min: start,
                        max: end,
                    };

                    let tile = Tile::new(
                        idx,
                        self.progressive_epoch,
                        channels.iter().map(|c| c.kind()),
                        self.res,
                        tile_bounds,
                    );
                    tiles.push(tile);

                    idx += 1;
                }
            }
        }

        let fis = FilterImportanceSampler::new(filter);

        let sets_1d = 1 + integrator.requested_1d_sample_sets();
        let sets_2d = 2 + integrator.requested_2d_sample_sets();

        let sample_sets = Samples::new_rd(4 * samples, sets_1d, sets_2d, frame as u64);
        // let sample_sets = Samples::new_random(4 * samples, sets_1d, sets_2d);

        let width = self.res.w;

        self.integrate_tiles(tiles, samples * 4, |tile| {
            // let mut rng = SmallRng::from_rng(thread_rng()).unwrap();
            // let offset = (tile.index as u64) << 32;

            let ray_bump = Bump::new();
            let mut spawned_rays = BumpVec::new_in(&ray_bump);
            let mut spawned_wrays = BumpVec::new_in(&ray_bump);
            let shading_point_bump = Bump::new();
            let mut wintersections = BumpVec::new_in(&shading_point_bump);
            let sample_bump = Bump::new();
            let mut new_samples = BumpVec::new_in(&sample_bump);
            let hit_bump = Bump::new();
            let mut hit_store = HitStore::from_hitable_store(&hit_bump, &world.hitables);
            let mut bsdf_bump = Bump::new();

            let time_range_range = f32x4::from(time_range.end - time_range.start);

            for x in tile.raster_bounds.min.x..tile.raster_bounds.max.x {
                for y in tile.raster_bounds.min.y..tile.raster_bounds.max.y {
                    let tile_coord = Vec2u::new(x, y) - tile.raster_bounds.min;

                    let mut rng = SmallRng::seed_from_u64((x + y * width) as u64);
                    let scramble = rng.gen();

                    for samp in 0..samples {
                        let sample_nums = [4 * samp, 4 * samp + 1, 4 * samp + 2, 4 * samp + 3];

                        let ndcs = Wec2::from([
                            sample_uv(
                                x,
                                y,
                                tile.screen_to_ndc_size,
                                &fis,
                                &[
                                    sample_sets.sample_2d(0, sample_nums[0], scramble, 0),
                                    sample_sets.sample_2d(1, sample_nums[0], scramble, 0),
                                ],
                            ),
                            sample_uv(
                                x,
                                y,
                                tile.screen_to_ndc_size,
                                &fis,
                                &[
                                    sample_sets.sample_2d(0, sample_nums[1], scramble, 0),
                                    sample_sets.sample_2d(1, sample_nums[1], scramble, 0),
                                ],
                            ),
                            sample_uv(
                                x,
                                y,
                                tile.screen_to_ndc_size,
                                &fis,
                                &[
                                    sample_sets.sample_2d(0, sample_nums[2], scramble, 0),
                                    sample_sets.sample_2d(1, sample_nums[2], scramble, 0),
                                ],
                            ),
                            sample_uv(
                                x,
                                y,
                                tile.screen_to_ndc_size,
                                &fis,
                                &[
                                    sample_sets.sample_2d(0, sample_nums[3], scramble, 0),
                                    sample_sets.sample_2d(1, sample_nums[3], scramble, 0),
                                ],
                            ),
                        ]);

                        let times = f32x4::from(time_range.start)
                            + time_range_range
                            // * f32x4::from(rng.gen::<[f32; 4]>());
                            * sample_sets.wide_sample_1d(sample_nums[0], scramble, 0);

                        let rays = camera.get_rays(
                            scramble,
                            sample_nums,
                            tile_coord,
                            ndcs,
                            times,
                            &[
                                sample_sets.wide_sample_2d(0, sample_nums[0], scramble, 1),
                                sample_sets.wide_sample_2d(1, sample_nums[0], scramble, 1),
                            ],
                        );

                        spawned_wrays.push(rays);
                    }
                }
            }

            for depth in 0.. {
                bsdf_bump.reset();

                if spawned_wrays.is_empty() {
                    break;
                }

                hit_store.reset();

                let half_pixel_size_at: Box<dyn Fn(f32x4) -> f32x4> = if depth == 0 {
                    Box::new(
                        #[inline]
                        |t: f32x4| camera.half_pixel_size_at(t),
                    )
                // Box::new(|_t| f32x4::from(0.0001))
                } else {
                    Box::new(
                        #[inline]
                        |t| f32x4::from(0.0001 * 2.0 * depth as f32) * t,
                    )
                };

                for wray in spawned_wrays.drain(..) {
                    world.hitables.add_hits(
                        wray,
                        f32x4::from(crate::setup::WORLD_RADIUS * 2.0),
                        &mut hit_store,
                        &half_pixel_size_at,
                    );
                }

                hit_store.process_hits(&world.hitables, &mut wintersections, &half_pixel_size_at);

                for (mat_id, wshading_point) in wintersections.drain(..) {
                    let mut samples_1d = [f32x4::ZERO; 3 + VOLUME_MARCHES_PER_SAMPLE];
                    let num_1d_samples = samples_1d.len();

                    for (set, sample) in samples_1d.iter_mut().enumerate() {
                        *sample = sample_sets.wide_sample_1d_array(
                            wshading_point.ray.sample,
                            wshading_point.ray.scramble,
                            1 + set + depth * num_1d_samples,
                        );
                    }

                    let mut samples_2d = [f32x4::ZERO; 12 + 8 * VOLUME_MARCHES_PER_SAMPLE];
                    let num_2d_samples = samples_2d.len();

                    for (i, sample) in samples_2d.iter_mut().enumerate() {
                        let dim = i % 2;
                        let set = i / 2;

                        *sample = sample_sets.wide_sample_2d_array(
                            dim,
                            wshading_point.ray.sample,
                            wshading_point.ray.scramble,
                            2 + set + depth * num_2d_samples / 2,
                        );
                    }

                    integrator.integrate(
                        world,
                        &samples_1d,
                        &samples_2d,
                        depth,
                        mat_id,
                        wshading_point,
                        &bsdf_bump,
                        &mut spawned_rays,
                        &mut new_samples,
                    );
                }

                for (tile_coord, sample) in new_samples.drain(..) {
                    tile.add_sample(tile_coord, sample);
                }

                while spawned_rays.len() % 4 != 0 {
                    spawned_rays.push(Ray::new_invalid());
                }

                for rays in spawned_rays[0..].chunks_exact(4) {
                    // Safe because we just ensured that it has the correct length
                    let wray = WRay::from(unsafe {
                        [
                            *rays.get_unchecked(0),
                            *rays.get_unchecked(1),
                            *rays.get_unchecked(2),
                            *rays.get_unchecked(3),
                        ]
                    });

                    spawned_wrays.push(wray);
                }
                spawned_rays.clear();
            }
        });
    }

    fn integrate_tiles<FN>(&mut self, tiles: Vec<Tile<N>>, samples: usize, integrate_tile: FN)
    where
        FN: FnOnce(&mut Tile<N>) + Send + Sync + Copy,
    {
        let num_tiles = tiles.len();

        let pb = Arc::new(Mutex::new(pbr::ProgressBar::new(num_tiles as u64)));

        {
            let this = &*self;
            rayon::scope_fifo(|scope| {
                for mut tile in tiles.into_iter() {
                    let pb = pb.clone();
                    scope.spawn_fifo(move |_| {
                        integrate_tile(&mut tile);

                        this.tile_finished(tile, samples, pb)
                    })
                }
            });
        }

        while !self.this_epoch_tiles_finished.load(Ordering::Relaxed) == num_tiles {
            std::thread::sleep(std::time::Duration::from_millis(10));
        }

        self.this_epoch_tiles_finished.store(0, Ordering::Relaxed);
        self.progressive_epoch += 1;
    }

    fn tile_finished(
        &self,
        tile: Tile<N>,
        samples: usize,
        pb: Arc<Mutex<pbr::ProgressBar<std::io::Stdout>>>,
    ) {
        if self.progressive_epoch != tile.epoch {
            panic!(
                "Epoch mismatch! Expected: {}, got: {}",
                self.progressive_epoch, tile.epoch
            );
        }

        let mut pb = pb.lock().unwrap();
        pb.inc();

        let mut channels = self.channels.lock().unwrap();

        let Tile {
            channels: tile_channels,
            raster_bounds: tile_bounds,
            ..
        } = tile;

        for (tile_channel, channel) in tile_channels.iter().zip(channels.iter_mut()) {
            // Safe because we guarantee that we won't start modifying this chunk again
            // until the next epoch.
            channel
                .copy_from_tile(tile_channel, self.res, tile_bounds, samples)
                .unwrap();
        }
    }
}

#[inline]
fn sample_uv(
    x: u32,
    y: u32,
    screen_to_ndc_size: Vec2,
    fis: &FilterImportanceSampler,
    samples: &[f32; 2],
) -> Vec2 {
    let uv_samp = Vec2::new(samples[0], samples[1]);
    let fis_samp = Vec2::new(fis.sample(uv_samp.x), fis.sample(uv_samp.y));

    let screen_coord = Vec2::new(x as f32 + 0.5, y as f32 + 0.5) + fis_samp;
    // let screen_coord = Vec2::new(x as f32 + uv_samp.x, y as f32 + uv_samp.y);

    screen_to_ndc_size * screen_coord
}
