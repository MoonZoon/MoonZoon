use zoon::*;

zoons!{

    #[derive(Copy, Clone)]
    enum Color {
        Red,
        Blue,
    }

    #[var]
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
                    ctx.strokeRect(75., 140., 150., 110.);
                    // Door
                    ctx.fillRect(130., 190., 40., 60.);
                    // Roof
                    ctx.beginPath();
                    ctx.moveTo(50., 140.);
                    ctx.lineTo(150., 60.);
                    ctx.lineTo(250., 140.);
                    ctx.closePath();
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

fn main() {
    start!()
}
