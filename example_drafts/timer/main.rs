use zoon::*;
use std::ops::Not;

zoons!{

    // -- stopwatch --

    #[var]
    fn seconds() -> u32 {
        0
    }

    #[var]
    fn stopwatch() -> Option<Timer> {
        None
    }

    #[update]
    fn start_stopwatch() {
        stopwatch().set(Some(Timer::new(1_000, increment_seconds)));
    }

    #[update]
    fn stop_stopwatch() {
        stopwatch().set(None);
    }

    #[update]
    fn increment_seconds() {
        seconds().update(|seconds| seconds + 1);
    }

    // -- timeout --

    #[var]
    fn timeout() -> Option<Timer> {
        None
    }

    #[update]
    fn start_timeout() {
        timeout().set(Some(Timer::new(2_000, stop_timeout)));
    }

    #[update]
    fn stop_timeout() {
        timeout().set(None);
    }

    // -- view --

    #[el]
    fn root() -> Column {
        column![
            spacing(30),
            stopwatch_panel(),
            timeout_panel(),
        ]
    }

    #[el]
    fn stopwatch_panel() -> Row {
        let seconds = seconds().inner();
        let enabled = stopwatch().map(Option::is_some);

        row![
            spacing(10),
            format!("Seconds: {}", seconds),
            button![
                background::color(if enabled {
                    color::gray(),
                } else {
                    color::green().set_l(66),
                }),
                enabled.not().then(|| button::on_press(start_stopwatch)),
                "Start",
            ],
            button![
                background::color(if enabled {
                    color::red().set_l(66),
                } else {
                    color::gray(),
                }),
                enabled.then(|| button::on_press(stop_stopwatch)),
                "Stop",
            ],
        ]
    }

    #[el]
    fn timeout_panel() -> Row {
        let enabled = timeout().map(Option::is_some);

        row![
            spacing(10),
            button![
                background::color(if enabled {
                    color::gray(),
                } else {
                    color::green().set_l(66),
                }),
                enabled.not().then(|| button::on_press(start_timeout)),
                "Start 2s timeout",
            ],
            button![
                background::color(if enabled {
                    color::red().set_l(66),
                } else {
                    color::gray(),
                }),
                enabled.then(|| button::on_press(stop_timeout)),
                "Stop",
            ],
        ]
    }

}

fn main() {
    start!(zoons)
}
