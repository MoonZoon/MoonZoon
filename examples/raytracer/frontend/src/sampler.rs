use rand::prelude::*;

use ultraviolet::f32x4;

// pub trait Sampler {
//     fn begin_pixel(&mut self, pixel: Vec2u);
//     fn request_samples_2d(&mut self, samples: usize);
//     fn request_samples_1d(&mut self, samples: usize);
// }

pub struct Samples {
    pub samples: usize,
    pub samples_1d: Vec<f32>,
    pub samples_2d: Vec<f32>,
}

impl Samples {
    pub fn new_rd(samples: usize, sets_1d: usize, sets_2d: usize, offset: u64) -> Self {
        let mut samples_1d = vec![0f32; samples * sets_1d];
        let mut samples_2d = vec![0f32; samples * 2 * sets_2d];

        for i in 0..sets_1d {
            let mut seq_1d = quasi_rd::Sequence::new_with_offset(1, (offset + i as u64) << 32);
            seq_1d.fill_with_samples_f32(&mut samples_1d[samples * i..samples * (i + 1)]);
        }

        for i in 0..sets_2d {
            let mut seq_2d = quasi_rd::Sequence::new_with_offset(2, (offset + sets_1d as u64 + i as u64) << 32);
            seq_2d.fill_with_samples_f32(&mut samples_2d[samples * 2 * i..2 * samples * (i + 1)]);
        }

        Self {
            samples,
            samples_1d,
            samples_2d,
        }
    }

    #[allow(dead_code)]
    pub fn new_random(samples: usize, sets_1d: usize, sets_2d: usize) -> Self {
        let mut samples_1d = vec![0f32; samples * sets_1d];
        let mut samples_2d = vec![0f32; samples * 2 * sets_2d];

        let mut rng = SmallRng::from_rng(thread_rng()).unwrap();

        for s in samples_1d.iter_mut() {
            *s = rng.gen();
        }

        for s in samples_2d.iter_mut() {
            *s = rng.gen();
        }

        Self {
            samples,
            samples_1d,
            samples_2d,
        }
    }

    #[inline]
    pub fn sample_1d(&self, sample: usize, scramble: f32, set: usize) -> f32 {
        (self.samples_1d[sample + self.samples * set] + scramble).fract()
    }

    #[inline]
    pub fn wide_sample_1d(&self, start_sample: usize, scramble: f32, set: usize) -> f32x4 {
        f32x4::from([
            self.sample_1d(start_sample, scramble, set),
            self.sample_1d(start_sample + 1, scramble, set),
            self.sample_1d(start_sample + 2, scramble, set),
            self.sample_1d(start_sample + 3, scramble, set),
        ])
    }

    #[inline]
    pub fn wide_sample_1d_array(
        &self,
        samples: [usize; 4],
        scrambles: [f32; 4],
        set: usize,
    ) -> f32x4 {
        f32x4::from([
            self.sample_1d(samples[0], scrambles[0], set),
            self.sample_1d(samples[1], scrambles[1], set),
            self.sample_1d(samples[2], scrambles[2], set),
            self.sample_1d(samples[3], scrambles[3], set),
        ])
    }

    #[inline]
    pub fn sample_2d(&self, dim: usize, sample: usize, scramble: f32, set: usize) -> f32 {
        (self.samples_2d[dim + sample * 2 + self.samples * 2 * set] + scramble).fract()
    }

    #[inline]
    pub fn wide_sample_2d(
        &self,
        dim: usize,
        start_sample: usize,
        scramble: f32,
        set: usize,
    ) -> f32x4 {
        f32x4::from([
            self.sample_2d(dim, start_sample, scramble, set),
            self.sample_2d(dim, start_sample + 1, scramble, set),
            self.sample_2d(dim, start_sample + 2, scramble, set),
            self.sample_2d(dim, start_sample + 3, scramble, set),
        ])
    }

    #[inline]
    pub fn wide_sample_2d_array(
        &self,
        dim: usize,
        samples: [usize; 4],
        scrambles: [f32; 4],
        set: usize,
    ) -> f32x4 {
        f32x4::from([
            self.sample_2d(dim, samples[0], scrambles[0], set),
            self.sample_2d(dim, samples[1], scrambles[1], set),
            self.sample_2d(dim, samples[2], scrambles[2], set),
            self.sample_2d(dim, samples[3], scrambles[3], set),
        ])
    }
}
