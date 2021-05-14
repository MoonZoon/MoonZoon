# MoonZoon Dev News (3): Signals, React-like Hooks, Optimizations

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

# Chapters
- [News](#news)
- [Old Zoon architecture](#old-zoon-architecture) - How React and Rust Hooks work
- [New Zoon architecture](#new-zoon-architecture) - Signals: You can do it without a Virtual DOM
- [Builder pattern with rules](#builder-pattern-with-rules) - Yes, builder pattern can support required parameters
- [Optimizations](#optimization) - Need for speed. The size matters.

---

# News

1. Zoon API almost doesn't use macros, it's safer, more expressive and compiler-friendly.
    - Working [examples](https://github.com/MoonZoon/MoonZoon/tree/main/examples) rewritten with the new API: `counter`, `counters`, `js-framework-benchmark`.
    - [Docs](https://github.com/MoonZoon/MoonZoon#documentation) updated to reflect changes.
    - The trade-off is slightly increased verbosity for `Element` APIs.

1. A new article [_"Rust on the Frontend and Backend"_](https://blog.abor.dev/p/moonzoon) on the blog _Always Bet On Rust_.
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
- [flosse](https://github.com/flosse) for [fighting](https://github.com/MoonZoon/MoonZoon/pull/6) with Warp in Moon.

---

This blog post is a bit longer but I hope you'll enjoy it!

---

# Old Zoon architecture
> How React and Rust Hooks work

First, I would like to write this sentence to sound clever: _"The old architecture was based on topologically-aware functions with stable call graph identifiers and local states stored in a heterogenous vector."_

Unfortunately, it's not my original idea. It powers [React Hooks](https://reactjs.org/docs/hooks-intro.html). Or [moxie](https://moxie.rs/). Or [Crochet](https://github.com/raphlinus/crochet).

And now the explanation what is it and why it doesn't work well enough to stay in Zoon (just like all over-engineered stuff).

--

So let's say we have `main` and 3 simple functions:

```rust
fn main() {
    loop {
        amber()
    }
}

fn amber() {
    mike()
}

fn mike() {
    layla_rose();
    layla_rose()
}

fn layla_rose() { }
```

Let's add a counter with some helpers and `println`s:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn call_id() -> usize { COUNTER.load(Ordering::SeqCst) }
fn increment_call_id() { COUNTER.fetch_add(1, Ordering::SeqCst); }
fn reset_call_id() { COUNTER.store(0, Ordering::SeqCst) }

fn main() {
    for _ in 0..3 { 
        amber();
        reset_call_id()
    }
}

fn amber() {
    increment_call_id();
    println!("amber id: {}", call_id());
    mike()
}

fn mike() {
    increment_call_id();
    println!("mike id: {}", call_id());
    layla_rose();
    layla_rose()
}

fn layla_rose() {
    increment_call_id();
    println!("layla_rose id: {}", call_id());
}
```

When you run the code ([Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=542bca0402771da40176bdc373d5bc21)), you should see the loop of the sequence:
```
amber id: 1
mike id: 2
layla_rose id: 3
layla_rose id: 4
```

Now you can apply some "magic" to our functions with [proc macros](https://github.com/anp/moxie/blob/33c84788885322895a3f26ce1f5a70a5dbe2d237/topo/macro/src/lib.rs) and/or closures to hide unnecessary counter helpers. The result will look like:

```rust
fn main() {
    for _ in 0..3 { run(amber) }
}

#[i_am_special]
fn amber() {
    println!("amber id: {}", call_id());
    mike()
}

#[i_am_special]
fn mike() {
    println!("mike id: {}", call_id());
    layla_rose();
    layla_rose()
}

#[i_am_special]
fn layla_rose() {
    println!("layla_rose id: {}", call_id());
}
```

But let's get back to our non-macro example and improve it by adding `STATES` and the _hook_ `use_age`. ([Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3bbcdab68607c80fcd340f3ad3e39744))
   - _Note:_ The code below may look a bit scary but you don't have to understand all implementation details.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
static COUNTER: AtomicUsize = AtomicUsize::new(0);
fn call_id() -> usize { COUNTER.load(Ordering::SeqCst) }
fn increment_call_id() { COUNTER.fetch_add(1, Ordering::SeqCst); }
fn reset_call_id() { COUNTER.store(0, Ordering::SeqCst) }

use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
static STATES: Lazy<Mutex<HashMap<usize, u8>>> = Lazy::new(Mutex::default);

fn use_age(default_value: impl FnOnce() -> u8 + Copy) -> u8 {
    *STATES.lock().unwrap().entry(call_id()).or_insert_with(default_value)
}

fn main() {
    for _ in 0..3 { 
        amber(32);
        println!("{:-<28}", "-");
        reset_call_id()
    }
}

fn amber(age: u8) {
    increment_call_id();
    let age = use_age(|| { println!("Saving amber's state!"); age });
    println!("amber id: {}, age: {}", call_id(), age);
    mike(15)
}

fn mike(age: u8) {
    increment_call_id();
    let age = use_age(|| { println!("Saving  mike's state!"); age });
    println!("mike id: {}, age: {}", call_id(), age);
    layla_rose(26);
    layla_rose(22)
}

fn layla_rose(age: u8) {
    increment_call_id();
    let age = use_age(|| { println!("Saving layla_rose's state!"); age });
    println!("layla_rose id: {}, age: {}", call_id(), age);
}
```

The output:
```
Saving amber's state!
amber id: 1, age: 32
Saving  mike's state!
mike id: 2, age: 15
Saving layla_rose's state!
layla_rose id: 3, age: 26
Saving layla_rose's state!
layla_rose id: 4, age: 22
----------------------------
amber id: 1, age: 32
mike id: 2, age: 15
layla_rose id: 3, age: 26
layla_rose id: 4, age: 22
----------------------------
amber id: 1, age: 32
mike id: 2, age: 15
layla_rose id: 3, age: 26
layla_rose id: 4, age: 22
----------------------------
```

The main fact: Closures passed to the `use_age` hook are invoked only once. `use_age` invokes them only if it doesn't find the age from the previous iteration in `STATES`.

Another important fact: `STATES` is a key-value storage, where the key is _call_id_ and the value is `u8` (aka _age_).

So.. do we have nice React Hooks and the world is smiling? 

Yeah, until a wild condition appears...

```rust
mike(30)
if day == "good_day" {
    layla_rose(26)
} else {
    amber(60)
}
```

The first iteration with a `good_day`:
```rust
mike(30)   // call id == 1  ;  age 30 saved
if day == "good_day" {
    layla_rose(26)   // call id == 2 ; age 26 saved
} else {
    amber(60)    // not called
}
```

The next iteration with a `bad_day`:
```rust
mike(30)   // call id == 1  ; age 30 loaded
if day == "good_day" {
    layla_rose(26)   // not called
} else {
    amber(60)    // call id == 2  ; age 26 loaded
}
```

Output ([Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=d3f2e3a7cb1024730f88fbe1e73617ae)):

```
Saving mike's state!
mike id: 1, age: 30
Saving layla_rose's state!
layla_rose id: 2, age: 26
----------------------------
mike id: 1, age: 30
amber id: 2, age: 26
----------------------------
mike id: 1, age: 30
layla_rose id: 2, age: 26
----------------------------
```

You would be really surprised if a Tinder developer accidentally wrapped a React component in a condition and you plan a date with Amber...

That's why there are official [Rules of Hooks](https://reactjs.org/docs/hooks-rules.html):

> - Don’t call Hooks inside loops, conditions, or nested functions.
> - Don’t call Hooks from regular JavaScript functions.

Our call ids based on a counter / indices are just not _stable_ enough.

Fortunately, Rust offers more tools to fight with hooks limitations.

We can get the [Location](https://doc.rust-lang.org/std/panic/struct.Location.html) of the caller. It means we know where exactly in the source code has been a function called. So we can distinguish different calls by their caller, even if their index is equal. 

We can leverage newer Rust built-in attribute [#[track_caller]](https://rust-lang.github.io/rfcs/2091-inline-semantic.html) in combination with [Location::caller](https://doc.rust-lang.org/stable/std/panic/struct.Location.html#method.caller). The code is starting to be pretty complex ([Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=474dfd8daa9078467fc77016142b036a)).

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::panic::Location;
static COUNTER: AtomicUsize = AtomicUsize::new(0);
#[track_caller]
fn call_id() -> (usize, &'static Location<'static>) { 
    (COUNTER.load(Ordering::SeqCst), Location::caller()) 
}
fn increment_call_id() { COUNTER.fetch_add(1, Ordering::SeqCst); }
fn reset_call_id() { COUNTER.store(0, Ordering::SeqCst) }

use std::{sync::Mutex, collections::HashMap};
use once_cell::sync::Lazy;
static STATES: Lazy<Mutex<HashMap<(usize, &'static Location), u8>>> = Lazy::new(Mutex::default);

#[track_caller]
fn use_age(default_value: impl FnOnce() -> u8 + Copy) -> u8 {
    *STATES.lock().unwrap().entry(call_id()).or_insert_with(default_value)
}

fn main() {
    for i in 0..3 { 
        root(if i % 2 == 0 { "good_day" } else { "bad_day" });
        println!("{:-<28}", "-");
        reset_call_id()
    }
}

fn root(day: &str) {
    mike(30);
    if day == "good_day" {
        layla_rose(26)
    } else {
        amber(60)
    }
}

#[track_caller]
fn mike(age: u8) {
    increment_call_id();
    let age = use_age(|| { println!("Saving mike's state!"); age });
    println!("mike id: {:?}, age: {}", call_id(), age);
}

#[track_caller]
fn amber(age: u8) {
    increment_call_id();
    let age = use_age(|| { println!("Saving amber's state!"); age });
    println!("amber id: {:?}, age: {}", call_id(), age);
}

#[track_caller]
fn layla_rose(age: u8) {
    increment_call_id();
    let age = use_age(|| { println!("Saving layla_rose's state!"); age });
    println!("layla_rose id: {:?}, age: {}", call_id(), age);
}
```

Updated output (notice Amber's age and `Saving amber's state!`):
```
Saving mike's state!
mike id: (1, Location { file: "src/main.rs", line: 29, col: 5 }), age: 30
Saving layla_rose's state!
layla_rose id: (2, Location { file: "src/main.rs", line: 31, col: 9 }), age: 26
----------------------------
mike id: (1, Location { file: "src/main.rs", line: 29, col: 5 }), age: 30
Saving amber's state!
amber id: (2, Location { file: "src/main.rs", line: 33, col: 9 }), age: 60
----------------------------
mike id: (1, Location { file: "src/main.rs", line: 29, col: 5 }), age: 30
layla_rose id: (2, Location { file: "src/main.rs", line: 31, col: 9 }), age: 26
----------------------------
```

To make the code more robust we'll need to track also ancestors. Otherwise we may have calls with equal index and direct callers but they are actually different because they have different callers of callers... So.. we need to create a simple _blockchain_ where each call has a hash of the previous call (yeah, another buzzword for SEO..)

However Nemesis for all Javascript and Rust Hooks are loops. Different calls in loops may have equal both index and location. It means we need another factor to correctly distinguish calls - `keys`. Unfortunately they need to be provided by developer because they depend on application data.

Many frameworks (with or without Hooks) support `keys`:
- [React Keys](https://reactjs.org/docs/reconciliation.html#keys)
- [Elm Html.Keyed](https://guide.elm-lang.org/optimization/keyed.html)
- [moxie/topo slots](https://docs.rs/topo/0.13.2/topo/attr.nested.html#slots)
- [svelte keyed each blocks](https://svelte.dev/tutorial/keyed-each-blocks)

When the developer forgets to define keys, the app may be slower or doesn't work as expected (look at the Svelte demonstration above).

So... from the frontend app developer point of view, Hooks (especially Rust ones) may be a useful tool to reduce boilerplate and introduce local state for function-based components, but the developer has to follow some artificial rules.

--

Let's move to the technical challenges of Hooks.

1. Complexity. It's pretty hard to implement Hooks correctly, especially due to many edge-cases and macros. It also mean a lot of code bloat if you are not careful enough.

1. Good luck with Hooks integration into the framework with asynchronous rendering - you may get lost in the Dark caller forest (just a note from MoonZoon trenches).

1. We were working only with the _age_ in our examples. Hooks have to support as many types as possible (not only `u8`). It means we need a heterogenous storage for user data, probably based on [Any](https://doc.rust-lang.org/std/any/trait.Any.html) (if you know Typescript [any](https://www.typescriptlang.org/docs/handbook/2/everyday-types.html#any), Rust `Any` may open similar gates of hell).

1. It's pretty hard to start a new iteration in a non-root "node" (when you want to invoke only the function in the call graph representing a component with changed data).

1. However the Hooks Achilles' heel is the storage performance. When you decide to use the simplest solution - a `HashMap` with `Box<dyn Any>` as values:

   - `Box` and `Any` means a lot of type gymnastics and checks and allocations.

   - `HashMap`'s default hash function isn't the fastest one, but the replacement with a faster non-secure one didn't help to increase speed in practice.

   - `HashMap` resizing is pretty slow - it has to move all its items to the new location after the reallocation. [griddle](https://crates.io/crates/griddle) helps to eliminate resizing spikes, but it doesn't help too much with the overall speed.

As the result, Zoon's code was a slow spaghetti monster. It was working good enough for cca 2_000 elements, but when  there were more complex business logic and more elements then the app becomes too slow for comfortable usage.

I've also tried more mature libraries instead of my code but the performance didn't change too much. 

Then I remembered the term _Sunk cost fallacy_ from the awesome book [Thinking, Fast and Slow](https://en.wikipedia.org/wiki/Thinking,_Fast_and_Slow) and with the words "Don't love your code, no code no bugs" I selected most Zoon files and hit my favorite key: `Delete`.

---

# New Zoon architecture
> Signals: You can do it without a Virtual DOM

So Hooks was a dead end. The Elm architecture has its own problems (explained in the [previous post](https://dev.to/martinkavik/moonzoon-dev-news-2-live-demo-zoon-examples-architectures-2oem#frontend-framework-architectures)). I don't want to invent another complex component system with templates. What now?

Let's learn from the past and see what works and what doesn't.

- Hooks - Simple creation of local states helps to write element/component libraries and don't pollute our business data with GUI-specific variables.

- TEA - Single-source of truth (aka `Model`) eliminates bugs related to state synchronization.

- TEA - Asynchronous "pipelines" may be hard to follow in the source code without an `await/async` mechanism. Imagine a chain of HTTP requests with error handling and some business logic.

- Many frameworks / GUI libraries often try to store and manage all objects representing elements/components by themselves and use the target platform only as a "canvas" where they render elements.
   
   - Why write a custom DOM when we still need to use the browser DOM? The custom DOM then basically becomes a cache. And what are the [most difficult things](https://martinfowler.com/bliki/TwoHardThings.html) in computer science?
   
   - Why to store and manage objects when we only want to render a HTML string for a Google bot?

- Passing properties down to child elements/components may lead to boilerplate (TEA) and then to cumbersome abstractions (many frameworks). TEA-like frameworks try to mitigate it with [Pub/Sub](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern) mechanisms.

- There are often problems with _keys_ for element/component lists (explained in the previous chapter).

- Virtual DOM + Asynchronous rendering (the render waits for the next [animation frame](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)) 
   - Adds a lot of complexity and causes bugs. 
   - The typical bug in most frameworks is a "jumping cursor" in text inputs ([Elm issue with demonstation](https://github.com/elm/virtual-dom/issues/138), [React explanation](https://stackoverflow.com/questions/28922275/in-reactjs-why-does-setstate-behave-differently-when-called-synchronously/28922465#28922465)).
   - Text selection is [pretty hard to manage](https://github.com/seed-rs/seed/blob/master/src/browser/util.rs#L161-L260) in the browser, especially with async rendering.

- Many native browser elements behave quite unpredictably and it's very hard to set them correctly. There has to be a layer above them to protect the app developer.
   - _"Did you know #456: [Setting element attributes is order-sensitive](https://github.com/seed-rs/seed/issues/335)?"_

Now I'll show you 4 examples with a new Zoon API and explain how they work. Then we'll discuss how the API corresponds with the notes above.

## Example 1

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
    // We want to attach our app to the browser element with id "app".
    // Note: `start_app(None, root);` would attach to `body` but it isn't recommended.
    start_app("app", root);
}
```

The function `counter()` is marked by the attribute `#[static_ref]`. It means the function is transformed by a procedural macro into this:

```rust
fn counter() -> &'static Mutable<i32> {
    use once_cell::race::OnceBox;
    static INSTANCE: OnceBox<Mutable<i32>> = OnceBox::new();
    INSTANCE.get_or_init(move || Box::new(Mutable::new(0)))
}
```
- The macro is defined in the crate `static_ref_macro` in the [MoonZoon repo](https://github.com/MoonZoon/MoonZoon/tree/main/crates).
- The macro currently uses `OnceBox`. It may use `OnceCell` or probably `lazy_static!`.
- You can deactivate the macro by a Zoon feature flag `static_ref`.

`Mutable` is very similar to [RwLock](https://doc.rust-lang.org/std/sync/struct.RwLock.html). However it has one unique feature - it sends a _signal_ on change. Let's explain it on the `Text` element.

There are multiple ways to create a new `Text` element: 
```rust
.item(counter().get())
.item(Text::new(counter().get())
.item(Text::with_signal(counter().signal()))
```
- _Note_: The method `.item` expects the `impl IntoElement` parameter. Many Rust basic types (`&str`, `Cow<str>`, `i32`, ..) implement `IntoElement` by creating a new `Text`.

The first two lines are practically the same. They just creates a `Text` element with a _static_ value. It means the text doesn't change at all once set. We can only replace the `Text` element with a new one if we want to change it.  

The third line is more interesting. `Text` created with the method `with_signal` rerenders its text when it receives a new value from the chosen _signal_. `Mutable` transmits its value to all associated signals when the value has been changed. We can say that `Text` created by `with_signal` has a _dynamic_ value.

--

## Example 2

```rust
use zoon::*;
use std::rc::Rc;

fn root() -> impl Element {
    let counter = Rc::new(Mutable::new(0));
    let on_press = clone!((counter) move |step: i32| *counter.lock_mut() += step);
    Column::new()
        .item(Button::new().label("-").on_press(clone!((on_press) move || on_press(-1))))
        .item_signal(counter.signal())
        .item(Button::new().label("+").on_press(move || on_press(1)))
}
```

This example works exactly like the previous one but there are some differences in the code.

1. `counter` isn't stored in a static reference / global variable, but created as a local variable. 
   
   - Soo... where is it stored?? In the browser DOM! `Button::new` creates immediately a new DOM node and our `counter` is passed into its `on_press` handler. It's possible because the `root` function is invoked only once to build the app / create DOM.

1. `counter`'s `Mutable` is wrapped in `Rc`.
   - We need to pass the same `counter` into two `on_press` handlers. Otherwise it wouldn't be necessary.

1. There is a `clone!` macro.
   - Yeah, it's just an alias for `enc!` macro in the [enclose](https://crates.io/crates/enclose) crate. I hope Rust will support cloning into closures natively.
   - The `clone!` macro is active when the Zoon's feature flag `clone` is enabled.

1. `counter().update(|counter| counter - 1)` has been replaced with `*counter.lock_mut() += step`.
   
   - You probably wouldn't find the method `update` in `futures-signals` docs - there are traits like `MutableExt` in the Zoon with such helpers.

   - Be careful with `lock_*` methods. There are cases where it's a bit hard to predict in Rust when the lock is unlocked / dropped (you'll find an example in the next chapter). Also `futures-signals` crate currently uses `std::sync::RwLock` under the hood that doesn't output a nice error message to the console (especially in Firefox) so it may be hard to track the problem of trying to lock already locked `Mutable`. (I was talking about it with the `futures-signals` author, it should be less confusing in the future.)

--

## Example 3

```rust
...
type ID = usize;
struct Row {
    id: ID,
    label: Mutable<String>,
}

#[static_ref]
fn rows() -> &'static MutableVec<Arc<Row>> {
    MutableVec::new()
}

fn remove_row(id: ID) {
    rows().lock_mut().retain(|row| row.id != id);
}
...

fn table() -> RawEl {
    ...
    RawEl::new("tbody")
        .attr("id", "tbody")
        .children_signal_vec(
            rows().signal_vec_cloned().map(row)
        )
    ...
}

fn row(row: Arc<Row>) -> RawEl {
    let id = row.id;
    ...
    row_remove_button(id),
    ...
}

fn row_remove_button(id: ID) -> RawEl {
    ...
    RawEl::new("a")
        .event_handler(move |_: events::Click| remove_row(id))
    ...
}

```

The most interesting are these two parts:
```rust
// from `table()`
.children_signal_vec(
    rows().signal_vec_cloned().map(row)
)
// from `remove_row(id: Id)`
rows().lock_mut().retain(|row| row.id != id)
```

`RawEl::children_signal_vec` updates its child elements according to the input signal. The signal comes from a `MutableVec` returned from `rows()`. The most important fact is that this signal transmits only differences between the old and the updated vector. It means it's fast because it doesn't have to clone the entire vector on every change and it can transmit only the child's index in the case of removing. 

_Note:_ `RawEl` is a "low-level element". It means `RawEl` is used as a foundation for other Zoon elements like `Row` and `Button`. Only the element `Text` is based on `RawText`. Both `RawEl` and `RawText` implement `Element` and `From for RawElement`. There will be probably also a `RawSvgEl` in the future. The idea is all _raw elements_ can write directly to the browser DOM or to `String` as needed.

--

## Example 4

```rust
// ----- app.rs -----

// ------ ------
//    Statics 
// ------ ------

#[static_ref]
fn columns() -> &'static MutableVec<()> {
    MutableVec::new_with_values(vec![(); 5])
}

#[static_ref]
fn rows() -> &'static MutableVec<()> {
    MutableVec::new_with_values(vec![(); 5])
}

// ------ ------
//    Signals 
// ------ ------

fn column_count() -> impl Signal<Item = usize> {
    columns().signal_vec().len()
}

fn row_count() -> impl Signal<Item = usize> {
    rows().signal_vec().len()
}

pub fn counter_count() -> impl Signal<Item = usize> {
    map_ref!{
        let column_count = column_count(),
        let row_count = row_count() =>
        column_count * row_count
    }
}

// ----- app/view.rs -----

fn counter_count() -> impl Element {
    El::new()
        .child_signal(super::counter_count().map(|count| format!("Counters: {}", count)))
}

```

This example demonstrates how to combine multiple signals into one.

For more info about _signals_, _mutables_, `map_ref` and other entities I recommend to read the excellent [tutorial](https://docs.rs/futures-signals/0.3.20/futures_signals/tutorial/index.html) in the `futures-signals` crate.

_Note:_ If you remember the old Zoon API: `Statics` replace `SVars` ; `Signals` replace `Caches`.    

--

You've seen all examples, let's revisit our notes:

- Hooks - Simple creation of local states helps to write element/component libraries and don't pollute our business data with GUI-specific variables.
   - `let counter = Rc::new(Mutable::new(0));` or an equivalent without `Rc` seems to be a good way to create a local state.

- TEA - Single-source of truth (aka `Model`) eliminates bugs related to state synchronization.
   - static refs or Rust atomics and "update functions" (like `increment` in our counter example) should be a good alternative to `Model` + `update`. 

- TEA - Asynchronous "pipelines" may be hard to follow in the source code without an `await/async` mechanism. Imagine a chain of HTTP requests with error handling and some business logic.
   - `futures-signals` is based, well, on `futures`. You can write (according to the official docs) `my_state.map_future(|value| do_some_async_calculation(value));`. You can create also `Streams` and much more.

- Many frameworks / GUI libraries often try to store and manage all objects representing elements/components by themselves and use the target platform only as a "canvas" where they render elements.
   - _Raw elements_ writes directly to the browser DOM and stores the state inside it. They'll be able to write also to `String` in the future.

- Passing properties down to child elements/components may lead to boilerplate (TEA) and then to cumbersome abstractions (many frameworks). TEA-like frameworks try to mitigate it with [Pub/Sub](https://en.wikipedia.org/wiki/Publish%E2%80%93subscribe_pattern) mechanisms.
   - You'll be able to combine _static refs_, _signals_, standard Rust constructs and maybe Zoon's _channels_ to eliminate the boilerplate.  

- There are often problems with _keys_ for element/component lists (explained in the previous chapter).
   - Do you remember `RawEl::children_signal_vec` from the _Example 3_? No keys - no problems.

- Virtual DOM + Asynchronous rendering (the render waits for the next [animation frame](https://developer.mozilla.org/en-US/docs/Web/API/window/requestAnimationFrame)) 
   - No VDOM, no async rendering - no problems. However I can imagine Zoon will need to support async rendering, but ideally it would be used only when the app developer creates animations.

- Many native browser elements behave quite unpredictably and it's very hard to set them correctly. There has to be a layer above them to protect the app developer.
   - Two layers should shield the app developer. Standard Zoon elements (`Button`, `Row`..) is the first layer and raw elements (`RawEl`, `RawText`) is the second one.

---

# Builder pattern with rules
> Yes, builder pattern can support required parameters

```rust
Button::new().label("X").label("Y")
El::new().child("X").child("Y")
```
- Will the button be labeled "Y" or "XY"?
- Will the el's children be rendered in a row or in a column?

```
error[E0277]: the trait bound `LabelFlagSet: FlagNotSet` is not satisfied
  --> frontend\src\lib.rs:17:30
   |
17 |     Button::new().label("X").label("Y");
   |                              ^^^^^ the trait `FlagNotSet` is not implemented for `LabelFlagSet`

error[E0277]: the trait bound `ChildFlagSet: FlagNotSet` is not satisfied
  --> frontend\src\lib.rs:18:26
   |
18 |     El::new().child("X").child("Y");
   |                          ^^^^^ the trait `FlagNotSet` is not implemented for `ChildFlagSet`
```

The Rust compiler doesn't allow us to write the code that would break `Button` or `El` _rules_. Only one label and one child makes sense. 

The compilation also fails when you don't set the label or child at all:

```rust
fn root() -> impl Element {
    El::new()
}
```

```
error[E0277]: the trait bound `zoon::El<ChildFlagNotSet>: zoon::Element` is not satisfied
  --> frontend\src\lib.rs:16:14
   |
16 | fn root() -> impl Element {
   |              ^^^^^^^^^^^^ the trait `zoon::Element` is not implemented for `zoon::El<ChildFlagNotSet>`
   |
   = help: the following implementations were found:
             <zoon::El<ChildFlagSet> as zoon::Element>
```

Yeah, we may have constructors like `El::new(..)` instead. But then we also need at least `El::with_child_signal(..)`. And other constructors for more complex elements with more required parameters and their combinations. It becomes cumbersome very quickly.

_Note:_ There are exceptions in the Zoon API like `RawEl::new("div")` and `Text::new("text")` because it's not possible to even create a builder for these types without the most important input data. 

Why we can't just take the last value as the valid one? E.g. `Button::new().label("X").label("Y");` will be a button labeled "Y".
   - All methods (`.label(..)`, `.child(..)`) modifies the DOM immediately. It means we would need to delete the previous label and it would be pretty inefficient.
   - It will be confusing - `El` can have only one child, but `Row` accepts multiple children. 

Why all methods modifies the DOM immediately?
  - I've tried to store element builder arguments in the builder and render it at once later. However this approach leads to slow and cumbersome elements and it's almost impossible in some cases. 

--

How those _rules_ work?

Let's look at the current `Button` implementation.

```rust
use zoon::*;
use std::marker::PhantomData;

// ------ ------
//    Element 
// ------ ------

make_flags!(Label, OnPress);

pub struct Button<LabelFlag, OnPressFlag> {
    raw_el: RawEl,
    flags: PhantomData<(LabelFlag, OnPressFlag)>
}

impl Button<LabelFlagNotSet, OnPressFlagNotSet> {
    pub fn new() -> Self {
        Self {
            raw_el: RawEl::new("div")
                .attr("class", "button")
                .attr("role", "button")
                .attr("tabindex", "0"),
            flags: PhantomData,
        }
    }
}

impl<OnPressFlag> Element for Button<LabelFlagSet, OnPressFlag> {
    fn into_raw_element(self) -> RawElement {
        self.raw_el.into()
    }
}

// ------ ------
//  Attributes 
// ------ ------

impl<'a, LabelFlag, OnPressFlag> Button<LabelFlag, OnPressFlag> {
    pub fn label(
        self, 
        label: impl IntoElement<'a> + 'a
    ) -> Button<LabelFlagSet, OnPressFlag>
        where LabelFlag: FlagNotSet
    {
        Button {
            raw_el: self.raw_el.child(label),
            flags: PhantomData
        }
    }

    pub fn label_signal(
        self, 
        label: impl Signal<Item = impl IntoElement<'a>> + Unpin + 'static
    ) -> Button<LabelFlagSet, OnPressFlag> 
        where LabelFlag: FlagNotSet
    {
        Button {
            raw_el: self.raw_el.child_signal(label),
            flags: PhantomData
        }
    }

    pub fn on_press(
        self, 
        on_press: impl FnOnce() + Clone + 'static
    ) -> Button<LabelFlag, OnPressFlagSet> 
        where OnPressFlag: FlagNotSet
    {
        Button {
            raw_el: self.raw_el.event_handler(move |_: events::Click| (on_press.clone())()),
            flags: PhantomData
        }
    }
} 
```

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

The only purpose of _flags_ is to enforce _rules_ by the Rust compiler.

The compiler doesn't allow to call `label` or `label_signal` if the label is already set. The same rule applies for `on_press` handler.

The trade-off for compile-time checked rules are generics. However it isn't a big problem in practice because in _view_ you often return them from a function by `impl Element`. And when you really need to _box_ them because you want to use them in a collection or in `match` / `if` arms, then you can because `Element` trait is [object safe](https://doc.rust-lang.org/reference/items/traits.html#object-safety) for these purposes.

--

What about API with macros?

```rust
Column::new()
    .item(Button::new().label("-").on_press(decrement))
    .item(Text::with_signal(counter().signal()))
    .item(Button::new().label("+").on_press(increment))
```

vs

```rust
col![
    button![button::on_press(decrement), "-"],
    text![counter().signal()],
    button![button::on_press(increment), "+"],
]
```

### Macro API advantages:
- Less verbosity / boilerplate in most cases.
- Can accept more types than standard functions thanks to "tricks" (e.g. implementing different traits with the same methods for different types) to resolve conflicting `impl`s to achieve a simpler [specialization](https://github.com/rust-lang/rfcs/blob/master/text/1210-impl-specialization.md). _Note:_ You can see this trick in action in Seed's `UpdateEl*` [traits](https://github.com/seed-rs/seed/blob/938d71b6527efcd84c7d9ff37330949791b6d3a4/src/virtual_dom/update_el.rs) that power its element macros.
- It can protect from locks-related problems. See the example below.

```rust
fn root() -> impl Element {
    Column::new()
        .item(*counter().lock_ref())
        .item(Text::with_signal(counter().signal()))
}
```
The lock from `lock_ref` isn't dropped soon enough so the hidden locking in `.signal()` fails in runtime.

We can resolve it manually:
```rust
// By a  closure
.item((|| *counter().lock_ref())())

// By an extra `let` bindings
.item({ let lock = counter().lock_ref(); *lock })
```

_Note:_ I hope Rust compiler will be clever enough to resolve it in the future by itself and also provide a more descriptive error.

### Macro API disadvantages:
- Less compiler friendly (cryptic errors) and less auto-complete / IDE friendly.
- May cause code bloat.
- `Element` implementation is more complicated.
- Didn't pass the "girlfriend test" (A non-developer person with a good graphic taste points the finger to the nicer code when two same examples with different APIs are presented for examination.)
- Hard to learn for beginners.
- Hard to maintain.

---

# Optimizations
> Need for speed. The size matters.

## Speed

The most Rust tutorials and best practices are focused on speed. It means you can just follow general recommendations and pick the most used libraries and there is a chance everything will be fast.

The simplest way to increase speed is to set your `Cargo.toml` correctly. Example: 

```toml
[profile.release]
# Enable link time optimizations - slow compilation, faster & smaller app
lto = true  

# Disable parallel compilation (set to 1 thread) - slow compilation, faster & smaller app
codegen-units = 1  

# Set optimization level - 3 => fast app ; s/z => small app
opt-level = 3 

#  O4 => fast app ; Oz/Os => small app
# [See the explanation below]
[package.metadata.wasm-pack.profile.release]
wasm-opt = ['-O4']

```

MoonZoon CLI (`mzoon`) uses [wasm-pack](https://github.com/rustwasm/wasm-pack) to build your frontend app with Zoon. `wasm-pack` downloads and manages tools like [wasm-bindgen CLI](https://rustwasm.github.io/wasm-bindgen/reference/cli.html) and [wasm-opt](https://github.com/WebAssembly/binaryen#binaryen-optimizations) and browser drivers for testing.

- `wasm-bindgen` CLI and [crate](https://crates.io/crates/wasm-bindgen) do the hard work to connect the Javascript with Rust / Wasm world. `wasm-pack` / `wasm-bindgen CLI` often generates a Javascript file to "boot" your app stored in the Wasm file. `wasm-bindgen` also can generate Typescript types for exported functions from Wasm and JS snippets defined in your Rust code.

- `wasm-opt` is a tool for Wasm file optimizations. It can improve speed, but it's excellent in size reduction. _Note:_ It's written in C++.

`wasm-pack` can be [configured](https://rustwasm.github.io/wasm-pack/book/cargo-toml-configuration.html?highlight=wasm-opt#cargotoml-configuration) in `Cargo.toml`. And it automatically installs also the required [compilation target](https://doc.rust-lang.org/rustc/targets/index.html) `wasm32-unknown-unknown`.

When you run `mzoon start` or `mzoon build`, MZoon checks if the `wasm-pack` is installed on your system (it'll do it automatically in the future) and then runs
```
wasm-pack --log-level warn build frontend --target web --no-typescript --dev
```
to compile your app. _Note_: It doesn't append `--dev` if you run `mzoon start -r`

--

Ok, we have an idea how the Wasm Rust app compilation work and we set the most important options in `Cargo.toml`.

However let's look again at our `Cargo.toml`. I recommend to disable `default-features` and search for feature flags in docs and source code in your dependencies.

Example from `js-framework-benchmark`:

```toml
[dependencies]
zoon = { path = "../../../../crates/zoon", features = ["static_ref", "fmt"], default-features = false }
rand = { version = "0.8.3", features = ["small_rng", "getrandom"], default-features = false }
getrandom = { version = "0.2", features = ["js"], default-features = false }
```

- When you enable only the needed features, you can reduce compilation speed.

- Many creates offer features that optimize the crate and its dependencies for a particular platform (embedded, Wasm), characteristic (speed / size).

- You often need to look into the source code because many feature flags and conditions aren't documented or visible on [docs.rs](https://docs.rs/). Examples:

   - `rand` needs the flag `getrandom` and `getrandom` needs the flag `js` to not fail in runtime in Wasm.

   - `parking_lot` shows `wasm-bindgen` flag in its [docs.rs docs](https://docs.rs/crate/parking_lot/0.11.1/features) but the README says: _"The wasm32-unknown-unknown target is only supported on nightly and requires -C target-feature=+atomics in RUSTFLAGS"_

   - `wasm-bindgen` shows `std` flag in [docs](https://docs.rs/crate/wasm-bindgen/0.2.74/features), but it doesn't work without it. On the other hand, you can enable the feature flag `enable-interning` (will be explained later)

--

Zoon's current features are:

```toml
[features]
default = ["static_ref", "panic_hook", "small_alloc", "clone"]
static_ref = ["static_ref_macro", "once_cell"]
panic_hook = ["console_error_panic_hook"]
small_alloc = ["wee_alloc"]
fast_alloc = []
# tracing_alloc = ["wasm-tracing-allocator"]
clone = ["enclose"]
fmt = ["ufmt", "lexical"]
```

The features related to performance:

### `small_alloc` or `fast_alloc` or `tracing_alloc`

The default allocator for Rust Wasm is currently [dlmalloc](https://github.com/alexcrichton/dlmalloc-rs). You can choose a different allocator (see the [list of allocators](https://lib.rs/search?q=%23allocator)), but only a couple of them are compatible with Wasm.

I haven't found a Wasm-compatible allocator faster then the default `dlmalloc`. So when you enable the flag `fast_alloc`, compilation fails with the message _""Do you know a fast allocator working in Wasm?""_.

The flag `small_alloc` is enabled by default. It means your app will use a bit slower but small [wee_alloc](https://crates.io/crates/wee_alloc).

The flag `tracing_alloc` would switch to the [wasm-tracing-allocator](https://crates.io/crates/wasm-tracing-allocator).

   - _"wasm-tracing-allocator enables you to better debug and analyze memory leaks and invalid frees in an environment where we don't have access to the conventional tools like Valgrind."_
   
   - I tried it, it works but I didn't find it very useful for me. I'll integrate it into Zoon if we find some reasons in the future.

### `fmt`

This feature enables dependencies [ufmt](https://crates.io/crates/ufmt) and [lexical](https://crates.io/crates/lexical). It could replace `std::fmt` machinery (`Debug`, `format!`, `println!`) however I'll probably focus on it in other MoonZoon dev iteration. And you'll see some related notes in the next `Size` section.

--

Now we can finally talk about your application code.

There are some recommendation for Wasm + JS:
- Wrap `&str` in [intern](https://docs.rs/wasm-bindgen/0.2.74/wasm_bindgen/fn.intern.html) where it makes sense. It caches strings in JS to mitigate slow string passing through the Rust-JS "bridge" created by `wasm-bindgen`. Zoon (exactly [dominator](https://crates.io/crates/dominator)) interns automatically many element arguments so you don't need to do it by yourself in most cases.

- Be careful with sending strings and more complex items from and to JS world. It may be slow because of encoding and serialization and it may cause some boilerplate in the app code because it's often needed to tell `wasm-bindgen` how your items should be serialized for export to JS. _Note:_ This problem should be mitigated in the future by richer Wasm API that allows faster Wasm-JS communication.

- Use `unchecked_*` alternatives where it makes sense - see, for instance, [wasm_bindgen::JsCast](https://docs.rs/wasm-bindgen/0.2.74/wasm_bindgen/trait.JsCast.html).

And some general recommendations:

- Reduce memory allocations as much as possible. It means generics instead of `Box`, _arrays_ instead of _vectors_ and other stuff.

- Reduce the number of expensive `.clone`, `.to_owned`, `.to_string`, `.collect`, `.into` .., calls.

- Reduce reallocations. Try to call, for instance, [Vec::with_capacity](https://doc.rust-lang.org/std/vec/struct.Vec.html#method.with_capacity) instead of `Vec::new` / `vec![]` where possible.

- Try to switch algorithms / structures when it makes sense - e.g. `HashMap` vs `IndexMap` vs `HashMap` with a non-secure hash function vs `BTreeMap` vs `SlotMap`, etc. It makes sense only when you've prepared benchmarks - results could be quite surprising. _Tip_: Watch out for `println!` calls in your benchmarks, console operations could be pretty slow. 

- There are libraries like [smallvec](smallvec
) or [im](https://crates.io/crates/im) or [fst](https://crates.io/crates/fst) which helps A LOT if you know how and where to use them.

- Don't use `Rc`, `RefCell`, `Mutex` and similar stuff if you don't have to.

- Create errors or default values lazily where it makes sense - e.g. call `Result::unwrap_or_else` instead of `Result::unwrap_or` or `Option::map_or_else` instead of `Option::map_or`.

--

Recommendations for all web apps:

### Preloading

Moon generates `index.html` very similar to this one:

```html
    <head>
      ...
      <link rel="preload" href="/pkg/frontend_bg_{id}.wasm" as="fetch" type="application/wasm" crossorigin>
      <link rel="modulepreload" href="/pkg/frontend_{id}.js" crossorigin>
      {head_extra} 
    </head>

    <body>
      ...
      <script type="module">
        import init from '/pkg/frontend_{id}.js';
        init('/pkg/frontend_bg_{id}.wasm');
      </script>
    </body>
```

Notice [preload](https://developer.mozilla.org/en-US/docs/Web/HTML/Preloading_content) and [modulepreload](https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types/modulepreload) (don't ask me why there are two distict names, HTML and browser APIs is one big mystery to me).

The browser will be downloading files marked with `preload / moduleprelod` even if it needs to resolve scripts and styles hidden under the placeholder `{head_extra}` before it can move to process `body`.

When the browser starts to import `frontend_{id}.js` or init `frontend_bg_{id}.wasm`, it doesn't have to download these files because they have been already preloaded (or they are loading).

_Note_: There shouldn't be a large time span between preloading and using the files (it may happen when there is a script in `{head_extra}` hosted on a slow server). Otherwise the browser may show a warning or maybe don't match the files at all.

### HTTP/2

There are many reasons why to use HTTP/2 instead of HTTP/1.1. See the basic list of improvements and benchmarks in the article [HTTP/2 vs HTTP/1 - Performance Comparison](https://imagekit.io/blog/http2-vs-http1-performance/). HTTP/2 is also important for MoonZoon because it allows more [SSE](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events) connections than HTTP/1.

When you start your MoonZoon app or a MoonZoon example (see [Development.md](https://github.com/MoonZoon/MoonZoon/blob/main/docs/development.md)), it runs on `HTTP/1.1` by default. You can check it in the browser developer tools, in the tab `Network` when you add the column `Protocol`.

To enable HTTP/2 you have to enable HTTPS. Modify the file `MoonZoon.toml` in your project or MoonZoon example:
```toml
# port = 8080
port = 8443
https = true
...
```
then start/restart the server (`makers mzoon start` for examples) and go to [https://localhost:8443](https://localhost:8443). Accept a potential security risk caused be a self-signed certificate and you should see `HTTP/2` (Firefox) or `h2` (Chrome) in the dev tools. 

--

Aaaand how can we measure performance?

1. Learn to use the browser tools. Chrome dev tools are probably the best - tutorial: [Analyze runtime performance](https://developer.chrome.com/docs/devtools/evaluate-performance/).

1. You can use [web_sys::Performance](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Performance.html) to measure your app logic. See related [MDN docs](https://developer.mozilla.org/en-US/docs/Web/API/Performance).

1. I'm sure you'll be able to find some Rust or Javascript benchmark libraries suitable for Wasm. (Don't hesitate to share your experience.)

_Tips_: 

- Keep in mind that `debug` build is often MUCH slower than the `release` and optimized one, but it contains debug info needed to show function names and other data by profilers / benchmarks.

- There are cases when optimization for size results in higher speed - always test and measure your changes and avoid premature optimization.

## Size

There are 2 things that very likely increase the size A LOT - dependencies and macros.

### Dependencies

Optimized `counter` example (`makers mzoon build -r`):
- Without additional deps - 33 KB (GZip: 16 KB, Brotli: 14 KB, debug: 445 KB)
- With one formatting call `1.2.to_string()` - 52 KB (GZip: 24 KB, Brotli: 21 KB, debug: 468 KB)
- With the `url` crate - 338 KB (GZip: 144 KB, Brotli: 113 KB, debug: 1239 KB) 
- With the `url` and `regex` crate - 928 KB (GZip: 326 KB, Brotli: 236 KB, debug: 3601 KB)

So if one of your dependency calls `format!` or `.to_string` on a float number, expect cca 20 KB larger Wasm file. If you want to use libraries like [reqwest](https://crates.io/crates/reqwest/0.11.3/dependencies), expect more than 300 KB of extra binary size because it uses `url` crate.

So I recommend to first look at your dependencies and try to find popular, but large libraries like [url](https://crates.io/crates/url), [regex](https://crates.io/crates/regex) and [serde](https://crates.io/crates/serde). Also some parts of `std` contributes to the code bloat, especially `std::fmt`.

Try to find alternatives - e.g. [ufmt](https://crates.io/crates/ufmt) for `std::fmt` or [serde-lite](https://crates.io/crates/serde-lite) for `serde`.
- Zoon hides `ufmt` and [lexical](https://crates.io/crates/lexical) for float number formatting behind a non-default feature flag. I'll probably add also `serde-lite` and integrate them properly in the future.

Or you can try to use the browser API instead of Rust libs - e.g. [js_sys::RegExp](https://docs.rs/js-sys/0.3.51/js_sys/struct.RegExp.html) instead of `regex` or [web_sys::Url](https://docs.rs/web-sys/0.3.51/web_sys/struct.Url.html) instead of `url`.

### Macros

Macros are basically code generators. Be prepared for larger binaries if you use them. Don't forgot that attributes like `#[derive(Debug)]` could generate a lot of code.

If you need to write your custom macros, try to extract as much code as possible to new functions.

### Panics / errors

The most code behind panics are fortunately removed by `wasm-opt`. However we can help it:

Call `expect_throw` and `unwrap_throw` instead of standard `expect` and `unwrap`. See [wasm_bindgen::UnwrapThrowExt](https://docs.rs/wasm-bindgen/0.2.74/wasm_bindgen/trait.UnwrapThrowExt.html).

In the Zoon code, there is registered a panic hook:
```rust
pub fn start_app ... {
    #[cfg(feature = "panic_hook")]
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();
```
This panic hooks is useful for debugging because it shows panic errors in console log. However we don't need it in the `release` build. `wasm-opt` can't remove it by itself - we should remove all debug helpers during the compilation process.

_Note:_ We need `console_error_panic_hook` because panics aren't automatically redirected to the console log. There are many `std` APIs that just do nothing in Wasm. That's why you need to, for instance, add `use zoon::{*, println};` if you want to call `println` in your app.

### Allocators

We were already talking about allocators. When you enable Zoon's feature `small_alloc` (it's enabled by default), then [wee_alloc](https://crates.io/crates/wee_alloc) allocator is used.

Related Zoon's code:

```rust
#[cfg(feature = "small_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
```

### Cargo.toml config

It's very similar to optimization for speed:

```toml
[profile.release]
lto = true  
codegen-units = 1  
opt-level = 's'

[package.metadata.wasm-pack.profile.release]
wasm-opt = ['-Os']
```

You need to experiment with values `'s'` / `['-Os']` and `'z'` / `['-Oz']`. Sometimes `s` makes the app smaller than `z` and even faster then `3`. It depends on your app and maybe on the weather. Who knows.

### Generics

There is a nice popular world in the Rust world - _monomorphization_.

An excerpt from Rust book, section [Performance of Code Using Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics):

- _"You might be wondering whether there is a runtime cost when you’re using generic type parameters. The good news is that Rust implements generics in such a way that your code doesn’t run any slower using generic types than it would with concrete types."_

Well, it's a bad news for us. [Explained in the docs](https://rustwasm.github.io/twiggy/concepts/generic-functions-and-monomorphization.html) for a very useful code size profile for Wasm [Twiggy](https://github.com/rustwasm/twiggy):

- _"Generic functions with type parameters in Rust and template functions in C++ can lead to code bloat if you aren't careful. Every time you instantiate these generic functions with a concrete set of types, the compiler will monomorphize the function, creating a copy of its body replacing its generic placeholders with the specific operations that apply to the concrete types."_

So it basically says we shouldn't use generics. However there are two problems:

1. Generics are often used in your dependencies - out of your control. E.g. Twiggy says that most code bloat because of generics is caused by the crate `futures-signals` in Zoon's `js-framework-benchmark` example.  

1. When I was trying to optimize size by replacing generics with other constructs, the app was becoming slower and paradoxically also bigger. So I wouldn't recommend to focus too much on this optimization if you aren't sure it'll really reduce the Wasm file size.

### Compression

Browsers support multiple kinds of compression, always at least [Gzip](https://en.wikipedia.org/wiki/Gzip) and [Brotli](https://github.com/google/brotli).

MoonZoon CLI (`mzoon`) automatically compresses `wasm` and other files with both algorithms during the `release` build. And then Moon serves them according to the header [Accept-Encoding](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Accept-Encoding) extracted from incoming requests.

You've already saw examples above, but let's look again:
- A small optimized app: 33 KB - GZip: 16 KB - Brotli: 14 KB
- A large optimized app: 928 KB - GZip: 326 KB - Brotli: 236 KB

This way you can significantly reduce traffic between frontend and backend.

_Note:_ Firefox and probably other browsers support Brotli only on HTTPS. Chrome supports both Gzip and Brotli also on HTTP. It means you can't use only Brotli for all cases.

_Dev Note:_ It's [difficult](https://github.com/MoonZoon/MoonZoon/pull/6#issuecomment-840037580) to serve files according to a header from Warp.

---

And that's all for today! 
Thank You for reading and I hope you are looking forward to the next episode.

Martin

P.S.
We are waiting for you on [Discord](https://discord.gg/eGduTxK2Es).

