use zoon::{*, named_color::*, println};

#[static_ref]
fn playing() -> &'static Mutable<bool> {
    Default::default()
}

fn root() -> impl Element {
    let video_element = Mutable::new(None);    
    Column::new()
        .s(Padding::all(20))
        .s(Width::fill().max(600))
        .s(Align::center())
        .s(Spacing::new(20))
        // Note: `web_sys` element (aka `DomElement`) cloning is cheap 
        // because it clones only a reference to associated Javascript/browser DOM element.
        .item(video(video_element.clone()))  
        .item(play_button(video_element))  
}

fn video(video_element: Mutable<Option<web_sys::HtmlVideoElement>>) -> impl Element {
    // @TODO Improve `event_handler` API / creating events.
    make_event!(VideoPlay, "play" => web_sys::Event);
    make_event!(VideoPause, "pause" => web_sys::Event);

    // Note: `RawHtmlEl::new(..)`'s default `DomElement` is `web_sys::HtmlElement`.
    RawHtmlEl::<web_sys::HtmlVideoElement>::new("video")
        .attr("controls", "")
        .use_dom_element(|this, dom_element| {
            // Note: Poster may cause video element height changes (e.g. in Chrome at the time of writing).
            dom_element.set_poster("https://storage.googleapis.com/gtv-videos-bucket/sample/images/BigBuckBunny.jpg");
            dom_element.set_src("https://storage.googleapis.com/gtv-videos-bucket/sample/BigBuckBunny.mp4");
            this
        })
        .after_insert(move |dom_element| video_element.set(Some(dom_element)))
        .event_handler(|_: VideoPlay| playing().set_neq(true))
        .event_handler(|_: VideoPause| playing().set_neq(false))
        // Unchecked cast `DomElement` to another type.
        .dom_element_type::<web_sys::HtmlMediaElement>()
}

fn play_button(video_element: Mutable<Option<web_sys::HtmlVideoElement>>) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let size = 50;
    Button::new()
        // @TODO .(s) visibility
        .s(Background::new().color_signal(hovered_signal.map_bool(|| BLUE_7, || BLUE_9)))
        .s(Align::new().center_x())
        .s(Width::new(size))
        .s(Height::new(size))
        .s(RoundedCorners::all_max())
        .s(Borders::all(Border::new().color(BLUE_5).width(2)))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .on_click(move || {
            video_element.use_ref(|video| {
                let video = video.as_ref().expect_throw("failed to get video element");
                if video.paused() {
                    // See https://developer.mozilla.org/en-US/docs/Web/API/HTMLMediaElement/play
                    let play_promise = video.play().expect_throw("failed to play video");
                    Task::start(async {
                        JsFuture::from(play_promise).await.expect_throw("failed to play video");
                        println!("Play button clicked and playing!")
                    })
                } else {
                    video.pause().expect_throw("failed to pause video");
                }
            })
        })
        .label(play_button_icon())
}

fn play_button_icon() -> impl Element {
    macro_rules! make_icon {
        ($name:literal) => {
            $crate::paste! {
                fn [<icon_ $name>]() -> RawSvgEl<web_sys::SvgsvgElement> {
                    // Note: Icons downloaded from https://remixicon.com/.
                    RawSvgEl::from_markup(include_str!(concat!("../icons/", $name, ".svg")))
                        .unwrap_throw()
                        // Set `currentColor` in SVG elements.
                        .style("color", &BLUE_3.to_string())
                }
            }
        };
    }
    
    // Tip: You can write a `build.rs` script to automatically generate the lines below 
    // according to the files in the `icons` folder,
    // see https://doc.rust-lang.org/cargo/reference/build-scripts.html
    make_icon!("play-fill");
    make_icon!("pause-fill");

    El::new()
        .child_signal(playing().signal().map_bool(icon_pause_fill, icon_play_fill))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
