use frontend::*;

// WARNING: This is a proof of concept of using Web Workers.
// It leverages the crate `gloo_worker`.
// Worker examples have been copied from the `gloo` GitHub repo and modified.

// @TODO automatize / improve API / etc
// @TODO don't forget to add extra `wasm-bindgen` arguments for workers, process it through `wasm-opt` and compress it (uncomment the related code in Moon)
// @TODO make it compatible with `--frontend-dist` and `mzoon new` (don't forget to add `resolver = "2"` and update `.gitignore` everywhere)

fn main() {
    store();
    start_app("app", root);
}
