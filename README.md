<p align="center">
  <img src="branding/MoonZoon_logo_readme.png" height="300" title="MoonZoon logo">
</p>

### [Discord](https://discord.gg/eGduTxK2Es) | [moonzoon.rs](http://moonzoon.rs)* | [martin@moonzoon.rs](mailto:martin@moonzoon.rs)

\* redirects to this repo until ready

---

### _"We don't want to lose time and money due to millions of unnecessary micro-decisions and bikeshedding."_

**MoonZoon** is a Rust Fullstack Framework.

<div style="display: flex; flex-wrap: wrap;">
    <ul style="border-right: 1px solid; padding-right: 1rem">
        <li>NO Javascript</li>
        <li>NO HTML</li>
        <li>NO CSS</li>
        <li>NO REST</li>
        <li>NO GraphQL</li>
        <li>NO SQL</li>
        <li>NO Analysis Paralysis
    </ul>
    <ul>
        <li>Rust</li>
        <li>Fast</li>
        <li>Simple</li>
        <li>Scalable</li>
        <li>SEO</li>
        <li>Offline Support</li>
        <li>Easy Deploy</li>
    </ul>
<div>

## Philosophy

**A)** Users shouldn't have to learn many languages, tools and patterns to just start a simple project.

**B)** No dogmas (Functions vs Objects, Mutable vs Immutable, Monolith vs Micro-services, etc.).

**C)** _Example-driven development_ - No API changes without simple examples based on real-world needs.

**D)** No special terminology. Let's try to use words like _portal_, _hook_, _atom_ or _selector_ as little as possible because they have different meanings in different frameworks.

**E)** Don't build artificial barriers for users - if they want to use REST, CSS or SQL, don't try to stop them.

**F)** There is no silver bullet. However there should be a common denominator for most web projects.

**G)** There should be always a clear path forward for users. They will recognize when they need non-core tools and they will integrate them when needed, not before.

**H)** People make mistakes and things too complex. So linters, compilers, formatters and other tools should be as pedantic as possible. Ideally no configuration options for anything. Breaking changes are encouraged to reduce accidental complexity and technical debt.

## Non-Goals

**a)** Satisfy everybody.

**b)** Make the framework super-universal, i.e. natively support all use-cases, protocols and external services.

**c)** Don't change API, goals and features in the future.

**d)** Set and meet deadlines. It's done when it's done.

**e)** Deliver all features at once. Don't expect to deploy scalable realtime apps on a stable cloud with the first MoonZoon release.

---

# Frontend - Zoon

## Basics

The **Counter** example:

```rust
#![no_std]

use zoon::*;

blocks!{

    #[s_var]
    fn counter() -> i32 {
        0
    }

    #[update]
    fn increment() {
        counter().update(|counter| counter + 1);
    }

    #[update]
    fn decrement() {
        counter().update(|counter| counter - 1);
    }

    #[el]
    fn root() -> Column {
        column![
            button![button::on_press(decrement), "-"],
            counter().inner(),
            button![button::on_press(increment), "+"],
        ]
    }

}

#[wasm_bindgen(start)]
pub fn start() {
    start!()
}
```

### 1. The App Initialization

1. The function `start` is invoked automatically from the Javascript code.
1. Zoon's macro `start!` is called to start the app.
1. The function `counter` is invoked and its default value `0` is stored in the Zoon's internal storage.
1. The function `root` is invoked and its value is stored in the Zoon's internal storage, too.
1. The `counter` function was called in the `root`'s body - it means `root` has subscribed to `counter` changes and it will be automatically invoked on each `counter` change. 

### 2. The First Render

1. Zoon waits until the browser is ready for rendering.
1. The entire `#[el]` tree (only `root` in this example) is rendered to the predefined location in the browser DOM.

### 3. Update

1. The user clicks the decrement button.
1. The function `decrement` is invoked.
1. `counter`'s value is decremented. 
   - _Note_: The function `counter` actually returns `SVar<i32>` (_**S**tatic **Var**iable_) - basically a copyable wrapper for a typed reference to the Zoon's internal storage.
1. `root` element listens for `counter` changes - it's automatically recomputed and Zoon waits for the browser to allow rendering.
1. Elements dependent on changed data are effectively rerendered in the DOM. 
   - _Note_: When a parent element has to be rerendered, it doesn't mean that all its descendants have to be rerendered as well - each `#[el]` block may depend on different variables.

## Elements & Styles

The **Todos** example part:

```rust
    #[el]
    fn todo(todo: Var<super::Todo>) -> Row {
        let selected = Some(todo) == super::selected_todo();
        let checkbox_id = el_var(ElementId::new);
        let row_hovered = el_var(|| false);
        row![
            font::size(24),
            padding!(15),
            spacing(10),
            on_hovered_change(row_hovered.setter()),
            todo_checkbox(checkbox_id, todo),
            selected.not().then(|| todo_label(checkbox_id, todo)),
            selected.then(selected_todo_title),
            row_hovered.inner().then(|| remove_todo_button(todo)),
        ]
    }
```

- The blocks marked `#[el]` are functions that can contain its own state. The state variables (e.g. `ElVar<bool>`, _**El**ement **Var**iable_) are created by the `el_var` function and are dropped when their container (aka _function instance_ or _component_) is removed from the element tree.

- `[#el]` blocks accept only _Zoon variables_ like `SVar` or `ElVar` as arguments.

- Element macros (e.g. `row`) accepts only compatible attributes and children.

- Also concepts or events like _focus_, _hover_ and _breakpoints_ are handled by Zoon.

- There isn't something like _margins_ or _selectors_.

- All elements should be _accessible_ by default or at least make it easy to set it correctly.

## View & Viewport

The **Time Tracker** example part:

```rust
    #[el]
    fn root() -> View {
        view![
            viewport::on_width_change(super::update_viewport_width),
            on_click(super::view_clicked),
            column![
                header(),
                menu_panel(),
                page(),
            ]
        ]
    }
```

- `view` represents the root container for the web page.
- `viewport` represents a part of the _view_ currently visible by the user. It could be used for scrolling and to write responsive elements.
- The _view/viewport_ concept will be probably used for scrollable elements, too.  

## Built-in libraries / API
- They will be probably written as standalone crates or they'll need to be activated by feature flags.

### Timer
 
- Could be used as a timeout or stopwatch (to set an interval between callback calls). 
- See `examples/timer` for the entire code.

```rust
    #[s_var]
    fn timeout() -> Option<Timer> {
        None
    }

    #[update]
    fn start_timeout() {
        timeout().set(Some(Timer::new(2_000, stop_timeout)));
    }

    #[update]
    fn stop_timeout() {
        timeout().set(None);
    }
```

### Connection

- `UpMsg` are sent from Zoon to Moon. `DownMsg` in the opposite direction.
- `UpMsg` could be buffered when the Moon (server) is offline. And `DownMsg` when the Zoon (client) is automatically reconnecting.
- `UpMsg` are sent in a short-lived _fetch_ request, `DownMsg` are sent in a _server-sent event_  to provide real-time communication.
- A _correlation id_ is automatically generated and sent to the Moon with each request. Moon sends it back. You can also send a token together with the `UpMsg`. 
- See `examples/chat` for the entire code.

```rust
    #[s_var]
    fn connection() -> Connection<UpMsg, DownMsg> {
        Connection::new(|down_msg, _| {
            if let DownMsg::MessageReceived(message) = down_msg {
                ...
            }
        })
    }

    #[update]
    fn send_message() {
        connection().use_ref(|connection| {
            connection.send_up_msg(UpMsg::SendMessage(...), None);
        });
    }
```

### Routing

- An example with the nested route `admin::Route`.
- See `examples/pages` for the entire code.

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

- A more complete example with _guards_ and Zoon's function `url()`. 
- See `examples/time_tracker` for the entire code.

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

    #[cache]
    fn route() -> Route {
        url().map(Route::from)
    }

    #[update]
    fn set_route(route: Route) {
        url().set(Url::from(route))
    }
```

## SEO

- When the request comes from a robot (e.g. _Googlebot_), then MoonZoon renders elements to a HTML string and sends it back to the robot. (It's basically a limited _Server-Side Rendering_.)  

- You'll be able to configure the default page title, _The Open Graph Metadata_ and other things in the Moon app.
   - Example (draft API design):
   ```rust
   async fn frontend() -> Frontend {
       Frontend::new().title("Time Tracker example")
   }
   ```

## FAQ
1. _"Why another frontend framework? Are you mad??"_
   - Because I have some problems with the existing ones. For example:

        <details>
        <summary>Problems with existing frontend frameworks</summary>

        - I'm not brave enough to write apps and merge pull requests written in a dynamic language.
        - I'm tired of configuring Webpack-like bundlers and fixing bugs caused by incorrectly typed JS libraries to Typescript.
        - I want to share code between the client and server and I want server-side rendering and I don't want to switch context (language, ecosystem, best practices, etc.) while I'm writing both frontend and server.
        - I don't want to reread the entire stackoverflow.com and MDN docs to find out why my image on the website has incorrect size.
        - I don't want to be afraid to refactor styles.
        - I don't want to write code on the backend instead on the frontend because frontend is just too slow.
        - Who have time and energy to properly learn, write and constantly think about accessibility and write unit tests that take into account weird things like `null` or `undefined`?
        - I'm tired of searching for missing semicolons and brackets in HTML and CSS when it silently fails in the runtime.
        - I don't want to choose a CSS framework, bundler, state manager, router, app plugins, bundler plugins, CSS compiler plugins, test framework and debug their incompatibilities and learn new apis everytime I want to create a new project.
        - Why the layout is broken on iPhone, the app crashes on Safari, it's slow on Chrome and scrollbars don't work on Windows? 
        - I just want to send a message to a server. I don't want to handle retrying, set headers, set timeout, correctly serialize everything, handle errors by numbers, constantly think about cookies, domains, protocols, etc.
        - What about SEO?
        - Should I use standard routing, hash routing, query parameters, custom base paths? Is everything correctly encoded and decoded?
        - etc.
        
        </details>

1. _"Hey Martin, what about [Seed](https://seed-rs.org/)?"_
   - Zoon and Seed have very different features and goals. I assume we will be able to implement some interesting features inspired by Zoon in Seed, if needed. I'll maintain Seed as usual.

---

# Backend - Moon

---

# Current State

# Next Steps

# Contributing

# OpenHope


