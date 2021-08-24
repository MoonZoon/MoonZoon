use crate::*;
use std::{rc::Rc, cell::Cell};

type U32Width = u32;
type U32Height = u32;

pub trait ResizableViewport<T: RawEl>: UpdateRawEl<T> + Sized {
    fn on_viewport_size_change(
        self,
        handler: impl FnOnce(U32Width, U32Height) + Clone + 'static,
    ) -> Self {
        self.update_raw_el(|mut raw_el| {
            let resize_observer = Rc::new(Cell::new(None));
            let resize_observer_for_insert = Rc::clone(&resize_observer);

            raw_el = raw_el.after_insert(move |ws_element| {
                let observer = ResizeObserver::new(ws_element.as_ref(), handler);
                resize_observer_for_insert.set(Some(observer));
            });

            raw_el.after_remove(move |_| {
                drop(resize_observer);
            })
        })
    }
}
