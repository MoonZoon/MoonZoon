
pub trait Style {

}

pub struct Font;

impl Font {
    pub fn new() -> Self {
        Self
    }

    pub fn bold(self) -> Self {
        self
    }
}

impl Style for Font {
    
}
