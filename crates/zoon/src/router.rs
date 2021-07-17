use crate::*;
use futures_signals::signal::{channel, Sender};
use futures_util::future::{abortable, ready, AbortHandle};
use std::marker::PhantomData;
use web_sys::Event;

mod from_route_segments;
mod route_segment;

pub use from_route_segments::FromRouteSegments;
pub use route_segment::RouteSegment;

pub fn decode_uri_component(component: impl AsRef<str>) -> Result<String, JsValue> {
    let decoded = js_sys::decode_uri_component(component.as_ref())?;
    Ok(String::from(decoded))
}

pub fn encode_uri_component(component: impl AsRef<str>) -> String {
    let encoded = js_sys::encode_uri_component(component.as_ref());
    String::from(encoded)
}

pub struct Router<R> {
    popstate_listener: SendWrapper<Closure<dyn Fn()>>,
    link_interceptor: SendWrapper<Closure<dyn Fn(Event)>>,
    url_change_sender: Sender<Option<Vec<String>>>,
    url_change_handle: AbortHandle,
    _route_type: PhantomData<R>,
}

// @TODO: Encoding

impl<R: FromRouteSegments> Router<R> {
    pub fn new(on_route_change: impl FnOnce(Option<R>) + Clone + 'static) -> Self {
        let on_route_change = move |route: Option<R>| on_route_change.clone()(route);

        let (url_change_sender, url_change_receiver) = channel(Self::current_url_segments());
        let url_change_handler = url_change_receiver.for_each(move |segments| {
            let route = segments.and_then(R::from_route_segments);
            on_route_change(route);
            ready(())
        });
        let (url_change_handler, url_change_handle) = abortable(url_change_handler);
        spawn_local(async {
            let _ = url_change_handler.await;
        });

        Router {
            popstate_listener: Self::setup_popstate_listener(url_change_sender.clone()),
            link_interceptor: Self::setup_link_interceptor(url_change_sender.clone()),
            url_change_sender,
            url_change_handle,
            _route_type: PhantomData,
        }
    }

    pub fn current_url() -> String {
        window().location().href().unwrap_throw()
    }

    pub fn go<'a>(&self, to: impl IntoCowStr<'a>) {
        Self::inner_go(&self.url_change_sender, to);
    }

    pub fn replace<'a>(&self, with: impl IntoCowStr<'a>) {
        let with = with.into_cow_str();

        let mut segments = Vec::new();
        for segment in with.trim_start_matches('/').split_terminator('/') {
            segments.push(segment.to_owned());
        }

        history()
            .replace_state_with_url(&JsValue::NULL, "", Some(&with))
            .unwrap_throw();

        self.url_change_sender.send(Some(segments)).unwrap_throw();
    }

    // -- private --

    fn inner_go<'a>(url_change_sender: &Sender<Option<Vec<String>>>, to: impl IntoCowStr<'a>) {
        let to = to.into_cow_str();

        let mut segments = Vec::new();
        for segment in to.trim_start_matches('/').split_terminator('/') {
            segments.push(segment.to_owned());
        }

        history()
            .push_state_with_url(&JsValue::NULL, "", Some(&to))
            .unwrap_throw();

        url_change_sender.send(Some(segments)).unwrap_throw();
    }

    fn current_url_segments() -> Option<Vec<String>> {
        let path = window().location().pathname().unwrap_throw();
        let mut segments = Vec::new();
        for segment in path.trim_start_matches('/').split_terminator('/') {
            match decode_uri_component(segment) {
                Ok(segment) => segments.push(segment),
                Err(error) => {
                    crate::eprintln!(
                        "Cannot decode the URL segment '{}'. Error: {:#?}",
                        segment,
                        error
                    );
                    None?
                }
            }
        }
        Some(segments)
    }

    fn setup_popstate_listener(
        url_change_sender: Sender<Option<Vec<String>>>,
    ) -> SendWrapper<Closure<dyn Fn()>> {
        let closure = Closure::wrap(Box::new(move || {
            url_change_sender
                .send(Self::current_url_segments())
                .unwrap_throw();
        }) as Box<dyn Fn()>);

        window()
            .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
            .unwrap_throw();

        SendWrapper::new(closure)
    }

    fn setup_link_interceptor(
        url_change_sender: Sender<Option<Vec<String>>>,
    ) -> SendWrapper<Closure<dyn Fn(Event)>> {
        let closure = Closure::wrap(Box::new(move |event: Event| {
            event
                .target()
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
                        Self::inner_go(&url_change_sender, href);
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
            .remove_event_listener_with_callback(
                "popstate",
                self.popstate_listener.as_ref().unchecked_ref(),
            )
            .unwrap_throw();

        document()
            .remove_event_listener_with_callback(
                "click",
                self.link_interceptor.as_ref().unchecked_ref(),
            )
            .unwrap_throw();

        self.url_change_handle.abort();
    }
}
