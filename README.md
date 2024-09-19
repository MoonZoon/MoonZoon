<p align="center">
  <img src="branding/MoonZoon_logo_both_texts_light_on_black.svg" width="630" title="MoonZoon logo">
</p>

<p align="center">
  <a href="https://discord.gg/eGduTxK2Es">
  <img src="https://discordapp.com/api/guilds/797429007683158046/widget.png?style=banner2" title="MoonZoon Discord">
  </a>
</p>

### _"We don't want to lose time and money due to millions of unnecessary technical micro-decisions."_

---

**MoonZoon** is a Rust Fullstack Framework to build Web and Desktop apps with Multithreading and Durable Computing. No HTML, CSS or JS needed.

---

# Code example

<p align="center">
  <img src="docs/images/counter_demo.gif" width="559" title="MoonZoon logo">
</p>

```rust
use zoon::*;

fn main() {
    start_app("app", root);
}

static COUNTER: Lazy<Mutable<i32>> = lazy::default();

fn root() -> impl Element {
    Row::new()
        .s(Align::center())
        .s(Gap::new().x(15))
        .item(counter_button("-", -1))
        .item_signal(COUNTER.signal())
        .item(counter_button("+", 1))
}

fn counter_button(label: &str, step: i32) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Button::new()
        .s(Width::exact(45))
        .s(RoundedCorners::all_max())
        .s(Background::new()
            .color_signal(hovered_signal.map_bool(|| color!("#edc8f5"), || color!("#E1A3EE", 0.8))))
        .s(Borders::all(
            Border::new()
                .width(2)
                .color(color!("oklch(0.6 0.182 350.53 / .7")),
        ))
        .on_hovered_change(move |is_hovered| hovered.set(is_hovered))
        .label(label)
        .on_press(move || *COUNTER.lock_mut() += step)
}
```

# Demos

## New Project Template [on Netlify](https://moonzoon-new-project.netlify.app/) | [Template](https://github.com/MoonZoon/MoonZoon/tree/main/crates/mzoon/new_project)
   - Used by the command `mzoon new` (see the section [Create & Run project](#create--run-project) below)

## Chat
   - Repo: [examples/chat](examples/chat)
   - Related blog post: _"Chat example, MoonZoon Cloud [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4) / [MD](docs/articles/dev_news_5.md)]_.

      ![Chat example](docs/images/chat_example.gif)

---

# Create & Run project

1. Install the latest stable [Rust](https://www.rust-lang.org/tools/install). (Or upgrade with `rustup update stable`.)
1. Install the web assembly target `rustup target add wasm32-unknown-unknown`
1. `cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --locked`
1. `mzoon new my_counter`
1. `cd my_counter`
1. `mzoon start --open`

   ![mzoon start --open](docs/images/mzoon_start_open.png)

---

# Deploy project

## Both Frontend & Backend

I use [Coolify](https://coolify.io/) on [Hetzner](https://www.hetzner.com/) with this `Dockerfile`:

```dockerfile
FROM rust:1
WORKDIR /app

RUN rustup target add wasm32-unknown-unknown
# NOTE: Set `--rev` to the commit you use in your project
RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
    cargo install mzoon --git https://github.com/MoonZoon/MoonZoon --rev ccc15d043e78a6656d68a60d46de1f540724e093 --locked

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo,from=rust,source=/usr/local/cargo \
    --mount=type=cache,target=target \
    /usr/local/cargo/bin/mzoon build -r

RUN --mount=type=cache,target=target \
    ["cp", "./target/release/backend", "/usr/local/bin/moon_app"]

ENTRYPOINT ["moon_app"]
```

MoonZoon app was successfully deployed to other services like [Heroku](https://www.heroku.com/) ([buildpack](https://github.com/MoonZoon/heroku-buildpack-moonzoon)), [Clever Cloud](https://www.clever-cloud.com/), [CapRover](https://caprover.com/) or [Fly.io](https://fly.io/). 

## Frontend-only

1. `mzoon build --release --frontend-dist netlify` 
   - Hosting name (`netlify`) is optional. It creates files like `netlify.toml`.
1. _[optional]_ Test it with a dev server like [miniserve](https://crates.io/crates/miniserve):
   ```bash
   cargo install miniserve
   
   miniserve --port 8079 --index index.html --spa frontend_dist
   ```
1. Drag & drop the `frontend_dist` directory to [Netlify](https://www.netlify.com/). 

   ![moonzoon-new-project on Netlify](docs/images/moonzoon-new-project_netlify.png)

---

# Examples

- [**Examples**](examples) in the repo [See [development.md](docs/development.md) for instructions how to run them.]

- [Raytracer on MoonZoon](https://github.com/MartinKavik/ray_tracer) [Not maintained)

- [Voting system](https://github.com/MartinKavik/voting-solana-moonzoon) on the [Solana](https://solana.com/) blockchain. [Not maintained)

   ![Voting system example](docs/images/voting_system_example.png)

---

# Blog

- Cure for Web Development [[Medium](https://martinkavik.medium.com/cure-for-web-development-65003afb701f) / [dev.to](https://dev.to/martinkavik/cure-for-web-development-nnn) / [MD](docs/articles/cure_for_web_development.md)]

- Rust on the Frontend and Backend [[Always Bet on Rust](https://blog.abor.dev/p/moonzoon)]

- Interview with Martin about MoonZoon [[console.substack.com](https://console.substack.com/p/console-114)]

### Dev News

1. CLI, Build pipeline, Live-reload, HTTPS [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-1-cli-build-pipeline-live-reload-https-1ba6) / [MD](docs/articles/dev_news_1.md)]

1. Live demo, Zoon, Examples, Architectures [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-2-live-demo-zoon-examples-architectures-2oem) / [MD](docs/articles/dev_news_2.md)]

1. Signals, React-like Hooks, Optimizations [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-3-signals-react-like-hooks-optimizations-39lp) / [MD](docs/articles/dev_news_3.md)]

1. Actix, Async CLI, Error handling, Wasm-pack installer [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-4-actix-async-cli-error-handling-wasm-pack-installer-57cp) / [MD](docs/articles/dev_news_4.md)]

1. **Chat example, MoonZoon Cloud [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-5-chat-example-moonzoon-cloud-5de4) / [MD](docs/articles/dev_news_5.md)]**

---

# Documentation

### 1. [Philosophy & Non-Goals](docs/philosophy_and_non_goals.md).md

### 2. [Frontend](docs/frontend.md).md

### 3. [Backend](docs/backend.md).md

### 4. [CLI](docs/cli.md).md

### 6. [Development](docs/development.md).md

---

# Size & Speed

![Benchmark example size](docs/images/demo_size_small.png)

[![Benchmark example speed](docs/images/frontend_frameworks_benchmark_1.png)](https://raw.githubusercontent.com/MoonZoon/MoonZoon/main/docs/images/frontend_frameworks_benchmark_screen.png)

- [MartinKavik/js-framework-benchmark](https://github.com/MartinKavik/js-framework-benchmark/tree/framework/moonzoon)

---

# Sponsors

<p align="center">
    <a href="https://NLnet.nl">
        <img src="docs/images/nlnet_logo.png" width="269" alt="Logo NLnet">
    </a>
</p>

<p align="center">
    <a href="https://github.com/sponsors/MartinKavik">
        <img src="docs/images/github_sponsors_logo.png" width="300" alt="Logo GitHub Sponsors">
    </a>
</p>

---

Thank you for reading! We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).
