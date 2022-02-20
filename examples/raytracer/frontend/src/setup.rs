use crate::{
    camera::{CameraStore, CameraHandle, PinholeCamera},
    light::{SphereLight, Light},
    world::World,
    material::MaterialStore,
    hitable::HitableStore,
    volume::VolumeParams,
    material::{Dielectric, Emissive, Sky},
    sphere::Sphere,
    math::{Extent2u, Vec2, Vec3},
    spectrum::Srgb,
    sdf::{MandelBox, BoxFold, SphereFold, TracedSDF},
};

// The resolution of the output image
pub const RESOLUTION: Extent2u = Extent2u::new(1280, 720);

// The number of samples per pixel. This number will actually get multiplied by 4, i.e. if you put 2
// for SAMPLES here, there will actually be 8 samples per pixel.
//
// Increase this for higher overall quality/less noise.
pub const SAMPLES: usize = 2;

// The number of marches (i.e. points along the ray) to sample for each ray for volume scattering.
pub const VOLUME_MARCHES_PER_SAMPLE: usize = 2;

// The number of times light will bounce around the scene to provide global illumination before
// being killed. Higher numbers of bounces are more expensive but will create a more "full" sense
// of global illumination.
pub const MAX_INDIRECT_BOUNCES: usize = 3;

// The radius of the world. You can probably just leave this at 100, but feel free to play and see what happens.
pub const WORLD_RADIUS: f32 = 100.0;

// The level of detail to render SDFs with (which is how the fractal is rendered).
// Closer to 0 = smaller detail will be shown. Larger means less detail.
pub const SDF_DETAIL_SCALE: f32 = 2.0;

// The number of iterations to run of the fractal function. More iterations will mean
// higher potential detail but also higher render times. If you use lower iterations, the
// surface of the fractal will be more sparsely defined, so you should use a higher SDF_DETAIL_SCALE
// in order to see it better, whereas with more iterations the surface will be more defined so you can
// use a lower (more detailed) SDF_DETAIL_SCALE.
pub const FRACTAL_ITERATIONS: usize = 12;

pub fn setup() -> (CameraHandle, World) {
    let mut materials = MaterialStore::new();
    let mut hitables = HitableStore::new();
    let mut lights: Vec<Box<dyn Light>> = Vec::new();

    // VOLUMETRICS
    // Volumetrics are very cool but are really expensive to render. You can change
    // these values or change them to None to disable that kind of volumetric effect.
    // Setting both to None will make the render significantly faster.
    let volume_params = VolumeParams {
        // coeff_scattering: None,
        // coeff_extinction: None,
        coeff_scattering: Some(0.25),
        coeff_extinction: Some(0.035),
    };

    // SKY
    let sky = materials.add_material(Sky::new(
        // You can change the following numbers to change the color of the sky. The first Srgb color is the
        // color of the top of the skydome while the second number is the color of the bottom... they will be
        // smoothly blended together towards the middle.
        Srgb::new(0.3, 0.4, 0.6),
        Srgb::new(0.2, 0.3, 0.6) * 0.05,
    ));

    hitables.push(Sphere::new(Vec3::new(0.0, 0.0, 0.0), WORLD_RADIUS, sky));

    // FRACTAL
    // Here you can change the material properties for the fractal. Try changing the Srgb color and the roughness, 
    // which should stay between 0.0 (completely smooth) and 1.0 (completely rough).
    let grey = materials.add_material(Dielectric::new_remap(Srgb::new(0.2, 0.2, 0.2), 0.6));

    hitables.push(TracedSDF::new(
        // Try playing around with these numbers, which will dramatically affect how the fractal in the middle looks! The commented line
        // below is the settings for the canonical, default 'Mandelbox', but changing them can make some really awesome and crazy shapes.
        // It can also make the fractal significantly grow or shrink, so you might have to move the camera in tandem with changing these settings
        // to get a good look!
        // MandelBox::new(FRACTAL_ITERATIONS, BoxFold::new(1.0), SphereFold::new(0.5, 1.0), -2.0)
        MandelBox::new(FRACTAL_ITERATIONS, BoxFold::new(1.0), SphereFold::new(0.01, 1.9), -2.1),
        grey,
    ));

    // SUN
    // Sun-like lights don't work very well with volumetrics yet,
    // but if you disable volumetrics then feel free to uncomment the below and play with it!
    // let bluesun = Srgb::new(1.5, 3.0, 5.0) * 30000.0;
    // lights.push(Box::new(SphereLight::new(
    //     Vec3::new(-2.0, 2.65, 1.0).normalized() * 99.0,
    //     1.0,
    //     bluesun,
    // )));

    // OTHER LIGHTS
    // Try playing with the colors below to change the colors of the lights.
    let green = Srgb::new(1.5, 4.5, 3.0).normalized();
    let blue = Srgb::new(1.5, 3.0, 4.5).normalized();
    let blue_emissive = materials.add_material(Emissive::new_splat(blue * 3.0));
    let green_emissive = materials.add_material(Emissive::new_splat(green * 3.0));

    // This defines a position and size for pairs of lights, which gets used in the for loop below.
    // Try playing with the positions and sizes :)
    let light_pairs = [
        (Vec3::new(1.2, -1.2, 1.2), 0.15),
        (Vec3::new(-1.2, 1.2, 1.2), 0.15),
    ];

    for &(pos, rad) in light_pairs.iter() {
        let mut green_pos = pos;
        green_pos.y *= -1.0;
        lights.push(Box::new(SphereLight::new(green_pos, rad, green * 40.0)));
        lights.push(Box::new(SphereLight::new(pos, rad, blue * 40.0)));
        hitables.push(Sphere::new(green_pos, rad - 0.01, green_emissive));
        hitables.push(Sphere::new(pos, rad - 0.01, blue_emissive));
    }

    lights.push(Box::new(SphereLight::new(Vec3::zero(), 0.25, green * 20.0)));
    hitables.push(Sphere::new(Vec3::zero(), 0.24, green_emissive));

    // CAMERA
    let res = Vec2::new(RESOLUTION.w as f32, RESOLUTION.h as f32);

    // Here you can play with the settings of the camera! You should leave the 'res'
    // alone, but feel free to play with the FOV, origin, etc.
    let camera = PinholeCamera::new(
        res,
        // The vertical FOV, in degrees, of the camera.
        60.0,
        // The origin, i.e. position of the camera
        Vec3::new(-0.45, 0.2, 2.0) * 2.25,
        // The location that the camera is pointing *at*
        Vec3::new(0.0, 0.0, 0.0),
        // The direction that the camera will try to align its "up" orientation to.
        // Generally you can leave this as is, unless you want to make a camera that is
        // pointing straight up or down, or if you want to rotate the camera
        // left/right around its own z axis.
        Vec3::new(0.0, 1.0, 0.0),
    );

    // You can also try using an Orthographic camera, which doesn't have
    // any perspective projection! The properties are the same as the PinholeCamera
    // except that instead of a field of view, we have the absolute vertical size
    // of the infinite rectangular prism that the orthographic camera will see.
    // let camera = OrthographicCamera::new(
    //     res,
    //     11.0 / 4.0,
    //     Vec3::new(9.5, -3.5, 9.5),
    //     Vec3::new(0.0, 0.8, 0.0),
    //     Vec3::new(0.0, 1.0, 0.0),
    // );

    let mut cameras = CameraStore::new();

    let camera = cameras.add_camera(Box::new(camera));

    (
        camera,
        World {
            materials,
            hitables,
            lights,
            cameras,
            volume_params,
        },
    )
}