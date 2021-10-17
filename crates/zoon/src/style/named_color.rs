use crate::*;

macro_rules! color {
    ($color:ident => $h:literal, $s:literal, $l:literal) => {
        pub static $color: HSLuv = hsluv!($h, $s, $l);
    }
}

// The palette based on https://tailwindcss.com/docs/customizing-colors

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
