use crate::*;
use futures_signals::signal::{channel, Sender};

// ------ Radius ------

#[derive(Clone, Copy)]
enum Radius {
    Px(u32),
    Max,
}

impl Default for Radius {
    fn default() -> Self {
        Self::Px(0)
    }
}

// ------ RoundedCorners ------

#[derive(Default, Clone, Copy)]
pub struct RoundedCorners {
    top_left: Radius,
    top_right: Radius,
    bottom_left: Radius,
    bottom_right: Radius,
}

impl RoundedCorners {
    pub fn all(radius: u32) -> Self {
        Self::default()
            .top(radius)
            .bottom(radius)
    }

    pub fn all_max() -> Self {
        Self::default()
            .top_max()
            .bottom_max()
    }

    pub fn top(self, radius: u32) -> Self {
        self.top_left(radius).top_right(radius)
    }

    pub fn top_max(self) -> Self {
        self.top_left_max().top_right_max()
    }

    pub fn bottom(self, radius: u32) -> Self {
        self.bottom_left(radius).bottom_right(radius)
    }

    pub fn bottom_max(self) -> Self {
        self.bottom_left_max().bottom_right_max()
    }

    pub fn left(self, radius: u32) -> Self {
        self.top_left(radius).bottom_left(radius)
    }

    pub fn left_max(self) -> Self {
        self.top_left_max().bottom_left_max()
    }

    pub fn right(self, radius: u32) -> Self {
        self.top_right(radius).bottom_right(radius)
    }

    pub fn right_max(self) -> Self {
        self.top_right_max().bottom_right_max()
    }

    pub fn top_left(mut self, radius: u32) -> Self {
        self.top_left = Radius::Px(radius);
        self
    }

    pub fn top_left_max(mut self) -> Self {
        self.top_left = Radius::Max;
        self
    }

    pub fn top_right(mut self, radius: u32) -> Self {
        self.top_right = Radius::Px(radius);
        self
    }

    pub fn top_right_max(mut self) -> Self {
        self.top_right = Radius::Max;
        self
    }

    pub fn bottom_left(mut self, radius: u32) -> Self {
        self.bottom_left = Radius::Px(radius);
        self
    }

    pub fn bottom_left_max(mut self) -> Self {
        self.bottom_left = Radius::Max;
        self
    }

    pub fn bottom_right(mut self, radius: u32) -> Self {
        self.bottom_right = Radius::Px(radius);
        self
    }

    pub fn bottom_right_max(mut self) -> Self {
        self.bottom_right = Radius::Max;
        self
    }
}

impl<'a> Style<'a> for RoundedCorners {
    fn apply_to_raw_el<E: RawEl>(self, mut raw_el: E, style_group: Option<StyleGroup<'a>>) -> (E, Option<StyleGroup<'a>>) {
        let (size_sender, size_receiver) = channel((0, 0));
        
        raw_el = raw_el.on_resize(move |width, height| {
            size_sender.send((width, height)).unwrap_throw();
        });

        let border_radius_signal = size_receiver.map(move |(width, height)| {
            compute_radii(self, width, height)
        });

        if let Some(mut style_group) = style_group {
            style_group = style_group.style_signal("border-radius", border_radius_signal);
            return (raw_el, Some(style_group))
        }
        raw_el = raw_el.style_signal("border-radius", border_radius_signal);
        (raw_el, None)
    }
}

fn compute_radii(corners: RoundedCorners, width: u32, height: u32) -> String {
    crate::println!("xx");
    "0px 0px 0px 0px".to_owned()
}
