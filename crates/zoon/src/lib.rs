pub use wasm_bindgen::{self, prelude::*, JsCast};
pub use blocks::blocks;

pub mod component;
mod console;
mod hook;
mod state;
mod runtime;
mod state_map;

pub use state::{State, CloneState};
pub use console::log;
pub use hook::{el_var, do_once};
pub use topo;
pub use topo::nested as render;
pub use component::*;

const ELEMENT_ID: &str = "app";

fn runtime_run_once() {
    runtime::run_once(|| root());
}

#[macro_export]
macro_rules! start {
    () => {
        $crate::start();
    };
}

#[derive(Copy, Clone)]
pub struct RenderContext {
    index: u32,
    state_node: State<Node>,
}

impl RenderContext {
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
            parent.remove_child(&self.node_ws).unwrap();
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

pub trait Component {
    fn render(&mut self, rcx: RenderContext);

    fn new() -> Self 
        where Self: Default
    {
        Self::default()
    }

    fn with(mut self, attribute: impl ApplyToComponent<Self>) -> Self
        where Self: Sized
    {
        attribute.apply_to_component(&mut self);
        self
    }
}

// pub trait IntoComponent {
//     type CMP;
//     fn into_component(self) -> Self::CMP; 
// }

pub trait ApplyToComponent<T: Component> {
    fn apply_to_component(self, component: &mut T);
}

pub fn start() {
    log!("start");
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

    let rcx = RenderContext { 
        index: 0,
        state_node 
    };

    
    row![
        col![
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


    ].render(rcx);

    //     do_once(|| {
    //         log!("FIRST RUN!");

    //         column![
    //             row![
    //                 el![
    //                     text!["B1"],
    //                 ],
    //                 button![
    //                     text!["X"],
    //                     button::on_press(|| log!("delete B1")),
    //                 ],
    //             ],
    //             row![
    //                 el![
    //                     text!["B2"],
    //                 ],
    //                 button![
    //                     text!["X"],
    //                     button::on_press(|| log!("delete B2")),
    //                 ],
    //             ],
    //         ]
    //     }),
    // ].render(rcx);
}

#[topo::nested]
fn raw_el(mut rcx: RenderContext, children: impl FnOnce(RenderContext)) -> State<Node> {
    // log!("el, index: {}", rcx.index);

    let state_node = el_var(|| {
        let el_ws = document().create_element("div").expect("element");
        el_ws.set_attribute("class", "el").expect("set class attribute");
        let node_ws = web_sys::Node::from(el_ws);
        rcx.state_node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    rcx.state_node = state_node;
    rcx.reset_index();
    children(rcx);
    state_node
}

#[topo::nested]
fn raw_text(mut rcx: RenderContext, text: &str) {  
    // log!("text, index: {}", rcx.index);

    let state_node = el_var(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        rcx.state_node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    rcx.state_node = state_node;
}
