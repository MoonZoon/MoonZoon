# MoonZoon Dev News (3): Simpler API, Optimizations

It's small.

![Benchmark example size](images/demo_size_small.png)

It's fast.

[![Benchmark example speed](images/frontend_frameworks_benchmark_1.png)](https://raw.githubusercontent.com/MoonZoon/MoonZoon/main/docs/articles/images/frontend_frameworks_benchmark_screen.png)

What is it?

Rewritten Zoon!

You can try it by yourself: [Live demo](https://moonzoon-demo.herokuapp.com/)

---

<p align="center">
  <strong>Welcome to the MoonZoon Dev News!</strong>
</p>

<p align="center">
  <img src="images/MoonZoon.png" width="200" title="MoonZoon logo">
</p>


[MoonZoon](https://moonzoon.rs) is a [Rust](https://www.rust-lang.org/) full-stack framework. If you want to read about new MZ features, architecture and interesting problems & solutions - Dev News is the right place.

---

# News

1. Zoon API almost doesn't use macros, it's safer, more expressive and compiler-friendly.
    - Working [examples](https://github.com/MoonZoon/MoonZoon/tree/main/examples) rewritten with the new API: `counter`, `counters`, `js-framework-benchmark`.
    - [Docs](https://github.com/MoonZoon/MoonZoon#documentation) updated to reflect changes.
    - The trade-off is slightly increased verbosity for `Element` APIs.

1. A new article [_"Rust on the Frontend and Backend"_](https://blog.abor.dev/p/moonzoon) on the blog _Always Bet On Rust_
   - _"An interview with Martin Kavík, creator of the MoonZoon full-stack framework"_

1. The [demo project](https://github.com/MoonZoon/demo) and [heroku-buildpack](https://github.com/MoonZoon/heroku-buildpack-moonzoon) updated. You can use them as a starting point for your experimental MoonZoon apps.

1. The MoonZoon [benchmark](https://github.com/MartinKavik/js-framework-benchmark/tree/framework/moonzoon) is ready to be merged (at the time of writing) to [krausest/js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), however I want to wait until MoonZoon is more mature. 

1. You don't have to be afraid to look at `zoon` and `static_ref_macro` [crates](https://github.com/MoonZoon/MoonZoon/tree/main/crates) code. 
   - They are clean enough thanks to awesome libraries [once_cell](https://crates.io/crates/once_cell), [futures-signals](https://crates.io/crates/futures-signals) and [dominator](https://crates.io/crates/dominator).

1. Some new APIs, configs, features and Brotli / Gzip compression integrated.

---

I would like to thank:
- [Pauan](https://github.com/Pauan) for lightning fast resolving of my problems with his libs `futures-signals` and `dominator`.
- [Alexhuszagh](https://github.com/Alexhuszagh) for working on [lexical](https://crates.io/crates/lexical) and answering my [questions](https://github.com/Alexhuszagh/rust-lexical/issues/34#issuecomment-832250773).

---

# X

---

And that's all for today! 
Thank You for reading and I hope you are looking forward to the next episode.

Martin

P.S.
We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).

