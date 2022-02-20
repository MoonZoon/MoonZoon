use crate::animation::WSequenced;
use crate::math::{f32x4, RandomSample2d, Vec2, Vec2u, Wec2, Wec3};
use crate::ray::WRay;

pub trait Camera: Send + Sync {
    fn get_rays(
        &self,
        scramble: f32,
        sample_nums: [usize; 4],
        tile_coord: Vec2u,
        uv: Wec2,
        time: f32x4,
        samples: &[f32x4; 2],
    ) -> WRay;

    /// gets the pixel radius size (half-width) at some t value (distance) from the camera
    /// assumes that the distance is along a ray emitted from the camera.
    fn half_pixel_size_at(&self, t: f32x4) -> f32x4;
}

#[derive(Clone, Copy, Debug)]
pub struct CameraHandle(usize);

pub struct CameraStore(Vec<Box<dyn Camera>>);

impl CameraStore {
    pub fn new() -> Self {
        CameraStore(Vec::new())
    }

    pub fn add_camera(&mut self, material: Box<dyn Camera>) -> CameraHandle {
        self.0.push(material);
        CameraHandle(self.0.len() - 1)
    }

    pub fn get(&self, handle: CameraHandle) -> &dyn Camera {
        self.0.get(handle.0).map(|b| b.as_ref()).unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct PinholeCamera<O, A, U> {
    half_size: Wec2,
    // tan(hfov/2) / resolution.h
    half_pixel_size: f32x4,
    origin: O,
    at: A,
    up: U,
}

impl<O, A, U> PinholeCamera<O, A, U> {
    #[allow(dead_code)]
    pub fn new(
        resolution: Vec2,
        vfov: f32,
        origin: O,
        at: A,
        up: U,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let aspect = resolution.x / resolution.y;
        let half_width = aspect * half_height;
        let half_pixel_size = f32x4::from(half_height / resolution.y);
        PinholeCamera {
            half_size: Wec2::splat(Vec2::new(half_width, half_height)),
            half_pixel_size,
            origin,
            at,
            up,
        }
    }
}

impl<O, A, U> Camera for PinholeCamera<O, A, U>
where
    O: WSequenced<Wec3>,
    A: WSequenced<Wec3>,
    U: WSequenced<Wec3>,
{
    fn get_rays(
        &self,
        scramble: f32,
        sample_nums: [usize; 4],
        tile_coord: Vec2u,
        uv: Wec2,
        time: f32x4,
        _samples: &[f32x4; 2],
    ) -> WRay {
        let origin = self.origin.sample_at(time);
        let at = self.at.sample_at(time);
        let up = self.up.sample_at(time);

        let basis_w = (origin - at).normalized();
        let basis_u = up.cross(basis_w).normalized();
        let basis_v = basis_w.cross(basis_u);
        let lower_left = origin
            - basis_u * self.half_size.x
            - basis_v * self.half_size.y
            - basis_w;

        let horiz = basis_u * self.half_size.x * f32x4::from(2.0) * uv.x;
        let verti = basis_v * self.half_size.y * f32x4::from(2.0) * uv.y;

        WRay::new(
            origin,
            (lower_left + horiz + verti - origin).normalized(),
            time,
            [tile_coord, tile_coord, tile_coord, tile_coord],
            [true, true, true, true],
            [scramble, scramble, scramble, scramble],
            sample_nums,
        )
    }

    fn half_pixel_size_at(&self, t: f32x4) -> f32x4 {
        self.half_pixel_size * t
    }
}
#[derive(Clone, Copy)]
pub struct ThinLensCamera<A, O, LA, U, F> {
    half_size: Wec2,
    // 2.0 * tan(hfov/2) / resolution.h / 2.0
    half_pixel_size: f32x4,
    aperture: A,
    origin: O,
    at: LA,
    up: U,
    focus: F,
}

impl<A, O, LA, U, F> ThinLensCamera<A, O, LA, U, F> {
    #[allow(dead_code)]
    pub fn new(
        resolution: Vec2,
        vfov: f32,
        aperture: A,
        origin: O,
        at: LA,
        up: U,
        focus: F,
    ) -> Self {
        let theta = vfov * std::f32::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let aspect = resolution.x / resolution.y;
        let half_width = aspect * half_height;
        let half_pixel_size = f32x4::from(half_height / resolution.y);
        ThinLensCamera {
            half_size: Wec2::splat(Vec2::new(half_width, half_height)),
            half_pixel_size,
            aperture,
            origin,
            at,
            up,
            focus,
        }
    }
}

impl<A, O, LA, U, F> Camera for ThinLensCamera<A, O, LA, U, F>
where
    A: WSequenced<f32x4>,
    O: WSequenced<Wec3>,
    LA: WSequenced<Wec3>,
    U: WSequenced<Wec3>,
    F: WSequenced<Wec3>,
{
    fn get_rays(
        &self,
        scramble: f32,
        sample_nums: [usize; 4],
        tile_coord: Vec2u,
        uv: Wec2,
        time: f32x4,
        samples: &[f32x4; 2],
    ) -> WRay {
        let origin = self.origin.sample_at(time);
        let at = self.at.sample_at(time);
        let up = self.up.sample_at(time);
        let focus = self.focus.sample_at(time);
        let focus_dist = (focus - origin).mag();
        let aperture = self.aperture.sample_at(time);

        let basis_w = (origin - at).normalized();
        let basis_u = up.cross(basis_w).normalized();
        let basis_v = basis_w.cross(basis_u);
        let lower_left = origin
            - basis_u * self.half_size.x * focus_dist
            - basis_v * self.half_size.y * focus_dist
            - basis_w * focus_dist;

        let horiz = basis_u * self.half_size.x * focus_dist * f32x4::from(2.0) * uv.x;
        let verti = basis_v * self.half_size.y * focus_dist * f32x4::from(2.0) * uv.y;

        let rd = Wec2::rand_in_unit_disk(samples) * aperture;
        let offset = basis_u * rd.x + basis_v * rd.y;

        let origin = origin + offset;
        WRay::new(
            origin,
            (lower_left + horiz + verti - origin).normalized(),
            time,
            [tile_coord, tile_coord, tile_coord, tile_coord],
            [true, true, true, true],
            [scramble, scramble, scramble, scramble],
            sample_nums,
        )
    }

    fn half_pixel_size_at(&self, t: f32x4) -> f32x4 {
        self.half_pixel_size * t
    }
}

#[derive(Clone, Copy)]
pub struct OrthographicCamera<O, A, U> {
    half_size: Wec2,
    full_size: Wec2,
    half_pixel_size: f32x4,

    origin: O,
    at: A,
    up: U,
}

impl<O, A, U> OrthographicCamera<O, A, U> {
    #[allow(dead_code)]
    pub fn new(resolution: Vec2, vertical_size: f32, origin: O, at: A, up: U) -> Self {
        let aspect = resolution.x / resolution.y;
        let size = Vec2::new(vertical_size * aspect, vertical_size);
        let pixel_size = vertical_size / resolution.y;
        Self {
            half_size: Wec2::splat(size / 2.0),
            full_size: Wec2::splat(size),
            half_pixel_size: f32x4::from(pixel_size / 2.0),
            origin,
            at,
            up,
        }
    }
}

impl<O, A, U> Camera for OrthographicCamera<O, A, U>
where
    O: WSequenced<Wec3>,
    A: WSequenced<Wec3>,
    U: WSequenced<Wec3>,
{
    fn get_rays(
        &self,
        scramble: f32,
        sample_nums: [usize; 4],
        tile_coord: Vec2u,
        uv: Wec2,
        time: f32x4,
        _samples: &[f32x4; 2],
    ) -> WRay {
        let origin = self.origin.sample_at(time);
        let at = self.at.sample_at(time);
        let up = self.up.sample_at(time);

        let basis_w = (at - origin).normalized();
        let basis_u = basis_w.cross(up).normalized();
        let basis_v = basis_u.cross(basis_w);
        let lower_left = origin - basis_u * self.half_size.x - basis_v * self.half_size.y;

        let offset = basis_u * uv.x * self.full_size.x + basis_v * uv.y * self.full_size.y;

        let origin = lower_left + offset;

        WRay::new(
            origin,
            basis_w,
            time,
            [tile_coord, tile_coord, tile_coord, tile_coord],
            [true, true, true, true],
            [scramble, scramble, scramble, scramble],
            sample_nums,
        )
    }

    fn half_pixel_size_at(&self, _t: f32x4) -> f32x4 {
        self.half_pixel_size
    }
}
