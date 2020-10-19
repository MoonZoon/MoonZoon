use wasm_bindgen::{prelude::*, JsCast};
use moxie::{runtime::Runtime, state, Commit, Key};
use std::cell::RefCell;

const ELEMENT_ID: &str = "app";

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(input: &str);
}

type State<T> = (Commit<T>, Key<T>);

#[derive(Clone)]
struct Cx {
    state_node: State<Node>,
}

struct Node {
    node_ws: web_sys::Node,
}

impl Drop for Node {
    fn drop(&mut self) {
        let parent = self.node_ws.parent_node().expect("parent Node");
        parent.remove_child(&self.node_ws);
        log!("Node dropped");
    }
}

thread_local! {
    static RUNTIME: RefCell<Runtime> = RefCell::new(Runtime::new());
}

fn runtime_run_once() {
    RUNTIME.with(|runtime| {
        runtime.borrow_mut().run_once(root);
    });
}

fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("document")
}

fn main() {
    log!("main");
    console_error_panic_hook::set_once();

    runtime_run_once();
    // runtime_run_once();
}

#[topo::nested]
fn root() {
    log!("root");

    let state_node = state(|| Node {
        node_ws: web_sys::Node::from(document().get_element_by_id(ELEMENT_ID).expect("root element"))
    });
    let cx = Cx { state_node };

    row(cx.clone(), |cx| {
        column(cx.clone(), |cx| {
            el( cx.clone(), |cx| {
                text(cx.clone(), "A1"); 
            });
            el(cx.clone(), |cx| { 
                text(cx.clone(), "A2");
            });
        });
        column(cx.clone(), |cx| {
            el(cx.clone(), |cx| {
                text(cx.clone(), "B1");
            });
            el(cx.clone(), |cx| { 
                text(cx.clone(), "B2");
            });
        });
    });

    // let (first_run, first_run_key) = state(|| true);
    // if *first_run {
    //     first_run_key.set(false);
    //     panel(|cx| {
    //         text("Panel 1", cx.clone());
    //         text("A", cx.clone());
    //     }, cx.clone());
    // }
    // panel(|cx| {
    //     text("Panel 2", cx.clone());
    //     text("B", cx.clone());
    // }, cx.clone());
}

#[topo::nested]
fn row(mut cx: Cx, children: impl FnOnce(Cx)) {
    log!("row");

    let state_node = state(|| {
        let el_ws = document().create_element("div").expect("element");
        let node_ws = web_sys::Node::from(el_ws);
        (*cx.state_node.0).node_ws.append_child(&node_ws).expect("append element node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    children(cx);
}

#[topo::nested]
fn column(mut cx: Cx, children: impl FnOnce(Cx)) {
    log!("column");

    let state_node = state(|| {
        let el_ws = document().create_element("div").expect("element");
        let node_ws = web_sys::Node::from(el_ws);
        (*cx.state_node.0).node_ws.append_child(&node_ws).expect("append element node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    children(cx);
}

#[topo::nested]
fn el(mut cx: Cx, children: impl FnOnce(Cx)) {
    log!("el");

    let state_node = state(|| {
        let el_ws = document().create_element("div").expect("element");
        let node_ws = web_sys::Node::from(el_ws);
        (*cx.state_node.0).node_ws.append_child(&node_ws).expect("append element node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    children(cx);
}

#[topo::nested]
fn text(mut cx: Cx, text: &str) {  
    log!("text");

    let state_node = state(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        (*cx.state_node.0).node_ws.append_child(&node_ws).expect("append text node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    // log!("text cx: {:#?}", cx);
}

