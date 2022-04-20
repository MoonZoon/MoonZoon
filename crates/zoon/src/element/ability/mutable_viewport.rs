use crate::*;
use std::convert::TryFrom;

pub trait MutableViewport: UpdateRawEl + Sized {
    fn on_viewport_location_change(
        self,
        handler: impl FnOnce(Scene, Viewport) + Clone + 'static,
    ) -> Self {
        self.update_raw_el(|raw_el| {
            raw_el.event_handler(move |event: events::Scroll| {
                let target = event
                    .target()
                    .unwrap_throw()
                    .unchecked_into::<web_sys::Element>();
                let scene = Scene {
                    width: u32::try_from(target.scroll_width()).unwrap_throw(),
                    height: u32::try_from(target.scroll_height()).unwrap_throw(),
                };
                let viewport = Viewport {
                    x: target.scroll_left(),
                    y: target.scroll_top(),
                    width: u32::try_from(target.client_width()).unwrap_throw(),
                    height: u32::try_from(target.client_height()).unwrap_throw(),
                };
                handler(scene, viewport);
            })
        })
    }

    fn viewport_x_signal(self, x: impl Signal<Item = i32> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.prop_signal("scrollLeft", x))
    }

    fn viewport_y_signal(self, y: impl Signal<Item = i32> + Unpin + 'static) -> Self {
        self.update_raw_el(|raw_el| raw_el.prop_signal("scrollTop", y))
    }
}
