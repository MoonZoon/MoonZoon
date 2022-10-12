// Inspirations:
// - https://easings.net/
// - https://docs.rs/interpolation/0.2.0/interpolation/
// - https://svelte.dev/repl/6904f0306d6f4985b55f5f9673f762ef?version=3.4.1

// @TODO add other functions

pub fn cubic_out(x: impl Into<f64>) -> f64 {
    1. - (1. - x.into()).powi(3)
}

pub fn linear(x: impl Into<f64>) -> f64 {
    x.into()
}
