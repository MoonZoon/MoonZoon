mod into_color;

pub mod color_space;

// Warning: Oklch works properly only on Safari >= 16.2
// https://github.com/saadeghi/daisyui/issues/2703#issuecomment-1969865934

pub use color_macro::color;
pub use color_space::{oklch, Oklch, OklchExt, Rgba};
pub use cssparser::{self, ToCss};
pub use cssparser_color::{self, Color};
pub use into_color::{IntoColor, IntoOptionColor};
