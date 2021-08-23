use crate::*;

// ------ ResizeObserver ------

pub struct ResizeObserver {

}

impl ResizeObserver {
    pub fn new(on_resize: impl FnOnce(Vec<ResizeObserverEntry>) + Clone) -> Self {
        let on_resize = move |entries| on_resize.clone()(entries); 
        Self {}
    }

    pub fn observe(&self, element: web_sys::Element) {

    }

    pub fn unobserve(&self, element: web_sys::Element) {

    }

    pub fn disconnect(&self) {

    }
}

impl Drop for ResizeObserver {
    fn drop(&mut self) {
        self.disconnect();
    }
}

// ------ ResizeObserverEntry ------

pub struct ResizeObserverEntry {

}
