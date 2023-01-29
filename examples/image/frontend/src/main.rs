use zoon::{format, named_color::*, *};

#[derive(Clone, Copy)]
enum ImageType {
    Element,
    Background,
}

impl ImageType {
    fn url(&self) -> String {
        match self {
            Self::Element => public_url("picsum-89-800x800.jpg"),
            Self::Background => public_url("picsum-961-800x800.jpg"),
        }
    }

    fn element(&self) -> impl Element {
        match self {
            Self::Element => Image::new()
                .s(Align::center())
                .s(Width::fill().max(600))
                .s(Background::new().url(self.url()).color(BLUE_9))
                .description("important image")
                .url(self.url())
                .into_raw_element(),
            Self::Background => El::new()
                .s(Align::center())
                .s(Width::fill().max(600))
                .s(Height::fill())
                .s(Background::new().url(self.url()).color(BLUE_9))
                .into_raw_element(),
        }
    }
}

#[static_ref]
fn image_type() -> &'static Mutable<ImageType> {
    Mutable::new(ImageType::Element)
}

fn root() -> impl Element {
    Stack::new()
        .s(Width::fill())
        .s(Height::fill())
        .layer_signal(image_type().signal_ref(|type_| type_.element()))
        .layer(switcher())
}

fn switcher() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().top().center_x())
        .s(Padding::new().x(20).y(10))
        .s(Background::new().color_signal(hovered_signal.map_bool(|| GRAY_8, || GRAY_9)))
        .s(RoundedCorners::all_max())
        .label_signal(image_type().signal_ref(|type_| {
            let next_type_name = match type_ {
                ImageType::Element => "Background",
                ImageType::Background => "Element",
            };
            format!("Switch to {next_type_name}")
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(|| {
            image_type().update(|type_| match type_ {
                ImageType::Element => ImageType::Background,
                ImageType::Background => ImageType::Element,
            })
        })
}

fn main() {
    start_app("app", root);
}
