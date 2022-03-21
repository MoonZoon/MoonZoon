use zoon::{named_color::*, println, *, strum::{EnumIter, IntoEnumIterator}};

// ------ ------
//     Types
// ------ ------

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter)]
#[strum(crate = "strum")]
enum Rectangle {
    A,
    B,
    C,
}

impl Rectangle {
    fn color_and_align(&self) -> (HSLuv, Align) {
        match self {
            Self::A => (RED_6, Align::new()),
            Self::B => (GREEN_6, Align::center()),
            Self::C => (BLUE_6, Align::new().bottom().right()),
        }
    }
}

// ------ ------
//    States
// ------ ------

#[static_ref]
fn rectangles() -> &'static MutableVec<Rectangle> {
    MutableVec::new_with_values(Rectangle::iter().collect())
}

#[static_ref]
fn breathing() -> &'static Movement {
    WaveOscillator::new(100, 120).cycle(Duration::seconds(2))
}

pub struct Movement {
    repeat: Option<usize>,
    duration: Duration,
    oscillator: Box<dyn Oscillator + Sync + Send>,
    value: Mutable<f64>
}

impl Movement {
    pub fn new(
        repeat: Option<usize>, 
        duration: Duration, 
        oscillator: impl Oscillator + Sync + Send + 'static,
    ) -> Self {
        Self {
            repeat,
            duration,
            oscillator: Box::new(oscillator),
            value: Mutable::default(),
        }
    }

    pub fn signal(&self) -> MutableSignal<f64> {
        self.value.signal()
    }
}

pub trait Oscillator {
    fn interpolate(&self, input: f64) -> f64;
}

pub trait OscillatorExt: Oscillator where Self: Sized + Send + Sync + 'static {
    fn shift(self, shift: f64) -> Self;

    fn cycle(self, duration: Duration) -> Movement {
        Movement::new(None, duration, self)
    }
    fn repeat(self, n: usize, duration: Duration) -> Movement {
        Movement::new(Some(n), duration, self)
    }
    fn once(self, duration: Duration) -> Movement {
        self.repeat(1, duration)
    }
}

pub struct WaveOscillator {
    min: f64,
    max: f64,
    shift: f64,
}

impl WaveOscillator {
    pub fn new(min: impl Into<f64>, max: impl Into<f64>) -> Self {
        Self {
            min: min.into(),
            max: max.into(),
            shift: 0.,
        }
    }
}

impl Oscillator for WaveOscillator {
    fn interpolate(&self, input: f64) -> f64 {
        todo!()
    }
}

impl OscillatorExt for WaveOscillator {
    fn shift(mut self, shift: f64) -> Self {
        self.shift = shift;
        self
    }
}

pub struct WrapOscillator {
    start: f64,
    end: f64,
    shift: f64,
}

impl WrapOscillator {
    pub fn new(start: impl Into<f64>, end: impl Into<f64>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            shift: 0.,
        }
    }
}

impl Oscillator for WrapOscillator {
    fn interpolate(&self, input: f64) -> f64 {
        todo!()
    }
}

impl OscillatorExt for WrapOscillator {
    fn shift(mut self, shift: f64) -> Self {
        self.shift = shift;
        self
    }
}

pub struct ZigZagOscillator {
    start: f64,
    end: f64,
    shift: f64,
}

impl ZigZagOscillator {
    pub fn new(start: impl Into<f64>, end: impl Into<f64>) -> Self {
        Self {
            start: start.into(),
            end: end.into(),
            shift: 0.,
        }
    }
}

impl Oscillator for ZigZagOscillator {
    fn interpolate(&self, input: f64) -> f64 {
        todo!()
    }
}

impl OscillatorExt for ZigZagOscillator {
    fn shift(mut self, shift: f64) -> Self {
        self.shift = shift;
        self
    }
}

pub struct CustomOscillator {
    iterpolation: Box<dyn Fn(f64) -> f64 + Send + Sync>,
    shift: f64,
}

impl CustomOscillator {
    pub fn new(iterpolation: impl Fn(f64) -> f64 + 'static + Send + Sync) -> Self {
        Self {
            iterpolation: Box::new(iterpolation),
            shift: 0.,
        }
    }
}

impl Oscillator for CustomOscillator {
    fn interpolate(&self, input: f64) -> f64 {
        todo!()
    }
}

impl OscillatorExt for CustomOscillator {
    fn shift(mut self, shift: f64) -> Self {
        self.shift = shift;
        self
    }
}

// ------ ------
//   Commands
// ------ ------

fn bring_to_front(rectangle: Rectangle) {
    let mut rectangles = rectangles().lock_mut();
    let position = rectangles
        .iter()
        .position(|r| r == &rectangle)
        .unwrap_throw();
    rectangles.move_from_to(position, rectangles.len() - 1);
}

// ------ ------
//     View
// ------ ------

fn root() -> impl Element {
    Stack::new()
        .s(Align::center())
        .s(Width::new(180))
        .s(Height::new(180))
        .layers_signal_vec(rectangles().signal_vec().map(rectangle))
}

fn rectangle(rectangle: Rectangle) -> impl Element {
    println!("render Rectangle '{rectangle:?}'");

    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let (color, align) = rectangle.color_and_align();

    El::new()
        .s(Transform::with_signal(breathing().signal().map(|percent| {
            Transform::new().scale(percent)
        })))
        .s(Width::new(100))
        .s(Height::new(100))
        .s(RoundedCorners::all(15))
        .s(Cursor::new(CursorIcon::Pointer))
        .s(Shadows::new([Shadow::new().blur(20).color(GRAY_8)]))
        .s(Background::new().color_signal(
            hovered_signal.map_bool(move || color.update_l(|l| l + 5.), move || color),
        ))
        .s(align)
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || bring_to_front(rectangle))
}

// ------ ------
//     Start
// ------ ------

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
