use crate::log;

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
