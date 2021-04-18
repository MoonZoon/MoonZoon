use crate::{RenderContext, ElVar};
use crate::hook::el_var;
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
pub fn dom_element(mut rcx: RenderContext, children: impl FnOnce(RenderContext)) -> ElVar<Node> {
    // log!("el, index: {}", rcx.index);

    let node = el_var(|| {
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

    let node = el_var(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        rcx.node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });

    let old_text = el_var(|| None::<String>);
    old_text.update_mut(|old_text| {
        if let Some(old_text) = old_text {
            if old_text != text {
                *old_text = text.to_owned();
                node.update_mut(|node| {
                    node.node_ws.set_node_value(Some(text));
                });
            }
            return
        }
        *old_text = Some(text.to_owned());
        node.update_mut(|node| {
            node.node_ws.set_node_value(Some(text));
        });
    });
}
