use zoon::*;

static HUE: Lazy<Tweened> = Lazy::new(|| Tweened::new(0, Duration::seconds(3), ease::linear));

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    let speed = Duration::seconds(1);
    let ease = ease::cubic_out;

    let (radius, radius_signal) = Tweened::new_and_signal(20, speed, ease);
    let (cx, cx_signal) = Tweened::new_and_signal(50, speed, ease);
    let (cy, cy_signal) = Tweened::new_and_signal(50, speed, ease);

    Task::start(async move {
        loop {
            HUE.go_to(360);

            radius.go_to(100);
            cx.go_to(1000);
            cy.go_to(600);

            Timer::sleep(1500).await;
            radius.go_to(50);
            cx.go_to(700);
            cy.go_to(200);

            Timer::sleep(1500).await;
            radius.go_to(25);
            cx.go_to(500);
            cy.go_to(500);

            HUE.go_to(0);

            Timer::sleep(1500).await;
            radius.go_to(35);
            cx.go_to(40);
            cy.go_to(50);

            Timer::sleep(1500).await;
        }
    });

    RawSvgEl::new("svg")
        .attr("width", "100%")
        .attr("height", "100%")
        .attr("viewbox", "0 0 300 300")
        .child(
            RawSvgEl::new("circle")
                .attr_signal("cx", cx_signal)
                .attr_signal("cy", cy_signal)
                .attr_signal(
                    "fill",
                    HUE.signal()
                        .map(|hue| oklch().l(0.5).c(0.4).h(hue).a(1).to_css_string()),
                )
                .attr_signal("r", radius_signal),
        )
}
