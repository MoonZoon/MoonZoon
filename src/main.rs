use wasm_bindgen::JsCast;

mod console;
mod controls;
mod hooks;
mod state;
mod runtime;
mod state_map;
mod bool_ext;

use hooks::{el_var, do_once};
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
use bool_ext::BoolExt;

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

    let state_node = el_var(|| Node {
        node_ws: web_sys::Node::from(document().get_element_by_id(ELEMENT_ID).expect("root element"))
    });

    let cx = Cx { 
        index: 0,
        state_node 
    };

    row![
        column![
            row![
                el![
                    text!["A1"],
                ],
                button![
                    text!["X"],
                    button::on_press(|| log!("delete A1")),
                ],
            ],
            row![
                el![
                    text!["A2"],
                ],
                button![
                    text!["X"],
                    button::on_press(|| log!("delete A2")),
                ],
            ],
        ],

        do_once(|| {
            log!("FIRST RUN!");

            column![
                row![
                    el![
                        text!["B1"],
                    ],
                    button![
                        text!["X"],
                        button::on_press(|| log!("delete B1")),
                    ],
                ],
                row![
                    el![
                        text!["B2"],
                    ],
                    button![
                        text!["X"],
                        button::on_press(|| log!("delete B2")),
                    ],
                ],
            ]
        }),
    ].build(cx);
}

#[topo::nested]
fn raw_el(mut cx: Cx, children: impl FnOnce(Cx)) -> State<Node> {
    // log!("el, index: {}", cx.index);

    let state_node = el_var(|| {
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
    // log!("text, index: {}", cx.index);

    let state_node = el_var(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        cx.state_node.update(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(cx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    cx.state_node = state_node;
}

