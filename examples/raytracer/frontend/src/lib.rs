use zoon::{*, println};

use generic_array::typenum::*;

mod animation;
mod camera;
mod film;
mod filter;
mod hitable;
mod integrator;
mod light;
mod material;
mod math;
mod ray;
mod sampler;
mod sdf;
mod spectrum;
mod sphere;
mod volume;
mod world;
mod setup;

use film::{ChannelKind, Film};
use filter::BlackmanHarrisFilter;
use integrator::PathTracingIntegrator;
use math::Extent2u;

use instant::Instant;

pub use wasm_bindgen_rayon::init_thread_pool;

fn root() -> impl Element {
    Button::new()
        .label("Start!")
        .on_press(render)
}

fn render() {
    // rayon::ThreadPoolBuilder::new()
    //     .num_threads(num_cpus::get())
    //     .build_global()
    //     .unwrap();

    let (camera, world) = setup::setup();

    let mut film = Film::<U4>::new(
        &[
            ChannelKind::Color,
            ChannelKind::Alpha,
            ChannelKind::Background,
            ChannelKind::WorldNormal,
        ],
        crate::setup::RESOLUTION
    )
    .unwrap();

    let frame_rate = 24;
    let frame_range = 1..2;
    let shutter_speed = 1.0 / 24.0;

    let filter = BlackmanHarrisFilter::new(1.5);
    // let filter = BoxFilter::default();
    let integrator = PathTracingIntegrator {
        max_bounces: crate::setup::MAX_INDIRECT_BOUNCES,
        volume_marches: crate::setup::VOLUME_MARCHES_PER_SAMPLE,
    };

    for frame in frame_range {
        let start = Instant::now();

        let frame_start = frame as f32 * (1.0 / frame_rate as f32);
        let frame_end = frame_start + shutter_speed;

        film.render_frame_into(
            &world,
            camera,
            &integrator,
            &filter,
            Extent2u::new(16, 16),
            frame,
            frame_start..frame_end,
            crate::setup::SAMPLES,
        );

        let time = Instant::now() - start;
        let time_secs = time.as_secs();
        let time_millis = time.subsec_millis();

        println!(
            "Done in {} seconds.",
            time_secs as f32 + time_millis as f32 / 1000.0
        );

        println!("Post processing image...");

        // film.save_to(
        //     &[
        //         ChannelKind::Alpha,
        //         ChannelKind::WorldNormal,
        //         ChannelKind::Color,
        //     ],
        //     "renders",
        //     format!("{}_spp", crate::setup::SAMPLES * 4),
        //     false,
        // )
        // .unwrap();
    }
    println!("Done!!!!");
}

#[wasm_bindgen]
pub fn start() {
    start_app("app", root);
}
