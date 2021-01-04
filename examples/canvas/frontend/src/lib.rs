#![no_std]

use zoon::*;

blocks!{

    #[derive(Copy, Clone)]
    enum Color {
        Red,
        Blue,
    }

    #[s_var]
    fn color() -> Color {
        Color::A
    }

    #[update]
    fn toggle_color() {
        use Color::{Red, Blue};
        color().update(|color| if let Red = color { Blue } else { Red });
    }

    #[cache]
    fn fill_style() -> JsValue {
        let color = if let Color::Red = color.inner() { "red" } else { "blue" };
        JsValue::from(color)
    }

    #[el]
    fn root() -> Row {
        let fill_style = fill_style(); 
        row![
            canvas![
                canvas::width(300),
                canvas::height(300),
                canvas::on_ready(|canvas| {
                    let ctx = canvas.context_2d();
                    ctx.lineWidth = 10;
                    fill_style.use_ref(|style| ctx.set_fill_style(style));
                    // Wall
                    ctx.stroke_rect(75., 140., 150., 110.);
                    // Door
                    ctx.fill_rect(130., 190., 40., 60.);
                    // Roof
                    ctx.begin_path();
                    ctx.move_to(50., 140.);
                    ctx.line_to(150., 60.);
                    ctx.line_to(250., 140.);
                    ctx.close_path();
                    ctx.stroke();
                });
            ],
            button![
                button::on_press(toggle_color),
                "Change color",
            ]
        ]
    }

}

#[wasm_bindgen(start)]
pub fn start() {
    start!()
}
