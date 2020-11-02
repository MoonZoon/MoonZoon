use zoon::*;

zoons!{

    #[model]
    fn watching_enabled() -> bool {
        false
    }

    #[update]
    fn toggle_watching() {
        watching_enabled().update(|enabled| *enabled = !enabled);
    }

    #[model]
    fn mouse_position() -> mouse::Position {
        mouse::Position::default()
    }

    #[update]
    fn update_mouse_position(mouse: Mouse) {
        mouse_position().set(mouse.position);
    }

    #[model]
    fn last_key() -> String {
        String::new()
    }

    #[update]
    fn update_last_key(key: keyboard::Key) {
        last_key().set(key.id.to_string());
    }

    #[view]
    fn view() -> View {
        let enabled = watching_enabled().inner();

        view![
            enabled.map_true(|| vec![
                on_mouse_move(update_mouse_position),
                on_key_down(update_last_key),
            ]),
            control_panel(),
        ]
    }

    #[view]
    fn control_panel() -> View {
        let position = mouse_position().inner();
        let key = last_key();
        let enabled = watching_enabled().inner();

        column![
            format!("X: {}, Y: {}", position.x, position.y),
            key.map(|key| format!("Key: {}", key)),
            button![
                button::on_press(toggle_watching),
                format!("{} watching", if enabled { "Stop" } else { "Start"}),
            ],
        ]
    }
}

fn main() {
    start!(zoons)
}
