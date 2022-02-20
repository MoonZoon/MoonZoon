use crate::math::{f32x4, Vec2u, Vec3, Wec3};
use crate::spectrum::{Srgb, WSrgb};

macro_rules! rays {
    ($($n:ident => $t:ident, $st:ident, $tt:ident, $tc:ty, $bt:ty, $scramt:ty, $samplet:ty),+) => {
        $(#[derive(Clone, Copy, Debug)]
        pub struct $n {
            pub time: $tt,
            pub origin: $t,
            pub dir: $t,
            pub radiance: $st,
            pub throughput: $st,
            pub tile_coord: $tc,
            pub valid: $bt,
            pub scramble: $scramt,
            pub sample: $samplet,
        }

        impl $n {
            #[allow(dead_code)]
            #[inline(never)]
            pub fn point_at(&self, t: $tt) -> $t {
                self.dir.mul_add($t::new(t, t, t), self.origin)
            }
        })+
    }
}

rays!(Ray => Vec3, Srgb, f32, Vec2u, bool, f32, usize, WRay => Wec3, WSrgb, f32x4, [Vec2u; 4], [bool; 4], [f32; 4], [usize; 4]);

impl Ray {
    #[allow(dead_code)]
    pub fn new(
        origin: Vec3,
        dir: Vec3,
        time: f32,
        tile_coord: Vec2u,
        scramble: f32,
        sample: usize,
    ) -> Self {
        Self {
            time,
            origin,
            dir,
            radiance: Srgb::zero(),
            throughput: Srgb::one(),
            tile_coord,
            valid: true,
            scramble,
            sample,
        }
    }

    pub fn new_invalid() -> Self {
        Self {
            time: std::f32::NAN,
            origin: Vec3::broadcast(std::f32::NAN),
            dir: Vec3::broadcast(std::f32::NAN),
            radiance: Srgb::zero(),
            throughput: Srgb::zero(),
            tile_coord: Vec2u::zero(),
            valid: false,
            scramble: 0f32,
            sample: 0,
        }
    }
}

impl WRay {
    pub fn new(
        origin: Wec3,
        dir: Wec3,
        time: f32x4,
        tile_coord: [Vec2u; 4],
        valid: [bool; 4],
        scramble: [f32; 4],
        sample: [usize; 4],
    ) -> Self {
        Self {
            time,
            origin,
            dir,
            radiance: WSrgb::zero(),
            throughput: WSrgb::one(),
            tile_coord,
            valid,
            scramble,
            sample,
        }
    }

    #[allow(dead_code)]
    pub fn is_nan(&self) -> f32x4 {
        self.time.cmp_nan(self.time)
            | self.origin.x.cmp_nan(self.origin.x)
            | self.origin.y.cmp_nan(self.origin.y)
            | self.origin.z.cmp_nan(self.origin.z)
            | self.dir.x.cmp_nan(self.dir.x)
            | self.dir.y.cmp_nan(self.dir.y)
            | self.dir.z.cmp_nan(self.dir.z)
    }

    #[allow(dead_code)]
    pub fn is_nan_and_valid(&self) -> bool {
        let raynan = self.is_nan().move_mask();
        raynan != 0
            && !((raynan & 0b1000 == 0b1000) == self.valid[0])
            && !((raynan & 0b0100 == 0b0100) == self.valid[1])
            && !((raynan & 0b0010 == 0b0010) == self.valid[2])
            && !((raynan & 0b0001 == 0b0001) == self.valid[3])
    }
}

impl From<[Ray; 4]> for WRay {
    fn from(rays: [Ray; 4]) -> Self {
        Self {
            time: f32x4::from([rays[0].time, rays[1].time, rays[2].time, rays[3].time]),
            origin: Wec3::from([
                rays[0].origin,
                rays[1].origin,
                rays[2].origin,
                rays[3].origin,
            ]),
            dir: Wec3::from([rays[0].dir, rays[1].dir, rays[2].dir, rays[3].dir]),
            radiance: WSrgb::from([
                rays[0].radiance,
                rays[1].radiance,
                rays[2].radiance,
                rays[3].radiance,
            ]),
            throughput: WSrgb::from([
                rays[0].throughput,
                rays[1].throughput,
                rays[2].throughput,
                rays[3].throughput,
            ]),
            tile_coord: [
                rays[0].tile_coord,
                rays[1].tile_coord,
                rays[2].tile_coord,
                rays[3].tile_coord,
            ],
            valid: [rays[0].valid, rays[1].valid, rays[2].valid, rays[3].valid],
            scramble: [
                rays[0].scramble,
                rays[1].scramble,
                rays[2].scramble,
                rays[3].scramble,
            ],
            sample: [
                rays[0].sample,
                rays[1].sample,
                rays[2].sample,
                rays[3].sample,
            ],
        }
    }
}

impl Into<[Ray; 4]> for WRay {
    fn into(self) -> [Ray; 4] {
        let times = self.time.as_ref();
        let origins: [Vec3; 4] = self.origin.into();
        let dirs: [Vec3; 4] = self.dir.into();
        let throughputs: [Srgb; 4] = self.throughput.into();
        let radiances: [Srgb; 4] = self.radiance.into();
        [
            Ray {
                time: times[0],
                origin: origins[0],
                dir: dirs[0],
                radiance: radiances[0],
                throughput: throughputs[0],
                tile_coord: self.tile_coord[0],
                valid: self.valid[0],
                scramble: self.scramble[0],
                sample: self.sample[0],
            },
            Ray {
                time: times[1],
                origin: origins[1],
                dir: dirs[1],
                radiance: radiances[1],
                throughput: throughputs[1],
                tile_coord: self.tile_coord[1],
                valid: self.valid[1],
                scramble: self.scramble[1],
                sample: self.sample[1],
            },
            Ray {
                time: times[2],
                origin: origins[2],
                dir: dirs[2],
                radiance: radiances[2],
                throughput: throughputs[2],
                tile_coord: self.tile_coord[2],
                valid: self.valid[2],
                scramble: self.scramble[2],
                sample: self.sample[2],
            },
            Ray {
                time: times[3],
                origin: origins[3],
                dir: dirs[3],
                radiance: radiances[3],
                throughput: throughputs[3],
                tile_coord: self.tile_coord[3],
                valid: self.valid[3],
                scramble: self.scramble[3],
                sample: self.sample[3],
            },
        ]
    }
}
