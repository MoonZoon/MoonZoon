use zoon::{eprintln, *};

pub static THEME_STORAGE_KEY: &str = "moonzoon-time_tracker-theme";

#[static_ref]
pub fn theme() -> &'static Mutable<Theme> {
    Mutable::new(Theme::Light)
}

pub fn load_theme() {
    if let Some(Ok(stored_theme)) = local_storage().get(THEME_STORAGE_KEY) {
        theme().set_neq(stored_theme);
    }
}

pub fn toggle_theme() {
    theme().update(|theme| match theme {
        Theme::Light => Theme::Dark,
        Theme::Dark => Theme::Light,
    });
    store_theme(theme().get())
}

fn store_theme(theme: Theme) {
    if let Err(error) = local_storage().insert(THEME_STORAGE_KEY, &theme) {
        eprintln!("Failed to store selected theme: {}", error);
    }
}

// ------ Theme ------

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(crate = "serde")]
pub enum Theme {
    Light,
    Dark,
}

// ------ colors ------

macro_rules! color {
    ($color:ident => $light_color:expr, $dark_color:expr) => {
        pub fn $color() -> impl Signal<Item = HSLuv> {
            theme().signal().map(|theme| match theme {
                Theme::Light => $light_color,
                Theme::Dark => $dark_color,
            })
        }
    }
}

// 0) white / black
color!(background_0 => hsluv!(0, 0, 100), hsluv!(0, 0, 0));
color!(font_0 => hsluv!(0, 0, 0, 70), hsluv!(0, 0, 100, 70));

// 1) blue / white 
color!(background_1 => hsluv!(256.1, 87.8, 49.6), hsluv!(256.1, 87.8, 20));
color!(background_1_highlighted => hsluv!(257, 92.3, 44.9), hsluv!(257, 92.3, 25));
color!(font_1 => hsluv!(0, 0, 100, 95), hsluv!(0, 0, 90, 95));
color!(border_1 => hsluv!(168.3, 100, 75.3), hsluv!(168.3, 100, 24.7));

// 2) light gray / black 
color!(background_2 => hsluv!(0, 0, 96.5), hsluv!(0, 0, 3.5));
color!(background_2_highlighted => hsluv!(0, 0, 94.5), hsluv!(0, 0, 6));
color!(font_2 => hsluv!(0, 0, 0, 70), hsluv!(0, 0, 100, 70));

// 3) green / white 
color!(background_3 => hsluv!(168.3, 100, 75.3), hsluv!(168.3, 100, 40));
color!(background_3_highlighted => hsluv!(168.5, 100, 71.1), hsluv!(168.5, 100, 45));
color!(font_3 => hsluv!(0, 0, 100, 95), hsluv!(0, 0, 90, 95));

// 4) yellow / black / blue 
color!(background_4 => hsluv!(69.9, 100, 88.8), hsluv!(69.9, 100, 30));
color!(background_4_highlighted => hsluv!(69, 100, 87), hsluv!(69, 100, 13));
color!(font_4 => hsluv!(0, 0, 0, 70), hsluv!(0, 0, 100, 70));
color!(border_4 => hsluv!(256.1, 87.8, 49.6), hsluv!(256.1, 87.8, 20));

// 5) black / white
color!(background_5 => hsluv!(0, 0, 20, 70), hsluv!(0, 0, 80, 70));
color!(background_5_highlighted => hsluv!(0, 0, 0), hsluv!(0, 0, 100));
color!(font_5 => hsluv!(0, 0, 95, 95), hsluv!(0, 0, 5, 95));

// background of an invalid input
color!(background_invalid => hsluv!(12.2, 100, 53.2, 40), hsluv!(12.2, 100, 46.8, 40));

// shadow
color!(shadow => hsluv!(0, 0, 2.7, 10), hsluv!(0, 0, 0, 30));

// dark shadow
color!(shadow_2 => hsluv!(0, 0, 2.7, 30), hsluv!(0, 0, 0, 50));

// transparent
pub fn transparent() -> impl Signal<Item = HSLuv> {
    always(hsluv!(0, 0, 0, 0))
}
