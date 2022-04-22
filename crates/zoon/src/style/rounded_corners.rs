use crate::*;
use futures_signals::signal::channel;

// ------ Radius ------
/// Define radius for rounded corners with pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Radius {
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

impl From<u32> for Radius {
    fn from(radius: u32) -> Self {
        Self::Px(radius)
    }
}

// ------ IntoOptionRadius ------

pub trait IntoOptionRadius {
    fn into_option_radius(self) -> Option<Radius>;
}

impl<T: Into<Radius>> IntoOptionRadius for T {
    fn into_option_radius(self) -> Option<Radius> {
        Some(self.into())
    }
}

impl<T: Into<Radius>> IntoOptionRadius for Option<T> {
    fn into_option_radius(self) -> Option<Radius> {
        self.map(Into::into)
    }
}

// ------ RadiusSignal ------

struct RadiusSignal(Box<dyn Signal<Item = Option<Radius>> + 'static + Unpin>);

impl RadiusSignal {
    fn new_from_value(radius: impl Into<Radius>) -> Self {
        Self(Box::new(always(Some(radius.into()))))
    }

    fn new_from_signal(
        radius: impl Signal<Item = impl IntoOptionRadius> + 'static + Unpin,
    ) -> Self {
        Self(Box::new(radius.map(|radius| radius.into_option_radius())))
    }
}

impl Default for RadiusSignal {
    fn default() -> Self {
        Self::new_from_value(Radius::default())
    }
}

// ------ RoundedCorners ------
/// Define rounded corners. It does translate to css `border-radius` for web.
/// More information at <https://developer.mozilla.org/en-US/docs/Web/CSS/border-radius>.
#[derive(Default)]
pub struct RoundedCorners {
    top_left: RadiusSignal,
    top_right: RadiusSignal,
    bottom_left: RadiusSignal,
    bottom_right: RadiusSignal,
}

impl RoundedCorners {
    /// Set radius for top and bottom corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(RoundedCorners::all(10)).label("Click me");
    /// ```
    pub fn all(radius: u32) -> Self {
        Self::default().top(radius).bottom(radius)
    }

    pub fn all_signal(all: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static) -> Self {
        let this = Self::default();
        let all = Broadcaster::new(all.map(|radius| radius.into_option_radius()));
        this.top_signal(all.signal()).bottom_signal(all.signal())
    }

    /// Set radius to maximum value for top and bottom corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new().s(RoundedCorners::all_max()).label("Click me");
    /// ```
    pub fn all_max() -> Self {
        Self::default().top_max().bottom_max()
    }

    /// Set radius for top corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().top(10))
    ///     .label("Click me");
    /// ```
    pub fn top(self, radius: impl Into<Radius>) -> Self {
        let radius = radius.into();
        self.top_left(radius).top_right(radius)
    }

    pub fn top_signal(
        self,
        top: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        let top = Broadcaster::new(top.map(|radius| radius.into_option_radius()));
        self.top_left_signal(top.signal())
            .top_right_signal(top.signal())
    }

    /// Set radius to maximum for top corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().top_max())
    ///     .label("Click me");
    /// ```
    pub fn top_max(self) -> Self {
        self.top_left_max().top_right_max()
    }

    /// Set radius for bottom corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().bottom(10))
    ///     .label("Click me");
    /// ```
    pub fn bottom(self, radius: impl Into<Radius>) -> Self {
        let radius = radius.into();
        self.bottom_left(radius).bottom_right(radius)
    }

    pub fn bottom_signal(
        self,
        bottom: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        let bottom = Broadcaster::new(bottom.map(|radius| radius.into_option_radius()));
        self.bottom_left_signal(bottom.signal())
            .bottom_right_signal(bottom.signal())
    }

    /// Set radius to maximum for bottom corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().bottom_max())
    ///     .label("Click me");
    /// ```
    pub fn bottom_max(self) -> Self {
        self.bottom_left_max().bottom_right_max()
    }

    /// Set radius for left corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().left(10))
    ///     .label("Click me");
    /// ```
    pub fn left(self, radius: impl Into<Radius>) -> Self {
        let radius = radius.into();
        self.top_left(radius).bottom_left(radius)
    }

    pub fn left_signal(
        self,
        left: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        let left = Broadcaster::new(left.map(|radius| radius.into_option_radius()));
        self.top_left_signal(left.signal())
            .bottom_left_signal(left.signal())
    }

    /// Set radius to maximum for left corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().left_max())
    ///     .label("Click me");
    /// ```
    pub fn left_max(self) -> Self {
        self.top_left_max().bottom_left_max()
    }

    /// Set radius for right corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().right(10))
    ///     .label("Click me");
    /// ```
    pub fn right(self, radius: impl Into<Radius>) -> Self {
        let radius = radius.into();
        self.top_right(radius).bottom_right(radius)
    }

    pub fn right_signal(
        self,
        right: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        let right = Broadcaster::new(right.map(|radius| radius.into_option_radius()));
        self.top_right_signal(right.signal())
            .bottom_right_signal(right.signal())
    }

    /// Set radius to maximum for right corners.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().right_max())
    ///     .label("Click me");
    /// ```
    pub fn right_max(self) -> Self {
        self.top_right_max().bottom_right_max()
    }

    pub fn top_left_signal(
        mut self,
        radius: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        self.top_left = RadiusSignal::new_from_signal(radius);
        self
    }
    /// Set radius for the top left corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().top_left(10))
    ///     .label("Click me");
    /// ```
    pub fn top_left(mut self, radius: impl Into<Radius>) -> Self {
        self.top_left = RadiusSignal::new_from_value(radius);
        self
    }

    /// Set radius to maximum for the top left corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().top_left_max())
    ///     .label("Click me");
    /// ```
    pub fn top_left_max(mut self) -> Self {
        self.top_left = RadiusSignal::new_from_value(Radius::Max);
        self
    }

    pub fn top_right_signal(
        mut self,
        radius: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        self.top_right = RadiusSignal::new_from_signal(radius);
        self
    }

    /// Set radius for the top right corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().top_right(10))
    ///     .label("Click me");
    /// ```
    pub fn top_right(mut self, radius: impl Into<Radius>) -> Self {
        self.top_right = RadiusSignal::new_from_value(radius);
        self
    }

    /// Set radius to maximum for the top right corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().top_right_max())
    ///     .label("Click me");
    /// ```
    pub fn top_right_max(mut self) -> Self {
        self.top_right = RadiusSignal::new_from_value(Radius::Max);
        self
    }

    pub fn bottom_left_signal(
        mut self,
        radius: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        self.bottom_left = RadiusSignal::new_from_signal(radius);
        self
    }

    /// Set radius for the bottom left corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().bottom_left(10))
    ///     .label("Click me");
    /// ```
    pub fn bottom_left(mut self, radius: impl Into<Radius>) -> Self {
        self.bottom_left = RadiusSignal::new_from_value(radius);
        self
    }

    /// Set radius to maximum for the bottom left corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().bottom_left_max())
    ///     .label("Click me");
    /// ```
    pub fn bottom_left_max(mut self) -> Self {
        self.bottom_left = RadiusSignal::new_from_value(Radius::Max);
        self
    }

    pub fn bottom_right_signal(
        mut self,
        radius: impl Signal<Item = impl IntoOptionRadius> + Unpin + 'static,
    ) -> Self {
        self.bottom_right = RadiusSignal::new_from_signal(radius);
        self
    }

    /// Set radius for the bottom right corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().bottom_right(10))
    ///     .label("Click me");
    /// ```
    pub fn bottom_right(mut self, radius: impl Into<Radius>) -> Self {
        self.bottom_right = RadiusSignal::new_from_value(radius);
        self
    }

    /// Set radius to maximum for the bottom right corner.
    /// # Example
    /// ```no_run
    /// use zoon::*;
    ///
    /// let button = Button::new()
    ///     .s(RoundedCorners::new().bottom_right_max())
    ///     .label("Click me");
    /// ```
    pub fn bottom_right_max(mut self) -> Self {
        self.bottom_right = RadiusSignal::new_from_value(Radius::Max);
        self
    }
}

impl<'a> Style<'a> for RoundedCorners {
    fn merge_with_group(self, group: StyleGroup<'a>) -> StyleGroup<'a> {
        let (size_sender, size_receiver) = channel((0, 0));

        let border_radius_signal = map_ref! {
            let top_left = self.top_left.0,
            let top_right = self.top_right.0,
            let bottom_left = self.bottom_left.0,
            let bottom_right = self.bottom_right.0,
            let (width, height) = size_receiver =>
            compute_radii(
                top_left.unwrap_or_default(),
                top_right.unwrap_or_default(),
                bottom_left.unwrap_or_default(),
                bottom_right.unwrap_or_default(),
                *width,
                *height,
            )
        };
        group
            .on_resize(move |width, height| {
                size_sender.send((width, height)).unwrap_throw();
            })
            .style_signal("border-radius", border_radius_signal)
    }
}

// https://css-tricks.com/what-happens-when-border-radii-overlap/
// https://www.w3.org/TR/css-backgrounds-3/#corner-overlap
fn compute_radii(
    top_left: Radius,
    top_right: Radius,
    bottom_left: Radius,
    bottom_right: Radius,
    width: u32,
    height: u32,
) -> String {
    let width = f64::from(width);
    let height = f64::from(height);

    // @TODO cache some parts? If/when signal methods are implemented?
    // @TODO bug in ResizeObserver on iOS? (see "Add Client" button in time_tracker
    // example)

    let mut radii = [
        top_left.f64_pixels_or_zero(),
        top_right.f64_pixels_or_zero(),
        bottom_right.f64_pixels_or_zero(),
        bottom_left.f64_pixels_or_zero(),
    ];

    // It doesn't make sense to compute radii if the element is basically invisible.
    // Or the element's width and height is 0px when the element's `display` is
    // `inline` because `ResizeObserver` doesn't work with inlined elements.
    // Hence want to preserve at least fixed radii.
    if width == 0. || height == 0. {
        return crate::format!(
            "{}px {}px {}px {}px",
            radii[0],
            radii[1],
            radii[2],
            radii[3]
        );
    }

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
    let smallest_ratio = ratios.into_iter().fold(f64::INFINITY, |a, b| a.min(b));
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
        top_left.map_max_or_zero(|| {
            // left & top
            f64::min(height - radii[3], width - radii[1])
        }),
        top_right.map_max_or_zero(|| {
            // top & right sides
            f64::min(width - radii[0], height - radii[2])
        }),
        bottom_right.map_max_or_zero(|| {
            // right & bottom sides
            f64::min(height - radii[1], width - radii[3])
        }),
        bottom_left.map_max_or_zero(|| {
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
    let max_smallest_ratio = max_ratios.into_iter().fold(f64::INFINITY, |a, b| a.min(b));
    if max_smallest_ratio < 1. {
        // fix overlapping radii, but keep ratios between radii
        max_radii = [
            max_radii[0] * max_smallest_ratio,
            max_radii[1] * max_smallest_ratio,
            max_radii[2] * max_smallest_ratio,
            max_radii[3] * max_smallest_ratio,
        ];
    }

    for (index, max_radius) in max_radii.into_iter().enumerate() {
        if max_radius != 0. {
            radii[index] = max_radius;
        }
    }
    crate::format!(
        "{}px {}px {}px {}px",
        radii[0],
        radii[1],
        radii[2],
        radii[3]
    )
}

// @TODO remove or integrate to an existing example (e.g. slider) or to a new
// one?

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
