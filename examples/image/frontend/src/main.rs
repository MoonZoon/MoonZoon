use zoon::{format, *};

#[derive(Clone, Copy, Default)]
enum ImageType {
    #[default]
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
                .s(Background::new().url(self.url()).color(color!("DarkBlue")))
                .description("important image")
                .url(self.url())
                .into_raw(),
            Self::Background => El::new()
                .s(Align::center())
                .s(Width::fill().max(600))
                .s(Height::fill())
                .s(Background::new().url(self.url()).color(color!("DarkBlue")))
                .into_raw(),
        }
    }
}

static IMAGE_TYPE: Lazy<Mutable<ImageType>> = lazy::default();

fn main() {
    start_app("app", root);
}

fn root() -> impl Element {
    Stack::new()
        .s(Width::fill())
        .s(Height::fill())
        .layer_signal(IMAGE_TYPE.signal_ref(|type_| type_.element()))
        .layer(switcher())
}

fn switcher() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Align::new().top().center_x())
        .s(Padding::new().x(20).y(10))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("#222"), || color!("#111"))))
        .s(RoundedCorners::all_max())
        .label_signal(IMAGE_TYPE.signal_ref(|type_| {
            let next_type_name = match type_ {
                ImageType::Element => "Background",
                ImageType::Background => "Element",
            };
            format!("Switch to {next_type_name}")
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_press(|| {
            IMAGE_TYPE.update(|type_| match type_ {
                ImageType::Element => ImageType::Background,
                ImageType::Background => ImageType::Element,
            })
        })
}
