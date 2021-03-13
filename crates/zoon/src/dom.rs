use crate::log;
use crate::RenderContext;
use crate::l_var;
use crate::LVar;
use wasm_bindgen::JsCast;

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
        log!("Node dropped");
    }
}

#[topo::nested]
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

#[topo::nested]
pub fn dom_text(mut rcx: RenderContext, text: &str) {  
    // log!("text, index: {}", rcx.index);

    let node = l_var(|| {
        let node_ws = document().create_text_node(&text).unchecked_into::<web_sys::Node>();
        rcx.node.update_mut(|node| {
            let parent_node_ws = &node.node_ws;
            parent_node_ws.insert_before(&node_ws, parent_node_ws.child_nodes().get(rcx.index + 1).as_ref()).expect("insert node");
        });
        Node { node_ws }
    });
    rcx.node = node;
}
