use crate::*;

pub struct Background {
    color_signal: Option<Box<dyn Signal<Item = Box<dyn IntoOptionCowStr<'static>>> + Unpin>>,
}

impl Background {
    pub fn new() -> Self {
        Self {
            color_signal: None
        }
    }

    pub fn color_signal(mut self, color: impl Signal<Item = impl Color<'static> + 'static> + Unpin + 'static) -> Self {
        self.color_signal = Some(Box::new(color.map(|color| {
            Box::new(color) as Box<dyn IntoOptionCowStr<'static>>
        })));
        self
    }
}

impl Style for Background {
    fn update_raw_el_style<T: RawEl>(self, mut raw_el: T) -> T {
        if let Some(color_signal) = self.color_signal {
            raw_el = raw_el.style_signal("background-color", color_signal);
        }
        raw_el
    }
}
