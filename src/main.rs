use wasm_bindgen::{prelude::*, JsCast};

mod hooks;
mod state;
mod runtime;
mod state_map;

use hooks::use_state;
use state::{State, CloneState};

const ELEMENT_ID: &str = "app";

macro_rules! log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(input: &str);
}

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

mod button {
    use wasm_bindgen::JsCast;
    use super::{Cx, raw_el, log};

    #[derive(Default)]
    pub struct Button {
        label: Option<Label>,
        on_press: Option<OnPress>,
    }

    impl Button {
        pub fn new() -> Self {
            Self::default()
        }

        #[topo::nested]
        pub fn build(self, cx: Cx) {
            log!("button, index: {}", cx.index);

            let state_node = raw_el(cx, |cx| {
                if let Some(label) = self.label {
                    (label.0)(cx)
                }
            });
            state_node.update(|node| {
                let element = node.node_ws.unchecked_ref::<web_sys::Element>();
                element.set_attribute("class", "button");
                element.set_attribute("role", "button");
                element.set_attribute("tabindex", "0");
            });
        }
    }

    pub trait ApplyToButton {
        fn apply_to_button(self, button: &mut Button);
    }

    pub struct Label(Box<dyn FnOnce(Cx)>);
    pub fn label(label: impl FnOnce(Cx) + 'static) -> Label {
        Label(Box::new(label))
    }
    impl ApplyToButton for Label {
        fn apply_to_button(self, button: &mut Button) {
            button.label = Some(self);
        }
    }

    pub struct OnPress(Box<dyn FnOnce()>);
    pub fn on_press(on_press: impl FnOnce() + 'static) -> OnPress {
        OnPress(Box::new(on_press))
    }
    impl ApplyToButton for OnPress {
        fn apply_to_button(self, button: &mut Button) {
            button.on_press = Some(self);
        }
    }
}

use button::ApplyToButton;

macro_rules! button {
    ( $($item:expr),* $(,)?) => {
        {
            let mut button = button::Button::new();
            $(
                $item.apply_to_button(&mut button);
            )*
            button
        }
    }
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

    row(cx.inc_index().clone(), |mut cx| {
        column(cx.inc_index().clone(), |mut cx| {
            row(cx.inc_index().clone(), |mut cx| {
                el(cx.inc_index().clone(), |mut cx| {
                    text(cx.inc_index().clone(), "A1"); 
                });
                button![
                    button::label(|mut cx| text(cx.inc_index().clone(), "X")),
                    button::on_press(|| log!("delete A1")),
                ].build(cx.inc_index().clone());
            });
            row(cx.inc_index().clone(), |mut cx| {
                el(cx.inc_index().clone(), |mut cx| {
                    text(cx.inc_index().clone(), "A2"); 
                });
                button(cx.inc_index().clone(), || log!("delete A2"), |mut cx| {
                    text(cx.inc_index().clone(), "X"); 
                });
            });
        });
        if first_run.get() {
            log!("FIRST RUN!");

            column(cx.inc_index().clone(), |mut cx| {
                row(cx.inc_index().clone(), |mut cx| {
                    el(cx.inc_index().clone(), |mut cx| {
                        text(cx.inc_index().clone(), "B1"); 
                    });
                    button(cx.inc_index().clone(), || log!("delete B1"), |mut cx| {
                        text(cx.inc_index().clone(), "X"); 
                    });
                });
                row(cx.inc_index().clone(), |mut cx| {
                    el(cx.inc_index().clone(), |mut cx| {
                        text(cx.inc_index().clone(), "B2"); 
                    });
                    button(cx.inc_index().clone(), || log!("delete B2"), |mut cx| {
                        text(cx.inc_index().clone(), "X"); 
                    });
                });
            });
        }
    });

    if first_run.get() {
        first_run.set(false);
    }
}

#[topo::nested]
fn button(cx: Cx, on_press: impl FnOnce(), children: impl FnOnce(Cx)) {
    log!("button, index: {}", cx.index);

    let state_node = raw_el(cx, |cx| {
        children(cx)
    });
    state_node.update(|node| {
        let element = node.node_ws.unchecked_ref::<web_sys::Element>();
        element.set_attribute("class", "button");
        element.set_attribute("role", "button");
        element.set_attribute("tabindex", "0");
    });
}

#[topo::nested]
fn row(cx: Cx, children: impl FnOnce(Cx)) {
    log!("row, index: {}", cx.index);

    let state_node = raw_el(cx, |cx| {
        children(cx)
    });
    state_node.update(|node| {
        let element = node.node_ws.unchecked_ref::<web_sys::Element>();
        element.set_attribute("class", "row");
    });
}

#[topo::nested]
fn column(cx: Cx, children: impl FnOnce(Cx)) {
    log!("column, index: {}", cx.index);

    let state_node = raw_el(cx, |cx| {
        children(cx)
    });
    state_node.update(|node| {
        let element = node.node_ws.unchecked_ref::<web_sys::Element>();
        element.set_attribute("class", "column");
    });
}

#[topo::nested]
fn el(cx: Cx, children: impl FnOnce(Cx)) {
    log!("el, index: {}", cx.index);

    let state_node = raw_el(cx, |cx| {
        children(cx)
    });
    state_node.update(|node| {
        let element = node.node_ws.unchecked_ref::<web_sys::Element>();
        element.set_attribute("class", "element");
    });
}

#[topo::nested]
fn text(cx: Cx, text: &str) {
    log!("text, index: {}", cx.index);

    raw_text(cx, text);
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

