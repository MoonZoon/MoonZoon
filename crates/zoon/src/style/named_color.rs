//! Set of colors available for use in Zoon. They can be used in every methods
//! accepting [HSLuv] colors. The palette is based on <https://tailwindcss.com/docs/customizing-colors>
//! # Example
//! ```no_run
//! use zoon::{named_color::*, *};
//!
//! let (hovered, hovered_signal) = Mutable::new_and_signal(false);
//!
//! let button = Button::new()
//!     .s(Background::new().color_signal(hovered_signal.map_bool(|| GREEN_7, || GREEN_8)))
//!     .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered));
//! ```
use crate::*;

/// Create a `static` `HSluv` color from a color name, hue, saturation and lightness.
macro_rules! color {
    ($color:ident => $h:literal, $s:literal, $l:literal) => {
        pub static $color: HSLuv = hsluv!($h, $s, $l);
    };
}

pub static TRANSPARENT: HSLuv = hsluv!(0, 0, 0, 0);

color!(GRAY_0 => 235.5, 22.1, 98.2);
color!(GRAY_1 => 248.2, 18.5, 96.2);
color!(GRAY_2 => 248.3, 16.7, 91.6);
color!(GRAY_3 => 243.9, 14.4, 85.1);
color!(GRAY_4 => 246.8, 12.5, 66.8);
color!(GRAY_5 => 249.7, 15.7, 47.9);
color!(GRAY_6 => 245, 25.3, 35.8);
color!(GRAY_7 => 248.1, 32.2, 27.3);
color!(GRAY_8 => 247.2, 42, 16.3);
color!(GRAY_9 => 254, 48.3, 8.3);

color!(RED_0 => 12.2, 90.1, 96.4);
color!(RED_1 => 12.2, 95.3, 92.1);
color!(RED_2 => 12.2, 97.2, 85.8);
color!(RED_3 => 12.2, 94.5, 76.3);
color!(RED_4 => 12.2, 90.4, 64.1);
color!(RED_5 => 12.2, 80.7, 55);
color!(RED_6 => 12.2, 87.2, 47.9);
color!(RED_7 => 12.2, 88.6, 40);
color!(RED_8 => 12.2, 84.2, 33.2);
color!(RED_9 => 158, 96.7, 28.9);

color!(YELLOW_0 => 75.8, 100, 95.8);
color!(YELLOW_1 => 74.8, 91.2, 95.7);
color!(YELLOW_2 => 72.7, 90.5, 91.4);
color!(YELLOW_3 => 66.3, 90.5, 85.9);
color!(YELLOW_4 => 58, 96.5, 80.7);
color!(YELLOW_5 => 44.3, 99.1, 72.2);
color!(YELLOW_6 => 34.5, 99.2, 59.9);
color!(YELLOW_7 => 27.3, 98, 46.9);
color!(YELLOW_8 => 25.7, 94.9, 37.5);
color!(YELLOW_9 => 25.9, 91.8, 30.8);

color!(GREEN_0 => 154.7, 66.4, 97.9);
color!(GREEN_1 => 150.7, 63.8, 95.0);
color!(GREEN_2 => 152.1, 53.6, 90.3);
color!(GREEN_3 => 153, 74.3, 83.9);
color!(GREEN_4 => 150.8, 92.6, 75.8);
color!(GREEN_5 => 149.9, 98.5, 66.8);
color!(GREEN_6 => 150.8, 99.3, 54.9);
color!(GREEN_7 => 153.8, 99.1, 44.4);
color!(GREEN_8 => 155.4, 97.8, 35.3);
color!(GREEN_9 => 158, 96.7, 28.9);

color!(BLUE_0 => 241.3, 100, 96.6);
color!(BLUE_1 => 243, 95.5, 92.2);
color!(BLUE_2 => 243.4, 97.3, 86.5);
color!(BLUE_3 => 244.3, 96.6, 78);
color!(BLUE_4 => 249.7, 94.4, 66.7);
color!(BLUE_5 => 256.1, 92.9, 55.6);
color!(BLUE_6 => 260, 92.8, 46.1);
color!(BLUE_7 => 261.7, 93.7, 39);
color!(BLUE_8 => 261.8, 89.6, 31.9);
color!(BLUE_9 => 260.4, 84.2, 27.1);

color!(PURPLE_0 => 272.7, 100, 96.3);
color!(PURPLE_1 => 273.5, 94.8, 93.2);
color!(PURPLE_2 => 272.5, 97.1, 87.3);
color!(PURPLE_3 => 273.1, 96.8, 77.2);
color!(PURPLE_4 => 273.6, 94.8, 64.6);
color!(PURPLE_5 => 273.7, 93.8, 51.6);
color!(PURPLE_6 => 274.2, 90.3, 43.4);
color!(PURPLE_7 => 274.1, 88.9, 37.4);
color!(PURPLE_8 => 274.2, 88.2, 31.1);
color!(PURPLE_9 => 274.6, 85.6, 25.2);

color!(PINK_0 => 330.9, 80, 96.5);
color!(PINK_1 => 329.6, 83.3, 93.6);
color!(PINK_2 => 331, 88.1, 87.5);
color!(PINK_3 => 334.9, 89.3, 77.8);
color!(PINK_4 => 340.2, 86.4, 65.5);
color!(PINK_5 => 346.3, 80.1, 56.9);
color!(PINK_6 => 353.4, 89.1, 49.3);
color!(PINK_7 => 356.6, 92.8, 41.8);
color!(PINK_8 => 356.5, 89.9, 34.6);
color!(PINK_9 => 355.2, 84.9, 29.1);
