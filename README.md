<p align="center">
  <img src="branding/MoonZoon_logo_readme.png" width="360" title="MoonZoon logo">
</p>

<p align="center">
  <a href="https://discord.gg/eGduTxK2Es">
  <img src="https://invidget.switchblade.xyz/eGduTxK2Es" width="430" title="MoonZoon Discord">
  </a>
</p>

### [moonzoon.rs](http://moonzoon.rs)* | [martin@moonzoon.rs](mailto:martin@moonzoon.rs) | [Discord](https://discord.gg/eGduTxK2Es)

\* redirects to this repo until ready

---

### _"We don't want to lose time and money due to millions of unnecessary technical micro-decisions."_

---

**MoonZoon** is a Rust Fullstack Framework.

<table>
    <td>
        <ul>
            <li>NO Javascript</li>
            <li>NO CSS</li>
            <li>NO HTML</li>
            <li>NO REST</li>
            <li>NO GraphQL</li>
            <li>NO SQL</li>
            <li>NO Analysis Paralysis</li>
            <li>NO Wheel Reinventing</li>
            <li>NO Passwords*</li>
        </ul>
    </td>
    <td>
        <ul>
            <li>Rust</li>
            <li>Fast</li>
            <li>Simple</li>
            <li>Scalable</li>
            <li>SEO</li>
            <li>Auth</li>
            <li>MoonZoon CLI</li>
            <li>Easy Deploy</li>
            <li>Offline Support</li>
        </ul>
    </td>
</table>

\* Authentication methods are described in [Backend](docs/backend.md).md

---

# Demo

- Live demo: [moonzoon-demo.herokuapp.com](https://moonzoon-demo.herokuapp.com/)
   - _Note_: Heroku dyno slow cold starts may occur.
- Repo: [github.com/MoonZoon/demo](https://github.com/MoonZoon/demo)
   - _Note_: You can use the repo as a template for your new project for now.

---

# Blog

- Cure for Web Development [[Medium](https://martinkavik.medium.com/cure-for-web-development-65003afb701f) / [dev.to](https://dev.to/martinkavik/cure-for-web-development-nnn) / [MD](docs/articles/cure_for_web_development.md)]

- Rust on the Frontend and Backend [[Always Bet on Rust](https://blog.abor.dev/p/moonzoon)]

### Dev News

1. CLI, Build pipeline, Live-reload, HTTPS [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-1-cli-build-pipeline-live-reload-https-1ba6) / [MD](docs/articles/dev_news_1.md)]

1. Live demo, Zoon, Examples, Architectures [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-2-live-demo-zoon-examples-architectures-2oem) / [MD](docs/articles/dev_news_2.md)]

1. Signals, React-like Hooks, Optimizations [[dev.to](https://dev.to/martinkavik/moonzoon-dev-news-3-signals-react-like-hooks-optimizations-39lp) / [MD](docs/articles/dev_news_3.md)]

1. Actix, Async CLI, Error handling, Fails [[dev.to (WIP)](https://dev.to/martinkavik/moonzoon-dev-news-4-actix-async-cli-error-handling-fails-xxxx) / [MD](docs/articles/dev_news_4.md)]

---

# Documentation

### 1. [Philosophy & Non-Goals](docs/philosophy_and_non_goals.md).md

### 2. [Frontend](docs/frontend.md).md

### 3. [Backend](docs/backend.md).md

### 4. [CLI](docs/cli.md).md

### 5. [Cloud](docs/cloud.md).md

### 6. [Development](docs/development.md).md

---

# Size & Speed

![Benchmark example size](docs/images/demo_size_small.png)

[![Benchmark example speed](docs/images/frontend_frameworks_benchmark_1.png)](https://raw.githubusercontent.com/MoonZoon/MoonZoon/main/docs/images/frontend_frameworks_benchmark_screen.png)

- [MartinKavik/js-framework-benchmark](https://github.com/MartinKavik/js-framework-benchmark/tree/framework/moonzoon)

---

# Examples

- [Examples](examples) in the repo
- [Raytracer on MoonZoon](https://github.com/MartinKavik/ray_tracer)

---

# FAQ

1. _"Is it production-ready?"_
   - No, it's in the early development phase now, but you can subscribe to `#news` channel on our [Discord server](https://discord.gg/eGduTxK2Es) to don't miss the announcement. 
   - MoonZoon will be battle-tested during the MoonZoon Cloud, [OpenHope](http://openhope.net) and other projects development.

1. _"Why another framework? Are you mad??"_
   - I want to write products. Reliable products. Quickly. I want to enjoy the development. I don't want to play endlessly with tools, protocols and config files.

1. _"Why Rust?"_
   - It's the best language. 
        <details>
        <summary>Longer explanation</summary>

        I've written commercial or hobby projects in multiple languages (Js, CoffeeScript, TS, Elm, Elixir, PHP, C, C++, C#, Go, ..). However I want to write only in Rust. 

        Rust is hard to learn even for experienced developers, because they have to unlearn many things and adapt thought process to Rust concepts and best practices. However once you stop fighting the compiler, Rust takes your hand and push you to correct and efficient solutions. 

        I had similar feeling when I was learning to drive a car - it seems pretty hard/strange from the start but once you get used to it, you know that each control / button / pedal has it's specific place and purpose for a good reason. And it makes even more sense once you learn low-level stuff - e.g. how the transmission and a clutch work.

        However steep learning curve isn't bad: 
        - It means that Rust doesn't hide real complexity behind too simple models.
        - It's almost impossible for complete beginners to publish incomplete/buggy libraries. 
        
        _

        Rust is designed so well that I feel nervous while I'm writing in other languages - I have to do compiler's work again in my head and think about weird things like typos in code, `null`s, `undefined`s, memory leaks, accidental mutations, how to write fast code without mutability, etc. It generates significant cognitive load so I can't focus so much on business logic and other important stuff.

        I don't believe that you should use the most suitable language for a specific domain or problem at all costs. I think consistency among your / company projects, productivity and simplicity should have the highest priority. And Rust is a very universal language so I think it's a good choice for almost all cases.

        There are also things that should be improved (and are improving):
        1. Compilation is still slow, but it's not so frustrating now.
        1. IDE support still isn't very good because of Rust complex types and macros but thanks to [Rust Analyzer](https://rust-analyzer.github.io/) it's getting better every day.
        1. `target` folder (it's something like `node_modules`) can be pretty big.

        </details>

1. _"The API looks weird!"_
   - I would like to make it compilable on the stable Rust so I can't use some unstable features that would make the API a bit better. 
   - Or I wasn't able to find a simpler and nicer API - please let me know why and how do you want to improve it. 
   - Or we have just different experience and feel for graphic stuff.

1. _"Who is developing it?"_
   - Martin Kavík (a [Seed](https://seed-rs.org/) maintainer, Rust freelance developer) with the help of the awesome community.
   - [An interview with Martin Kavík](https://blog.abor.dev/p/moonzoon)

1. _"Could I help somehow? / Where can I find more information?_"
    - Join our [Discord chat](https://discord.gg/eGduTxK2Es) and don't hesitate to ask any questions or present your ideas.
    - Create a pull-request if you want to fix typos, dead links, weird Czech-English sentences, etc.
    - If you think MoonZoon will be useful for your project, I want to know that! (Use [chat](https://discord.gg/eGduTxK2Es) or [martin@moonzoon.rs](mailto:martin@moonzoon.rs)).
    - Don't hesitate to tell your friends about MoonZoon and feel free to share the link ([http://moonzoon.rs](http://moonzoon.rs)) on social platforms / forums / blogs / newsletters. 

1. _"My only concern is the “no SQL” comment. Will it be possible to use MZ with something like SQLx if I prefer?_" (by [@duspom](https://twitter.com/duspom/status/1362934142770450433))
    - From the [Philosophy & Non-Goals](docs/philosophy_and_non_goals.md) section: "E) Don't build artificial barriers for MoonZoon users - if they want to use REST, CSS or SQL, don't try to stop them."
    - You don't have to use built-in persistent variables in actors. Or you can use them and query the persistent store (e.g. Postgre) directly.
---

Thank you for reading! We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).
