use zoon::*;

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

// @TODO macro? impl Color?

// 0) white / black
pub fn background_0() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(0, 0, 100),
        Theme::Dark => hsl(0, 0, 0),
    })
}
pub fn font_0() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 0, 70),
        Theme::Dark => hsla(0, 0, 0, 70),
    })
}
// 1) blue / white 
pub fn background_1() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(256.1, 87.8, 49.6),
        Theme::Dark => hsl(256.1, 87.8, 49.6),
    })
}
pub fn background_1_highlighted() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(257, 92.3, 44.9),
        Theme::Dark => hsl(257, 92.3, 44.9),
    })
}
pub fn font_1() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 100, 95),
        Theme::Dark => hsla(0, 0, 100, 95),
    })
}
pub fn border_1() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(168.3, 100, 75.3),
        Theme::Dark => hsl(168.3, 100, 75.3),
    })
}
// 2) light gray / black 
pub fn background_2() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(0, 0, 96.5),
        Theme::Dark => hsl(0, 0, 96.5),
    })
}
pub fn background_2_highlighted() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(0, 0, 94.5),
        Theme::Dark => hsl(0, 0, 94.5),
    })
}
pub fn font_2() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 0, 70),
        Theme::Dark => hsla(0, 0, 0, 70),
    })
}
// 3) green / white 
pub fn background_3() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(168.3, 100, 75.3),
        Theme::Dark => hsl(168.3, 100, 75.3),
    })
}
pub fn background_3_highlighted() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(168.5, 100, 71.1),
        Theme::Dark => hsl(168.5, 100, 71.1),
    })
}
pub fn font_3() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 100, 95),
        Theme::Dark => hsla(0, 0, 100, 95),
    })
}
// 4) yellow / black / blue 
pub fn background_4() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(69.9, 100, 88.8),
        Theme::Dark => hsl(69.9, 100, 88.8),
    })
}
pub fn background_4_highlighted() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(69, 100, 87),
        Theme::Dark => hsl(69, 100, 87),
    })
}
pub fn font_4() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 0, 70),
        Theme::Dark => hsl(0, 0, 0),
    })
}
pub fn border_4() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(256.1, 87.8, 49.6),
        Theme::Dark => hsl(256.1, 87.8, 49.6),
    })
}
// 5) black / white
pub fn background_5() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 20, 70),
        Theme::Dark => hsla(0, 0, 20, 70),
    })
}
pub fn background_5_highlighted() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsl(0, 0, 0),
        Theme::Dark => hsl(0, 0, 0),
    })
}
pub fn font_5() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 95, 95),
        Theme::Dark => hsla(0, 0, 95, 95),
    })
}
// background of an invalid input
pub fn background_invalid() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(12.2, 100, 53.2, 40),
        Theme::Dark => hsla(12.2, 100, 53.2, 40),
    })
}
// shadow
pub fn shadow() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 2.7, 10),
        Theme::Dark => hsla(0, 0, 2.7, 10),
    })
}
// dark shadow
pub fn shadow_2() -> impl Signal<Item = HSLuv> {
    theme().signal().map(|theme| match theme {
        Theme::Light => hsla(0, 0, 2.7, 30),
        Theme::Dark => hsla(0, 0, 2.7, 30),
    })
}
// transparent
pub fn transparent() -> impl Signal<Item = HSLuv> {
    always(hsla(0, 0, 0, 0))
}
