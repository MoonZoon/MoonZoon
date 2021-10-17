use crate::*;

macro_rules! color {
    ($color:ident => $h:literal, $s:literal, $l:literal) => {
        pub static $color: HSLuv = hsluv!($h, $s, $l);
    }
}

color!(GRAY_7 => 248.1, 32.2, 27.3);

color!(GREEN_0 => 0, 0, 0);
color!(GREEN_2 => 149.9, 98.5, 66.8);
color!(GREEN_5 => 152, 53.6, 90.3);


