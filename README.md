<p align="center">
  <img src="branding/MoonZoon_logo_readme.png" height="300" title="MoonZoon logo">
</p>

### [Discord](https://discord.gg/eGduTxK2Es) | [moonzoon.rs](http://moonzoon.rs)* | [martin@moonzoon.rs](mailto:martin@moonzoon.rs)

\* redirects to this repo until ready

---

### _"We don't want to lose time and money due to millions of unnecessary micro-decisions and bikeshedding."_

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

# Documentation

### 1. [Philosophy & Non-Goals](docs/philosophy_and_non_goals.md).md

### 2. [Frontend](docs/frontend.md).md

### 3. [Backend](docs/backend.md).md

### 4. [CLI](docs/cli.md).md

---

# FAQ

1. _"Is it production-ready?"_
   - No, it's in the design phase now, but you can subscribe to `#news` channel on our [Discord server](https://discord.gg/eGduTxK2Es) to don't miss the announcement. 
   - MoonZoon will be battle-tested during the [OpenHope](http://openhope.net) development.

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

1. _"Could I help somehow? / Where can I find more information?_"
    - Join our [Discord chat](https://discord.gg/eGduTxK2Es) and don't hesitate to ask any questions or present your ideas.
    - Create a pull-request if you want to fix typos, dead links, weird Czech-English sentences, etc.
    - If you think MoonZoon will be useful for your project, I want to know that! (Use [chat](https://discord.gg/eGduTxK2Es) or [martin@moonzoon.rs](mailto:martin@moonzoon.rs)).
    - Don't hesitate to tell your friends about MoonZoon and feel free to share the link ([http://moonzoon.rs](http://moonzoon.rs)) on social platforms / forums / blogs / newsletters. 

