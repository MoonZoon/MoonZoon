use crate::*;
use futures_signals::signal::channel;
use std::array;

// ------ Radius ------

#[derive(Debug, Clone, Copy)]
enum Radius {
    Px(u32),
    Max,
}

impl Radius {
    fn f64_pixels_or_zero(self) -> f64 {
        if let Self::Px(radius) = self {
            f64::from(radius)
        } else {
            0.
        }
    }

    fn map_max_or_zero(self, f: impl FnOnce() -> f64) -> f64 {
        if matches!(self, Radius::Max) {
            f()
        } else {
            0.
        }
    }
}

impl Default for Radius {
    fn default() -> Self {
        Self::Px(0)
    }
}

// ------ RoundedCorners ------

#[derive(Debug, Default, Clone, Copy)]
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
            compute_radii(self, f64::from(width), f64::from(height))
        });

        if let Some(mut style_group) = style_group {
            style_group = style_group.style_signal("border-radius", border_radius_signal);
            return (raw_el, Some(style_group))
        }
        raw_el = raw_el.style_signal("border-radius", border_radius_signal);
        (raw_el, None)
    }
}

// https://css-tricks.com/what-happens-when-border-radii-overlap/
// https://www.w3.org/TR/css-backgrounds-3/#corner-overlap
fn compute_radii(corners: RoundedCorners, width: f64, height: f64) -> String {
    // @TODO cache some parts? If/when signal methods are implemented?
    // @TODO bug in ResizeObserver on iOS? (see "Add Client" button in time_tracker example)

    let mut radii = [
        corners.top_left.f64_pixels_or_zero(),
        corners.top_right.f64_pixels_or_zero(),
        corners.bottom_right.f64_pixels_or_zero(),
        corners.bottom_left.f64_pixels_or_zero(),
    ];
    let ratios = [
        // top side & adjacent radii
        width / (radii[0] + radii[1]),
        // right side & adjacent radii
        height / (radii[1] + radii[2]),
        // bottom side & adjacent radii
        width / (radii[2] + radii[3]),
        // left side & adjacent radii
        height / (radii[3] + radii[0]),
    ];
    let smallest_ratio = array::IntoIter::new(ratios).fold(f64::INFINITY, |a, b| a.min(b));
    if smallest_ratio < 1. {
        // fix overlapping radii, but keep ratios between radii
        radii = [
            radii[0] * smallest_ratio,
            radii[1] * smallest_ratio,
            radii[2] * smallest_ratio,
            radii[3] * smallest_ratio,
        ];
    }

    // @TODO do we want to keep ratios? ; the least surprising?

    // each value represents a max radius when treating other Radius::Max as zeros
    let mut max_radii = [
        corners.top_left.map_max_or_zero(|| {
            // left & top
            f64::min(height - radii[3], width - radii[1])
        }),
        corners.top_right.map_max_or_zero(|| {
            // top & right sides 
            f64::min(width - radii[0], height - radii[2])
        }),
        corners.bottom_right.map_max_or_zero(|| {
            // right & bottom sides 
            f64::min(height - radii[1], width - radii[3])
        }),
        corners.bottom_left.map_max_or_zero(|| {
            // bottom & left sides 
            f64::min(width - radii[2], height - radii[0])
        }),
    ];
    let max_ratios = [
        // top side & adjacent radii
        width / (max_radii[0] + max_radii[1]),
        // right side & adjacent radii
        height / (max_radii[1] + max_radii[2]),
        // bottom side & adjacent radii
        width / (max_radii[2] + max_radii[3]),
        // left side & adjacent radii
        height / (max_radii[3] + max_radii[0]),
    ];
    let max_smallest_ratio = array::IntoIter::new(max_ratios).fold(f64::INFINITY, |a, b| a.min(b));
    if max_smallest_ratio < 1. {
        // fix overlapping radii, but keep ratios between radii
        max_radii = [
            max_radii[0] * max_smallest_ratio,
            max_radii[1] * max_smallest_ratio,
            max_radii[2] * max_smallest_ratio,
            max_radii[3] * max_smallest_ratio,
        ];
    }
    
    for (index, max_radius) in array::IntoIter::new(max_radii).enumerate() {
        if max_radius != 0. {
            radii[index] = max_radius;
        }
    }
    crate::format!("{}px {}px {}px {}px", radii[0], radii[1], radii[2], radii[3])
}


//         .item(test_a())
//         .item(test_a2())
//         .item(test_b())
//         .item(test_c())
//         .item(test_d())
//         .item(test_e())
//         .item(test_f())
//         .item(test_g())
// }

// fn test_a() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(100))
//         .s(Height::new(100))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().top_left_max().right_max())
// }

// fn test_a2() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(100))
//         .s(Height::new(100))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().top_left_max().right_max().bottom_left(20))
// }

// fn test_b() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(200))
//         .s(Height::new(100))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().top_left_max().right_max())
// }

// fn test_c() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(200))
//         .s(Height::new(100))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().top_max())
// }

// fn test_d() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(100))
//         .s(Height::new(200))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().top_left_max().right_max())
// }

// fn test_e() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(100))
//         .s(Height::new(300))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().right_max())
// }

// fn test_f() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(100))
//         .s(Height::new(100))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(RoundedCorners::new().bottom_left_max().top_right_max())
// }

// fn test_g() -> impl Element {
//     El::new()
//         .s(Align::center())
//         .s(Width::new(200))
//         .s(Height::new(100))
//         .s(Background::new().color(hsl(256.1, 87.8, 49.6)))
//         .s(
//             RoundedCorners::new()
//                 .top_left_max()
//                 .top_right(10)
//                 .bottom_right_max()
//                 .bottom_left(10)
//         )
// }
