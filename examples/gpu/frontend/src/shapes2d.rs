use three_d::*;

#[allow(dead_code)]
pub fn run(canvas: zoon::web_sys::HtmlCanvasElement) {
    let window = Window::new(WindowSettings {
        title: "Shapes 2D!".to_string(),
        max_size: Some((super::CANVAS_WIDTH, super::CANVAS_HEIGHT)),
        // @TODO Set RustAnalyzer target for this crate to `wasm32-unknown-unknown` once possible
        // https://github.com/rust-lang/rust-analyzer/issues/11900
        canvas: Some(canvas),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();
    let scale_factor = window.device_pixel_ratio();
    let (width, height) = window.size();

    let mut rectangle = Gm::new(
        Rectangle::new(
            &context,
            vec2(200.0, 200.0) * scale_factor,
            degrees(45.0),
            100.0 * scale_factor,
            200.0 * scale_factor,
        ),
        ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
    );
    let mut circle = Gm::new(
        Circle::new(
            &context,
            vec2(500.0, 500.0) * scale_factor,
            200.0 * scale_factor,
        ),
        ColorMaterial {
            color: Srgba::BLUE,
            ..Default::default()
        },
    );
    let mut line = Gm::new(
        Line::new(
            &context,
            vec2(0.0, 0.0) * scale_factor,
            vec2(width as f32, height as f32) * scale_factor,
            5.0 * scale_factor,
        ),
        ColorMaterial {
            color: Srgba::GREEN,
            ..Default::default()
        },
    );

    window.render_loop(move |frame_input| {
        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button,
                position,
                modifiers,
                ..
            } = *event
            {
                if button == MouseButton::Left && !modifiers.ctrl {
                    rectangle.set_center(position);
                }
                if button == MouseButton::Right && !modifiers.ctrl {
                    circle.set_center(position);
                }
                if button == MouseButton::Left && modifiers.ctrl {
                    let ep = line.end_point1();
                    line.set_endpoints(position, ep);
                }
                if button == MouseButton::Right && modifiers.ctrl {
                    let ep = line.end_point0();
                    line.set_endpoints(ep, position);
                }
            }
        }
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &Camera::new_2d(frame_input.viewport),
                line.into_iter().chain(&rectangle).chain(&circle),
                &[],
            );

        FrameOutput::default()
    });
}
