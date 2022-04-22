use crate::*;

pub trait ResizableViewport: UpdateRawEl + Sized {
    fn on_viewport_size_change(
        self,
        handler: impl FnOnce(U32Width, U32Height) + Clone + 'static,
    ) -> Self {
        self.update_raw_el(|raw_el| raw_el.on_resize(handler))
    }
}
