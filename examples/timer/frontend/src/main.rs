use zoon::*;

// -- Stopwatch --

static STOPWATCH: Lazy<Stopwatch> = lazy::default();

#[derive(Default)]
struct Stopwatch {
    timer: Mutable<Option<Timer>>,
    seconds: Mutable<u32>,
}
impl Stopwatch {
    fn start(&self) {
        self.seconds.set(0);
        let seconds = self.seconds.clone();
        self.timer
            .set(Some(Timer::new(1_000, move || *seconds.lock_mut() += 1)));
    }
    fn stop(&self) {
        self.timer.set(None);
    }
    fn enabled(&self) -> impl Signal<Item = bool> {
        self.timer.signal_ref(Option::is_some)
    }
    fn seconds(&self) -> impl Signal<Item = u32> {
        self.seconds.signal()
    }
}

// -- Timeout --

static TIMEOUT: Lazy<Timeout> = lazy::default();

#[derive(Default)]
struct Timeout {
    timer: Mutable<Option<Timer>>,
}
impl Timeout {
    fn enabled(&self) -> impl Signal<Item = bool> {
        self.timer.signal_ref(Option::is_some)
    }
    fn start(&self) {
        let timer = self.timer.clone();
        self.timer
            .set(Some(Timer::once(2_000, move || timer.set(None))));
    }
    fn stop(&self) {
        self.timer.set(None);
    }
}

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Column::new()
        .s(Align::center())
        .s(Gap::both(30))
        .item(stopwatch_panel())
        .item(timeout_panel())
        .item(sleep_panel())
}

fn stopwatch_panel() -> impl Element {
    Row::new()
        .s(Gap::both(20))
        .item("Seconds: ")
        .item_signal(STOPWATCH.seconds())
        .item_signal(STOPWATCH.enabled().map_bool(
            || stop_button(|| STOPWATCH.stop()).left_either(),
            || start_button(|| STOPWATCH.start()).right_either(),
        ))
}

fn timeout_panel() -> impl Element {
    Row::new()
        .s(Gap::both(20))
        .item("2s Timeout")
        .item_signal(TIMEOUT.enabled().map_bool(
            || stop_button(|| TIMEOUT.stop()).left_either(),
            || start_button(|| TIMEOUT.start()).right_either(),
        ))
}

fn sleep_panel() -> impl Element {
    let (asleep, asleep_signal) = Mutable::new_and_signal(false);
    let sleep = move || {
        Task::start(clone!((asleep) async move {
            asleep.set_neq(true);
            Timer::sleep(2_000).await;
            asleep.set_neq(false);
        }))
    };
    Row::new()
        .s(Gap::both(20))
        .item("2s Async Sleep")
        .item_signal(asleep_signal.map_bool(
            || El::new().child("zZZ...").left_either(),
            move || start_button(sleep.clone()).right_either(),
        ))
}

fn start_button(on_press: impl FnMut() + 'static) -> impl Element {
    button("Start", color!("green"), color!("darkgreen"), on_press)
}

fn stop_button(on_press: impl FnMut() + 'static) -> impl Element {
    button("Stop", color!("red"), color!("darkred"), on_press)
}

fn button(
    label: &str,
    bg_color_hovered: Rgba,
    bg_color: Rgba,
    on_press: impl FnMut() + 'static,
) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::new().x(12).y(6))
        .s(RoundedCorners::all(5))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(move || bg_color_hovered, move || bg_color)))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(on_press)
}
