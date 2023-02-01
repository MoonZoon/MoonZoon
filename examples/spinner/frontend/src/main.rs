use zoon::{named_color::*, *};

fn root() -> impl Element {
    El::new()
        .s(Align::center())
        .child(spinner(50, 8, 10, YELLOW_6))
}

fn spinner(spinner_diameter: u32, dot_diameter: u32, dot_count: u32, color: HSLuv) -> impl Element {
    // ~= the higher speed of rotation
    let shortest_duration = Duration::seconds(1);
    // ~= the lowest speed of rotation
    let longest_duration = Duration::seconds(6);

    let rotation_oscillator = Oscillator::new(longest_duration);
    rotation_oscillator.cycle_wrap();

    let duration_oscillator = Oscillator::new(Duration::seconds(2));
    duration_oscillator.cycle();
    let rotation_duration_updater = Task::start_droppable(
        duration_oscillator
            .signal()
            .map(interpolate::linear(
                longest_duration.num_milliseconds() as f64,
                shortest_duration.num_milliseconds() as f64,
            ))
            .for_each_sync(clone!((rotation_oscillator) move |duration| {
                rotation_oscillator.set_duration(Duration::milliseconds(duration as i64));
            })),
    );

    Stack::new()
        .s(Height::exact(spinner_diameter))
        .s(Width::exact(spinner_diameter))
        .s(RoundedCorners::all_max())
        .s(Align::center())
        .s(Transform::with_signal(
            rotation_oscillator
                .signal()
                .map(|factor| Transform::new().rotate(factor * 360.)),
        ))
        .layers(dots(spinner_diameter, dot_diameter, dot_count, color))
        .after_remove(move |_| {
            drop(rotation_oscillator);
            drop(rotation_duration_updater);
        })
}

fn dots(
    spinner_diameter: u32,
    dot_diameter: u32,
    dot_count: u32,
    color: HSLuv,
) -> impl Iterator<Item = impl Element> {
    (0..dot_count).map(move |index| dot(index, spinner_diameter, dot_diameter, dot_count, color))
}

fn dot(
    index: u32,
    spinner_diameter: u32,
    dot_diameter: u32,
    dot_count: u32,
    color: HSLuv,
) -> impl Element {
    let dot_radius = dot_diameter as f64 / 2.;
    let spinner_radius = spinner_diameter as f64 / 2.;
    let circle_of_dot_centers = spinner_radius - dot_radius;

    let angle = (2. * std::f64::consts::PI / dot_count as f64) * index as f64;
    let (sin, cos) = angle.sin_cos();

    let x = cos * circle_of_dot_centers;
    let y = sin * circle_of_dot_centers;
    El::new()
        .s(Width::exact(dot_diameter))
        .s(Height::exact(dot_diameter))
        .s(Background::new().color(color))
        .s(RoundedCorners::all_max())
        .s(Align::center())
        .s(Transform::new().move_down(y).move_right(x))
}

fn main() {
    start_app("app", root);
}
