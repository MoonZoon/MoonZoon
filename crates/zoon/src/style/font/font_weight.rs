// ------ FontWeight ------

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub enum FontWeight {
    ExtraHeavy,
    Heavy,
    ExtraBold,
    Bold,
    SemiBold,
    Medium,
    Regular,
    Light,
    ExtraLight,
    Hairline,
    Number(u32),
}

impl FontWeight {
    pub fn number(&self) -> u32 {
        match self {
            Self::ExtraHeavy => 1000,
            Self::Heavy => 900,
            Self::ExtraBold => 800,
            Self::Bold => 700,
            Self::SemiBold => 600,
            Self::Medium => 500,
            Self::Regular => 400,
            Self::Light => 300,
            Self::ExtraLight => 200,
            Self::Hairline => 100,
            Self::Number(number) => *number,
        }
    }
}
