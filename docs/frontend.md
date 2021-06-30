# Frontend - Zoon

---

## Basics

The **Counter** example:

```rust
use zoon::*;

#[static_ref]
fn counter() -> &'static Mutable<i32> {
    Mutable::new(0)
}

fn increment() {
    counter().update(|counter| counter + 1)
}

fn decrement() {
    counter().update(|counter| counter - 1)
}

fn root() -> impl Element {
    Column::new()
        .item(Button::new().label("-").on_press(decrement))
        .item(Text::with_signal(counter().signal()))
        .item(Button::new().label("+").on_press(increment))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
```

The alternative **Counter** example with a local state:

```rust
use zoon::{*, println};

fn root() -> impl Element {
    println!("I'm different.");

    let (counter, counter_signal) = Mutable::new_and_signal(0);
    let on_press = move |step: i32| *counter.lock_mut() += step;

    Column::new()
        .item(Button::new().label("-").on_press(clone!((on_press) move || on_press(-1))))
        .item_signal(counter_signal)
        .item(Button::new().label("+").on_press(move || on_press(1)))
}

#[wasm_bindgen(start)]
pub fn start() {
    start_app("app", root);
}
```

### 1. The App Initialization

1. The function `start` is invoked automatically from the Javascript code.
1. Zoon's function `start_app` appends the element returned from the `root` function to the element with the id `app`.

    - You can also pass the value `None` instead of `"app"` to mount directly to `body` but it's not recommended.

    - When the `root` function is invoked (_note:_ it's invoked only once), all elements are immediately created and rendered to the browser DOM. (It means, for instance, methods `Column::new()` or `.item(..)` writes to DOM.)

    - Data stored in functions marked by the attribute `#[static_ref]` are lazily initialized on the first call.

### 2. Update

1. The user clicks the decrement button.

1. The function `decrement` is invoked.

1. `counter`'s value is decremented.

1. `counter` has type `Mutable` => it sends its updated value to all associated signals.

1. The new `counter` value is received through a signal and the corresponding text is updated.
    - In the original example, only the content of the `Text` element is changed.
    - In the alternative examples, the `counter` value is automatically transformed to a new `Text` element.

_Notes:_

- Read the excellent [tutorial](https://docs.rs/futures-signals/0.3.20/futures_signals/tutorial/index.html) for `Mutable` and signals in the `futures_signals` crate.
- `zoon::*` reimports most needed types and you can access some of Zoon's dependencies by `zoon::library` like `zoon::futures_signals`.
- `clone!` is a type alias for [enclose::enc](https://docs.rs/enclose/1.1.8/enclose/macro.enc.html).
- `static_ref`, `clone!` and other things can be disabled or set by Zoon's [features](https://doc.rust-lang.org/cargo/reference/features.html).

---

## Elements

The **Counter** example part:

```rust
Button::new().label("-").on_press(decrement)
```

We'll look at the `Button` element code (`crates/zoon/src/element/button.rs`). `Button` is a Zoon element, but you can create custom ones the same way.

There are three sections: `Element`, `Abilities` and `Attributes`.

```rust
use zoon::*;
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

make_flags!(Label, OnPress);

pub struct Button<LabelFlag, OnPressFlag> {
    raw_el: RawHtmlEl,
    flags: PhantomData<(LabelFlag, OnPressFlag)>,
}

impl Button<LabelFlagNotSet, OnPressFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawHtmlEl::new("div")
                .attr("class", "button")
                .attr("role", "button")
                .attr("tabindex", "0")
                .style("cursor", "pointer")
                .style("user-select", "none"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

impl<LabelFlag, OnPressFlag> UpdateRawEl<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {
    fn update_raw_el(mut self, updater: impl FnOnce(RawHtmlEl) -> RawHtmlEl) -> Self {
        self.raw_el = updater(self.raw_el);
        self
    }
}
```
- The macro `make_flags!` will be explained later.
- The element has to implement the trait `Element`.
- It's recommended to implement `UpdateRawEl` to allow users to customize the element and it's required for _abilities_.
- `RawHtmlEl::style` automatically add vendor prefixes for CSS property names when required. E.g. `"user-select"` will be replaced with `"-webkit-user-select"` on Safari and browsers on iOS. (Values aren't prefixed, let us know when it becomes a show-stopper for you.)

```rust
// ------ ------
//   Abilities
// ------ ------

impl<LabelFlag, OnPressFlag> Styleable<'_, RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> KeyboardEventAware<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> Focusable for Button<LabelFlag, OnPressFlag> {}
impl<LabelFlag, OnPressFlag> Hoverable<RawHtmlEl> for Button<LabelFlag, OnPressFlag> {}
```
Abilities are basically simple traits. For example when you implement `Styleable` then users can call the `.s(...)` method on your element:

```rust
MyElement::new().s(Padding::new().all(6))
``` 

You can find all built-in abilities in `crates/zoon/src/element/ability.rs`. The `Styleable` ability:

```rust
pub trait Styleable<'a, T: RawEl>: UpdateRawEl<T> + Sized {
    fn s(self, style: impl Style<'a>) -> Self {
        self.update_raw_el(|raw_el| style.update_raw_el_style(raw_el))
    }
}
```

_Note_: You can also implement your custom abilities.

```rust
// ------ ------
//  Attributes
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(mut self, label: impl IntoElement<'a> + 'a) -> Button<LabelFlagSet, OnPressFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child(label);
        self.into_type()
    }

    pub fn label_signal(
        mut self,
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Button<LabelFlagSet, OnPressFlag>
    where
        LabelFlag: FlagNotSet,
    {
        self.raw_el = self.raw_el.child_signal(label);
        self.into_type()
    }

    pub fn on_press(
        mut self,
        on_press: impl FnOnce() + Clone + 'static,
    ) -> Button<LabelFlag, OnPressFlagSet>
    where
        OnPressFlag: FlagNotSet,
    {
        self.raw_el = self
            .raw_el
            .event_handler(move |_: events::Click| (on_press.clone())());
        self.into_type()
    }

    fn into_type<NewLabelFlag, NewOnPressFlag>(self) -> Button<NewLabelFlag, NewOnPressFlag> {
        Button {
            raw_el: self.raw_el,
            flags: PhantomData,
        }
    }
}
```

_Note_: Attribute implementations look a bit verbose because of long types and generics but it's a trade-off for the user's comfort and safety. Also we will improve it once stable Rust has better support for const generics and other things. And you can always write your custom elements without generics to make the code simpler.

--

`make_flags!(Label, OnPress);` generates code like:
```rust
struct LabelFlagSet;
struct LabelFlagNotSet;
impl zoon::FlagSet for LabelFlagSet {}
impl zoon::FlagNotSet for LabelFlagNotSet {}

struct OnPressFlagSet;
struct OnPressFlagNotSet;
impl zoon::FlagSet for OnPressFlagSet {}
impl zoon::FlagNotSet for OnPressFlagNotSet {}
```

The only purpose of _flags_ is to enforce extra rules by the Rust compiler.

The compiler doesn't allow to call `label` or `label_signal` if the label is already set. The same rule applies for `on_press` handler.

```rust
Button::new()
    .label("-")
    .label("+")
```
fails with
```
error[E0277]: the trait bound `LabelFlagSet: FlagNotSet` is not satisfied
  --> frontend\src\lib.rs:20:14
   |
20 |.label("+"))
   | ^^^^^ the trait `FlagNotSet` is not implemented for `LabelFlagSet`
```

---

## Styles

The **Todos** example part:
   - _Note:_ The code below may differ from the current Todos implementation.

```rust
fn todo(todo: Arc<super::Todo>) -> impl Element {
    let checkbox_id = ElementId::new();
    let (row_hovered, row_hovered_signal) = Mutable::new_and_signal(false);
    let selected = {
        let todo_id = todo.id;
        super::selected_todo().signal(|selected_id| selected_id == Some(todo_id));
    };
    Row::new()
        .s(Font::new().size(24))
        .s(Padding::new().all(15))
        .s(Spacing::new(10))
        .on_hovered_change(move |hovered| row_hovered.set(hovered))
        .item(
            todo_checkbox(checkbox_id, &todo)
        )
        .item_signal(
            selected.map(clone!((todo) move |selected| {
                if selected { Box::new(selected_todo_title()) } 
                else { Box::new(todo_label(checkbox_id, &todo)) }
            }))
        )
        .item_signal(
            row_hovered_signal.map(|hovered| {
                hovered.then(move || remove_todo_button(&todo))
            })
        )
}
```

- CSS concepts / events like _focus_, _hover_ and _breakpoints_ are handled directly by Rust / Zoon _elements_.

- There is no such thing as CSS _margins_ or _selectors_. Padding and element nesting are more natural alternatives.

---
## Color

```rust
.s(Background::new().color(hsl(0, 0, 100)))
.s(
    BorderShadow::new()
        .offset_xy(0, 2)
        .size(0)
        .blur(4)
        .color(hsla(0, 0, 0, 20))
)
.s(Font::new().color_signal(hovered.map(|hovered| {
    if hovered { hsl(12, 35, 60) } else { hsl(10, 30, 50) }
})))
```

The most commonly used color code systems are:
- HEX - `#ffff00`, 
- RGB - `rgb(255, 255, 0)` 
- HSL - `hsl(60, 100%, 50%)`

_

However when you want to:
- create color palettes and themes
- make sure the button is slightly lighter or darker on hover
- make the text more readable

you often need to set saturation and lightness directly. Also it's nice to identify the hue on the first look when you are reading the code. These two conditions basically renders HEX and RGB unusable.

_

But there is also a problem with HSL. Let's compare these two colors:

<img src="images/yellow_hsl.svg" height="30px">
<img src="images/blue_hsl.svg" height="30px">

Are we sure they have the same lightness `50%`? I don't think so. The human eye perceives yellow as brighter than blue. Fortunately there is a color system that takes into account this perception: [HSLuv](https://www.hsluv.org/).

<img src="images/yellow_hsluv.svg" height="30px">
<img src="images/blue_hsluv.svg" height="30px">

That's why Zoon uses only HSLuv, represented in the code as `hsl(h, s, l)` or `hsla(h, s, l, a)`, where:
- `h` ;  _hue_  ; 0 - 360
- `s` ;  _saturation_  ; 0 - 100
- `l` ;  _lightness_  ; 0 - 100
- `a` ;  _alpha channel / opacity_ ; 0 (transparent) - 100 (opaque)

<details>
<summary>Other examples why color theory and design in general are difficult</summary>

- The human eye recognizes differences between lighter tones better than between darker tones. This fact is important for creating color palettes.
- Too extreme contrast weakens reading stamina - you shouldn't use pure black and white too often (unless you are creating a special theme for low vision users).
- Relatively many people are at least slightly color blind. It means, for example:
   - Red "Stop button" has to have also a text label.
   - Do you want to show different routes on the map? Use different line styles (e.g. dashed, dottted) instead of different colors.
   - The guy over there maybe doesn't know his T-shirt isn't gray but pink. (It's a typical issue for _deutan color blindness_; ~5% of men.)
   - Pick colors and labels for charts carefully - some charts could become useless for color blind people or when you decide to print them in a gray-scale mode. (HSLuv mode can help here a bit because you can pick colors with different lightness values.) 

</details>

---

## Size

### Units

CSS supports `cm`, `mm`, `in`, `px`, `pt`, `pc`, `em`, `ex`, `ch`, `rem`, `vw`, `vh`, `vmin`, `vmax` and `%`. I'm sure there were reasons for each of them, but let's just use `px`. Zoon may transform pixels to relative CSS units like `rem` or do other computations under the hood to improve accessibility.

### Font Size

Have you ever ever tried to align an element with a text block? An example:

<img src="images/element_text_alignment.svg" height="100px">

How can we measure or even remove the space above the `Zoon` text? It's an incredibly difficult task because the space is different for each font and it's impossible in CSS without ugly error-prone hacks.

You will be able to resolve it in the future CSS with some new properties, mainly with [leading-trim](https://www.w3.org/TR/css-inline-3/#leading-trim).
One of the comments for the article [Leading-Trim: The Future of Digital Typesetting](https://medium.com/microsoft-design/leading-trim-the-future-of-digital-typesetting-d082d84b202):
> _"This has been a huge annoyance to me for decades! I hope this gets standardized and implemented quickly, thank you for setting this in motion!_" - Tim Etler

_

Typography is one of the most complex parts of (web) design. But we have to somehow simplify it for our purposes.

So I suggest to make the _font size_ an alias for the [_cap height_](https://en.wikipedia.org/wiki/Cap_height). And the _font size_ would be also equal to the line height. It means the code:

```rust
Paragraph::new()
    .s(Font::new().size(40))
    .s(Spacing::new(30))
    .content("Moon")
    .content("Zoon")
```

would be rendered as:

<img src="images/font_size_example.svg" height="110px">

--

Related blog post: [_Font size is useless; let’s fix it_](https://tonsky.me/blog/font-size/) by Nikita Prokopov

---

## View & Viewport

The **Time Tracker** example part:
   - _Note:_ The code below may differ from the current Time Tracker implementation.

```rust
fn root() -> impl IntoRoot {
    View::new()
        .on_click(super::view_clicked)
        .viewport(
            ViewPort::new()
                .on_width_change(super::update_viewport_width)
        )
        .child(
            Column::new()
                .items(array::IntoIter::new([
                    header(),
                    menu_panel(),
                    page(),
                ])
        )
}
```

- `View` represents the root container for the web page.
- `Viewport` represents a part of the _View_ currently visible by the user. It could be used for scrolling and to help with writing responsive elements.
- The _view/viewport_ concept will probably be used for scrollable elements, too.  

---

## Built-in libraries / API
- They will be probably written as standalone crates or they'll need to be activated by feature flags.

### Timer
 
- Could be used as a timeout or stopwatch (to set an interval between callback calls).
- See `examples/timer` for the entire code.
    - _Note:_ The code below may differ from the current `timer` implementation.

```rust
#[static_ref]
fn timeout() -> &'static Mutable<Option<Timer>> {
    Mutable::new(None)
} 

fn start_timeout() {
    timeout().set(Some(Timer::new(2_000, stop_timeout)));
}

fn stop_timeout() {
    timeout().set(None);
}
```

### Connection + Task

- `UpMsg` are sent from Zoon to Moon. `DownMsg` in the opposite direction.
- `UpMsg` could be buffered when the Moon server is offline. And `DownMsg` when the Zoon client is automatically reconnecting.
- `UpMsg` are sent in a short-lived _fetch_ request, `DownMsg` are sent in a _server-sent event_ to provide real-time communication.
- A _correlation id_ is automatically generated and sent to the Moon with each request. Moon can send it back with the next `DownMsg`. You can also send an auth token together with the `UpMsg`.
- A _session id_ is automatically generated when the `Connection` is created. Then it's sent with each `UpMsg`. You can use it to simulate standard request-response mechanism.
- `Task::start` spawn the given `Future`. (_Note:_ Multithreading isn't supported yet.) 
- See `examples/chat` for the entire code.

```rust
#[static_ref]
fn connection() -> &'static Connection<UpMsg, DownMsg> {
    Connection::new(|DownMsg::MessageReceived(message), _| {
        messages().lock_mut().push_cloned(message);
    })
    // .auth_token_getter(|| AuthToken::new("my_auth_token"))
}

fn send_message() {
    Task::start(async {
        connection()
            .send_up_msg(UpMsg::SendMessage(Message {
                username: username().get_cloned(),
                text: new_message_text().take(),
            }))
            .await
            .unwrap_or_else(|error| eprintln!("Failed to send message: {:?}", error))
    });
}
```

### Routing

- An example with the nested route `admin::Route`.
- See `examples/pages` for the entire code.
    - _Note:_ The code below may differ from the current `pages` implementation.

```rust
#[route]
enum Route {
    #[route("admin", ..)]
    Admin(admin::Route),
    #[route()]
    Root,
    Unknown,
}
```

_

- A more complete example with _guards_ and Zoon's URL helpers.
- See `examples/time_tracker` for the entire code.
    - _Note:_ The code below may differ from the current `time_tracker` implementation.

```rust
#[route]
enum Route {
    #[route("login")]
    #[before_route(before_login_route)]
    Login,

    #[route("clients_and_projects")]
    #[before_route(before_protected_route)]
    ClientsAndProjects,

    #[route()]
    Home,

    #[before_route(before_unknown_route)]
    Unknown,
}

fn before_login_route(route: Route) -> Route {
    if user().map(Option::is_none) {
        return route
    }
    Route::home()
}

fn before_protected_route(route: Route) -> Route {
    if user().map(Option::is_some) {
        return route
    }
    Route::login()
}

fn before_unknown_route(route: Route) -> Route {
    Route::home()
}

fn route() -> impl Signal<Item = Route> {
    url().signal().map(Route::from)
}

fn set_route(route: Route) {
    url().set(Url::from(route))
}
```

---

## SEO

- When the request comes from a robot (e.g. _Googlebot_), then MoonZoon renders elements to a HTML string and sends it back to the robot. (It's basically a limited _Server-Side Rendering_.)  

- You'll be able to configure the default page title, _The Open Graph Metadata_ and other things in the Moon app. The example below will be continuously updated once.
    ```rust
    async fn frontend() -> Frontend {
        Frontend::new().title("Time Tracker example")
    }
    ```

---

## FAQ
1. _"Why another frontend framework? Are you mad??"_
   - Because I have some problems with the existing ones. Some examples:

        <details>
        <summary>Problems with existing frontend frameworks</summary>

        - I'm not brave enough to write apps and merge pull requests written in a dynamic language.
        - I'm tired of configuring Webpack-like bundlers and fixing bugs caused by incorrectly typed JS libraries to Typescript.
        - I want to share code between the client and server and I want server-side rendering and I don't want to switch context (language, ecosystem, best practices, etc.) while I'm writing both frontend and server.
        - I don't want to read the entire stackoverflow.com and MDN docs to find out why my image on the website has incorrect size.
        - I don't want to be afraid to refactor styles.
        - I don't want to write code on the backend instead on the frontend because frontend is just too slow.
        - Who have time and energy to properly learn, write and constantly think about accessibility and write unit tests that take into account weird things like `null` or `undefined`?
        - I'm tired of searching for missing semicolons and brackets in HTML and CSS when it silently fails in the runtime.
        - I don't want to choose a CSS framework, bundler, state manager, router, bundler plugins, CSS preprocessor plugins, test framework and debug their incompatibilities and learn new apis everytime I want to create a new web project.
        - Why the layout is broken on iPhone, the app crashes on Safari, it's slow on Chrome and scrollbars don't work on Windows? 
        - I just want to send a message to a server. I don't want to handle retrying, set headers, set timeout, correctly serialize everything, handle errors by their numbers, constantly think about cookies, domains, protocols, XSS, CSRF, etc.
        - What about SEO?
        - Should I use standard routing, hash routing, query parameters, custom base paths? Is everything correctly encoded and decoded?
        - etc.
        
        </details>
        
1. _"How are we taking care of animations?"_ (by None on [chat](https://discord.gg/eGduTxK2Es))
   - The API for animations haven't been designed yet. We'll probably focus on it once we have a proof-of-concept of the basic MoonZoon features.
   - Inspiration:
      - [react-spring](https://www.react-spring.io/)
      - [Framer Motion](https://www.framer.com/motion/)
      - [React UseGesture](https://use-gesture.netlify.app/)
      - [elm-animator](https://korban.net/posts/elm/2020-04-07-using-elm-animator-with-elm-ui/)
      - "svelte has really good set of animation examples in their tutorial site. Incase it can help somehow [section 9 -11]" (by Ruman on [chat](https://discord.gg/eGduTxK2Es))
      - [rust-dominator/examples/animation](https://github.com/Pauan/rust-dominator/blob/master/examples/animation/src/lib.rs)

1. _"Hey Martin, what about [Seed](https://seed-rs.org/)?"_
   - Zoon and Seed have very different features and goals. I assume we will be able to implement some interesting features inspired by Zoon in Seed, if needed. I'll maintain Seed as usual.
