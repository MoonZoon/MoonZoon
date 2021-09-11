use zoon::*;
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub enum Theme {
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
    Shadow,
    Transparent,
}

impl Color<'_> for Theme {}

impl<'a> IntoCowStr<'a> for Theme {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            // 0) white / black
            Theme::Background0 => hsl(0, 0, 100),
            Theme::Font0 => hsla(0, 0, 0, 70),
            // 1) blue / white 
            Theme::Background1 => hsl(256.1, 87.8, 49.6),
            Theme::Background1Highlighted => hsl(257, 92.3, 44.9),
            Theme::Font1 => hsla(0, 0, 100, 95),
            Theme::Border1 => hsl(168.3, 100, 75.3),
            // 2) light gray / black 
            Theme::Background2 => hsl(0, 0, 96.5),
            Theme::Background2Highlighted => hsl(0, 0, 94.5),
            Theme::Font2 => hsla(0, 0, 0, 70),
            // 3) green / white 
            Theme::Background3 => hsl(168.3, 100, 75.3),
            Theme::Background3Highlighted => hsl(168.5, 100, 71.1),
            Theme::Font3 => hsla(0, 0, 100, 95),
            // 4) yellow / black / blue 
            Theme::Background4 => hsl(69.9, 100, 88.8),
            Theme::Background4Highlighted => hsl(69, 100, 87),
            Theme::Font4 => hsla(0, 0, 0, 70),
            Theme::Border4 => hsl(256.1, 87.8, 49.6),
            // shadow
            Theme::Shadow => hsla(0, 0, 2.7, 10),
            // transparent
            Theme::Transparent => hsla(0, 0, 0, 0),
        }
        .into_cow_str()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
