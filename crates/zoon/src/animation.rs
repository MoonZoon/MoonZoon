mod animation_loop;
pub use animation_loop::AnimationLoop;

pub mod ease;
pub mod interpolate;

mod oscillator;
pub use oscillator::Oscillator;

mod tweened;
pub use tweened::Tweened;
