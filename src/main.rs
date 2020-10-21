use wasm_bindgen::JsCast;

mod console;
mod controls;
mod hooks;
mod state;
mod runtime;
mod state_map;

use hooks::use_state;
use state::{State, CloneState};
use console::log;
use controls::{
    Control,
    button::{self, ApplyToButton},
    column::{self, ApplyToColumn},
    el::{self, ApplyToEl},
    row::{self, ApplyToRow},
    text::{self, ApplyToText},
};

const ELEMENT_ID: &str = "app";

fn runtime_run_once() {
    runtime::run_once(|| root());
}

#[derive(Copy, Clone)]
pub struct Cx {
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

#[derive(Clone)]
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

    // for revision in 0..2 {
    //     log!("revision: {}", revision);
    //     root();
    // }

    log!("-------- revision: 0 --------");
    runtime_run_once();

    log!("-------- revision: 1 --------");
    runtime_run_once();

    log!("-------- revision: 3 --------");
    runtime_run_once();
}

#[topo::nested]
fn root() {
    log!("root");

    let state_node = use_state(|| Node {
        node_ws: web_sys::Node::from(document().get_element_by_id(ELEMENT_ID).expect("root element"))
    });

    let mut cx = Cx { 
        index: 0,
        state_node 
    };

    let first_run = use_state(|| true);

    row![
        row::children(|mut cx| {
            column![
                column::children(|mut cx| {
                    row![
                        row::children(|mut cx| {
                            el![
                                el::child(text!["A1"]),
                            ].build(cx.inc_index().clone());
                            button![
                                button::label(text!["X"]),
                                button::on_press(|| log!("delete A1")),
                            ].build(cx.inc_index().clone());
                        }),
                    ].build(cx.inc_index().clone());
                    row![
                        row::children(|mut cx| {
                            el![
                                el::child(text!["A2"]),
                            ].build(cx.inc_index().clone());
                            button![
                                button::label(text!["X"]),
                                button::on_press(|| log!("delete A2")),
                            ].build(cx.inc_index().clone());
                        }),
                    ].build(cx.inc_index().clone());
                }),
            ].build(cx.inc_index().clone());

            if first_run.get() {
                log!("FIRST RUN!");

                column![
                    column::children(|mut cx| {
                        row![
                            row::children(|mut cx| {
                                el![
                                    el::child(text!["B1"]),
                                ].build(cx.inc_index().clone());
                                button![
                                    button::label(text!["X"]),
                                    button::on_press(|| log!("delete B1")),
                                ].build(cx.inc_index().clone());
                            }),
                        ].build(cx.inc_index().clone());
                        row![
                            row::children(|mut cx| {
                                el![
                                    el::child(text!["B2"]),
                                ].build(cx.inc_index().clone());
                                button![
                                    button::label(text!["X"]),
                                    button::on_press(|| log!("delete B2")),
                                ].build(cx.inc_index().clone());
                            }),
                        ].build(cx.inc_index().clone());
                    }),
                ].build(cx.inc_index().clone());
            }
        }),
    ].build(cx.inc_index().clone());

    // if first_run.get() {
    //     first_run.set(false);
    // }
}

#[topo::nested]
fn raw_el(mut cx: Cx, children: impl FnOnce(Cx)) -> State<Node> {
    // log!("el, index: {}", cx.index);

    let state_node = use_state(|| {
        let el_ws = document().create_element("div").expect("element");
        el_ws.set_attribute("class", "el").expect("set class attribute");
        let node_ws = web_sys::Node::from(el_ws);
        cx.state_node.update(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    cx.state_node = state_node;
    cx.reset_index();
    children(cx);
    state_node
}

#[topo::nested]
fn raw_text(mut cx: Cx, text: &str) {  
    log!("text, index: {}", cx.index);

    let state_node = use_state(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        cx.state_node.update(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    cx.state_node = state_node;
}

