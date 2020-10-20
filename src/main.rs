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
    index: u32,
    state_node: State<Node>,
}

impl Cx {
    fn inc_index(&mut self) -> &mut Self {
        self.index += 1;
        self
    } 

    fn reset_index(&mut self) -> &mut Self {
        self.index = 0;
        self
    } 
}

struct Node {
    node_ws: web_sys::Node,
}

impl Drop for Node {
    fn drop(&mut self) {
        if let Some(parent) = self.node_ws.parent_node() {
            parent.remove_child(&self.node_ws);
        }
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

    let mut cx = Cx { 
        index: 0,
        state_node 
    };

    row(cx.inc_index().clone(), |mut cx| {
        column(cx.inc_index().clone(), |mut cx| {
            el( cx.inc_index().clone(), |mut cx| {
                text(cx.inc_index().clone(), "A1"); 
            });
            el(cx.inc_index().clone(), |mut cx| { 
                text(cx.inc_index().clone(), "A2");
            });
        });
        column(cx.inc_index().clone(), |mut cx| {
            el(cx.inc_index().clone(), |mut cx| {
                text(cx.inc_index().clone(), "B1");
            });
            el(cx.inc_index().clone(), |mut cx| { 
                text(cx.inc_index().clone(), "B2");
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
        let parent_node_ws = &(*cx.state_node.0).node_ws;
        parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("append node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    cx.reset_index();
    children(cx);
}

#[topo::nested]
fn column(mut cx: Cx, children: impl FnOnce(Cx)) {
    log!("column");

    let state_node = state(|| {
        let el_ws = document().create_element("div").expect("element");
        let node_ws = web_sys::Node::from(el_ws);
        let parent_node_ws = &(*cx.state_node.0).node_ws;
        parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("append node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    cx.reset_index();
    children(cx);
}

#[topo::nested]
fn el(mut cx: Cx, children: impl FnOnce(Cx)) {
    log!("el");

    let state_node = state(|| {
        let el_ws = document().create_element("div").expect("element");
        let node_ws = web_sys::Node::from(el_ws);
        let parent_node_ws = &(*cx.state_node.0).node_ws;
        parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("append node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    cx.reset_index();
    children(cx);
}

#[topo::nested]
fn text(mut cx: Cx, text: &str) {  
    log!("text");

    let state_node = state(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        let parent_node_ws = &(*cx.state_node.0).node_ws;
        parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("append node");
        Node { node_ws }
    });
    cx.state_node = state_node;
    // log!("text cx: {:#?}", cx);
}

