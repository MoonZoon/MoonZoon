use crate::{RenderContext, l_var, LVar};
use wasm_bindgen::JsCast;
use tracked_call_macro::tracked_call;
use crate::tracked_call::__TrackedCall;
use crate::tracked_call_stack::__TrackedCallStack;

// ------- Helpers ------

pub fn window() -> web_sys::Window {
    web_sys::window().expect("window")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("document")
}

// ------ Node ------

pub struct Node {
    pub node_ws: web_sys::Node,
}

impl Drop for Node {
    fn drop(&mut self) {
        if let Some(parent) = self.node_ws.parent_node() {
            parent.remove_child(&self.node_ws).unwrap();
        }
        // log!("Node dropped");
    }
}

#[tracked_call]
pub fn dom_element(mut rcx: RenderContext, children: impl FnOnce(RenderContext)) -> LVar<Node> {
    // log!("el, index: {}", rcx.index);

    let node = l_var(|| {
        let el_ws = document().create_element("div").expect("element");
        el_ws.set_attribute("class", "el").expect("set class attribute");
        let node_ws = web_sys::Node::from(el_ws);
        rcx.node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    rcx.node = node;
    rcx.reset_index();
    children(rcx);
    node
}

#[tracked_call]
pub fn dom_text(rcx: RenderContext, text: &str) {  
    // log!("text, index: {}", rcx.index);

    let old_text = l_var(|| None::<String>);
    let node = l_var(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        rcx.node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });

    if old_text.map(|old_text| {
        Some(text) == old_text.as_ref().map(String::as_str)
    }) {
        return;
    }
    old_text.set(Some(text.to_owned()));
    node.update_mut(|node| {
        node.node_ws.set_node_value(Some(text));
    });
}
