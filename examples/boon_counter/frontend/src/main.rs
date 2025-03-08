use zoon::*;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<i32>> = lazy::default();

fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Height::fill())
        .s(Background::new().color(color!("oklch(0.4 0 0)")))
        .item(Button::new().label("Load Boon program").on_press(load_boon_program))
        .item(boon_document_root())    
}

fn load_boon_program() {
    zoon::println!("Load!");
}

fn boon_document_root() -> impl Element {
    El::new().child("Boon root")
}

// fn counter() -> impl Element {
//     Row::new()
//         .s(Align::center())
//         .s(Gap::new().x(15))
//         .item(counter_button("-", -1))
//         .item_signal(COUNTER.signal())
//         .item(counter_button("+", 1))
// }

// fn counter_button(label: &str, step: i32) -> impl Element {
//     let (hovered, hovered_signal) = Mutable::new_and_signal(false);
//     Button::new()
//         .s(Width::exact(45))
//         .s(RoundedCorners::all_max())
//         .s(Background::new()
//             .color_signal(hovered_signal.map_bool(|| color!("#edc8f5"), || color!("#E1A3EE", 0.8))))
//         .s(Borders::all(
//             Border::new()
//                 .width(2)
//                 .color(color!("oklch(0.6 0.182 350.53 / .7")),
//         ))
//         .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
//         .label(label)
//         .on_press(move || *COUNTER.lock_mut() += step)
// }
