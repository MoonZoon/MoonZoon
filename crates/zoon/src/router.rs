use crate::*;
use std::marker::PhantomData;
use web_sys::Event;

mod route_segment;
mod from_route_segments;

pub use route_segment::RouteSegment;
pub use from_route_segments::FromRouteSegments;

pub struct Router<R> {
    _route_type: PhantomData<R>,
    popstate_listener: SendWrapper<Closure<dyn Fn()>>,
    link_interceptor: SendWrapper<Closure<dyn Fn(Event)>>,
}

impl<R: FromRouteSegments> Router<R> {
    pub fn new(on_route_change: impl FnOnce(Option<R>) + Clone + 'static) -> Self {
        let popstate_listener = Self::setup_popstate_listener();
        let link_interceptor = Self::setup_link_interceptor();
        Router {
            _route_type: PhantomData,
            popstate_listener,
            link_interceptor,
        }
    }

    pub fn go_to<'a>(to: impl IntoCowStr<'a>) {
        let to = to.into_cow_str();
        history()
            .push_state_with_url(&JsValue::NULL, "", Some(&to))
            .unwrap_throw();
    }

    pub fn current_url() -> String {
        window().location().href().unwrap_throw()
    }

    // -- private --

    fn setup_popstate_listener() -> SendWrapper<Closure<dyn Fn()>> {
        let closure = Closure::wrap(Box::new(move || {
            crate::println!("popstate!");
        }) as Box<dyn Fn()>);
    
        window()
            .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
            .unwrap_throw();
    
        SendWrapper::new(closure)
    }

    fn setup_link_interceptor() -> SendWrapper<Closure<dyn Fn(Event)>> {
        let closure = Closure::wrap(Box::new(move |event: Event| {
            event.target()
                .and_then(|et| et.dyn_into::<web_sys::Element>().ok())
                .and_then(|el| el.closest("a[href]").ok().flatten())
                .and_then(|href_el| {
                    if href_el.has_attribute("download") {
                        None
                    } else {
                        href_el.get_attribute("href")
                    }
                })
                // The first character being / or empty href indicates a rel link, which is what
                // we're intercepting.
                // @TODO: Resolve it properly, see Elm implementation:
                // @TODO: https://github.com/elm/browser/blob/9f52d88b424dd12cab391195d5b090dd4639c3b0/src/Elm/Kernel/Browser.js#L157
                .and_then(|href| {
                    if href.is_empty() || href.starts_with('/') {
                        Some(href)
                    } else {
                        None
                    }
                })
                .map(|href| {
                    // @TODO should be empty href ignored?
                    if href.is_empty() {
                        event.prevent_default(); // Prevent page refresh
                    } else {
                        event.prevent_default();
                        Self::go_to(href);
                    }
                });
        }) as Box<dyn Fn(Event)>);
    
        document()
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap_throw();

        SendWrapper::new(closure)
    }
}

impl<R> Drop for Router<R> {
    fn drop(&mut self) {
        window()
            .remove_event_listener_with_callback("popstate", self.popstate_listener.as_ref().unchecked_ref())
            .unwrap_throw();

        document()
            .remove_event_listener_with_callback("click", self.link_interceptor.as_ref().unchecked_ref())
            .unwrap_throw();
    }
}
