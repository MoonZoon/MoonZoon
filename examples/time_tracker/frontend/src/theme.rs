use zoon::*;
use std::borrow::Cow;

#[static_ref]
pub fn theme() -> &'static Mutable<Theme> {
    Mutable::new(Theme::Light)
}

pub fn toggle_theme() {
    theme().update(|theme| match theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
    });
}

#[derive(Clone, Copy)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Clone, Copy)]
pub enum ThemeColor {
    Background0,
    Font0,
    Background1,
    Background1Highlighted,
    Font1,
    Border1,
    Background2,
    Background2Highlighted,
    Font2,
    Background3,
    Background3Highlighted,
    Font3,
    Background4,
    Background4Highlighted,
    Font4,
    Border4,
    Background5,
    Background5Highlighted,
    Font5,
    BackgroundInvalid,
    Shadow,
    Shadow2,
    Transparent,
}

impl Color<'_> for ThemeColor {}

impl<'a> IntoCowStr<'a> for ThemeColor {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            // 0) white / black
            ThemeColor::Background0 => hsl(0, 0, 100),
            ThemeColor::Font0 => hsla(0, 0, 0, 70),
            // 1) blue / white 
            ThemeColor::Background1 => hsl(256.1, 87.8, 49.6),
            ThemeColor::Background1Highlighted => hsl(257, 92.3, 44.9),
            ThemeColor::Font1 => hsla(0, 0, 100, 95),
            ThemeColor::Border1 => hsl(168.3, 100, 75.3),
            // 2) light gray / black 
            ThemeColor::Background2 => hsl(0, 0, 96.5),
            ThemeColor::Background2Highlighted => hsl(0, 0, 94.5),
            ThemeColor::Font2 => hsla(0, 0, 0, 70),
            // 3) green / white 
            ThemeColor::Background3 => hsl(168.3, 100, 75.3),
            ThemeColor::Background3Highlighted => hsl(168.5, 100, 71.1),
            ThemeColor::Font3 => hsla(0, 0, 100, 95),
            // 4) yellow / black / blue 
            ThemeColor::Background4 => hsl(69.9, 100, 88.8),
            ThemeColor::Background4Highlighted => hsl(69, 100, 87),
            ThemeColor::Font4 => hsla(0, 0, 0, 70),
            ThemeColor::Border4 => hsl(256.1, 87.8, 49.6),
            // 5) black / white
            ThemeColor::Background5 => hsla(0, 0, 20, 70),
            ThemeColor::Background5Highlighted => hsl(0, 0, 0),
            ThemeColor::Font5 => hsla(0, 0, 95, 95),
            // background of an invalid input
            ThemeColor::BackgroundInvalid => hsla(12.2, 100, 53.2, 40),
            // shadow
            ThemeColor::Shadow => hsla(0, 0, 2.7, 10),
            // dark shadow
            ThemeColor::Shadow2 => hsla(0, 0, 2.7, 30),
            // transparent
            ThemeColor::Transparent => hsla(0, 0, 0, 0),
        }
        .into_cow_str()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
