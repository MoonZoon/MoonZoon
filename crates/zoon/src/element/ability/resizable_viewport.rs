use crate::*;

pub trait ResizableViewport: UpdateRawEl + Sized {
    fn on_viewport_size_change(self, handler: impl FnMut(U32Width, U32Height) + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.on_resize(handler))
    }
}
