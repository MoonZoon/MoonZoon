use zoon::*;
use std::borrow::Cow;

#[derive(Clone, Copy)]
pub enum Theme {
    Background1,
    Background1Highlighted,
    Font1,
    Background2,
    Background2Highlighted,
    Font2,
    Background3,
    Background3Highlighted,
    Font3,
    Transparent,
}

impl Color<'_> for Theme {}

impl<'a> IntoCowStr<'a> for Theme {
    fn into_cow_str(self) -> Cow<'a, str> {
        match self {
            // 1) blue / white 
            Theme::Background1 => hsl(256.1, 87.8, 49.6),
            Theme::Background1Highlighted => hsl(257, 92.3, 44.9),
            Theme::Font1 => hsla(0, 0, 100, 95),
            // 2) white / black 
            Theme::Background2 => hsl(0, 0, 96.5),
            Theme::Background2Highlighted => hsl(0, 0, 94.5),
            Theme::Font2 => hsla(0, 0, 0, 70),
            // 3) green / white 
            Theme::Background3 => hsl(168.3, 100, 75.3),
            Theme::Background3Highlighted => hsl(168.5, 100, 71.1),
            Theme::Font3 => hsla(0, 0, 100, 95),
            // transparent
            Theme::Transparent => hsla(0, 0, 0, 0),
        }
        .into_cow_str()
    }

    fn take_into_cow_str(&mut self) -> Cow<'a, str> {
        self.into_cow_str()
    }
}
