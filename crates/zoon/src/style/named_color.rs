use crate::*;

macro_rules! color {
    ($color:ident => $h:literal, $s:literal, $l:literal) => {
        pub static $color: HSLuv = hsluv!($h, $s, $l);
    }
}

// The palette based on https://tailwindcss.com/docs/customizing-colors

color!(GRAY_7 => 248.1, 32.2, 27.3);

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
