use shared::{DownMsg, Message, UpMsg};
use zoon::{eprintln, *};

mod markup;

static USERNAME: Lazy<Mutable<String>> = Lazy::new(|| Mutable::new("John".to_owned()));
static MESSAGES: Lazy<MutableVec<Message>> = lazy::default();
static NEW_MESSAGE_TEXT: Lazy<Mutable<String>> = lazy::default();
static RECEIVED_MESSAGES_VIEWPORT_Y: Lazy<Mutable<i32>> = lazy::default();

pub static CONNECTION: Lazy<Connection<UpMsg, DownMsg>> = Lazy::new(|| {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        MESSAGES.lock_mut().push_cloned(message);
        // jump to bottom
        RECEIVED_MESSAGES_VIEWPORT_Y.set(i32::MAX);
    })
});

fn send_message() {
    Task::start(async {
        let result = CONNECTION
            .send_up_msg(UpMsg::SendMessage(Message {
                username: USERNAME.get_cloned(),
                text: NEW_MESSAGE_TEXT.take(),
            }))
            .await;
        if let Err(error) = result {
            eprintln!("Failed to send message: {:?}", error);
        }
    });
}

fn main() {
    start_app("app", root);
    CONNECTION.init_lazy();
}

fn root() -> impl Element {
    El::new()
        .s(Padding::new().y(20))
        .s(Height::screen())
        .child(content())
}

fn content() -> impl Element {
    Column::new()
        .s(Width::exact(300))
        .s(Height::fill())
        .s(Align::new().center_x())
        .s(Gap::both(20))
        .item(received_messages())
        .item(new_message_panel())
        .item(username_panel())
}

// ------ received_messages ------

fn received_messages() -> impl Element {
    El::new()
        .s(Height::fill())
        .s(Scrollbars::both())
        .viewport_y_signal(RECEIVED_MESSAGES_VIEWPORT_Y.signal())
        .child(
            Column::new()
                .s(Align::new().bottom())
                .items_signal_vec(MESSAGES.signal_vec_cloned().map(received_message)),
        )
}

fn received_message(message: Message) -> impl Element {
    Column::new()
        .s(Padding::all(10))
        .s(Gap::both(6))
        .item(
            El::new()
                .s(Font::new()
                    .weight(FontWeight::Bold)
                    .color(color!("#EEE"))
                    .size(17))
                .child(message.username),
        )
        .item(
            Paragraph::new()
                .s(Font::new().color(color!("#DDD")).size(17).line_height(27))
                .contents(message_text_to_contents(&message.text)),
        )
}

fn message_text_to_contents(text: &str) -> impl Iterator<Item = RawElOrText> + '_ {
    markup::parse_markup_objects(text).map(|object| match object {
        markup::Object::Text(text) => Text::new(text).into_raw(),
        markup::Object::Smile => emoji("smile").into_raw(),
        markup::Object::SlightSmile => emoji("slight_smile").into_raw(),
    })
}

fn emoji(name: &str) -> impl Element {
    Image::new()
        .s(Height::exact(17))
        .s(Transform::new().scale(230).move_down(2))
        .s(Padding::new().x(5))
        .url(public_url!("emoji/{name}.png"))
        .description(name)
}

// ------ new_message_panel ------

fn new_message_panel() -> impl Element {
    Row::new().item(new_message_input()).item(send_button())
}

fn new_message_input() -> impl Element {
    TextInput::new()
        .s(Padding::all(10))
        .s(RoundedCorners::new().left(5))
        .s(Width::fill())
        .s(Font::new().size(17))
        .focus(true)
        .on_change(|text| NEW_MESSAGE_TEXT.set(text))
        .label_hidden("New message text")
        .placeholder(Placeholder::new("Message"))
        .on_key_down_event(|event| event.if_key(Key::Enter, send_message))
        .text_signal(NEW_MESSAGE_TEXT.signal_cloned())
}

fn send_button() -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Padding::all(10))
        .s(RoundedCorners::new().right(5))
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("Green"), || color!("DarkGreen"))))
        .s(Font::new().color(color!("#EEE")).size(17))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .on_press(send_message)
        .label("Send")
}

// ------ username_panel ------

fn username_panel() -> impl Element {
    let id = "username_input";
    Row::new()
        .s(Gap::both(15))
        .item(username_input_label(id))
        .item(username_input(id))
}

fn username_input_label(id: &str) -> impl Element {
    Label::new()
        .s(Font::new().color(color!("#EEE")))
        .for_input(id)
        .label("Username:")
}

fn username_input(id: &str) -> impl Element {
    TextInput::new()
        .s(Width::fill())
        .s(Padding::new().x(10).y(6))
        .s(RoundedCorners::all(5))
        .update_raw_el(|raw_el| {
            // don't autocomplete usernames by 1Password
            raw_el.attr("data-1p-ignore", "")
        })
        .id(id)
        .on_change(|username| USERNAME.set(username))
        .placeholder(Placeholder::new("Joe"))
        .text_signal(USERNAME.signal_cloned())
}
