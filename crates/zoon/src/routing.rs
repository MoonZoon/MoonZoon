use crate::*;
use futures_signals::signal::{channel, Sender};
use futures_util::future::{abortable, ready, AbortHandle};
use std::marker::PhantomData;
use web_sys::MouseEvent;

// @TODO: feature "routing"?

mod from_route_segments;
mod route_segment;

pub use from_route_segments::FromRouteSegments;
pub use route_segment::RouteSegment;

// ------ pub functions ------

pub fn current_url() -> String {
    window().location().href().unwrap_throw()
}

pub fn back() {
    history().back().unwrap_throw();
}

pub fn decode_uri_component(component: impl AsRef<str>) -> Result<String, JsValue> {
    let decoded = js_sys::decode_uri_component(component.as_ref())?;
    Ok(String::from(decoded))
}

pub fn encode_uri_component(component: impl AsRef<str>) -> String {
    let encoded = js_sys::encode_uri_component(component.as_ref());
    String::from(encoded)
}

// ------ Router ------

pub struct Router<R> {
    popstate_listener: SendWrapper<Closure<dyn Fn()>>,
    link_interceptor: SendWrapper<Closure<dyn Fn(MouseEvent)>>,
    url_change_sender: Sender<Option<Vec<String>>>,
    url_change_handle: AbortHandle,
    _route_type: PhantomData<R>,
}

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

    pub fn go<'a>(&self, to: impl IntoCowStr<'a>) {
        Self::inner_go(&self.url_change_sender, to);
    }

    pub fn replace<'a>(&self, with: impl IntoCowStr<'a>) {
        let with = with.into_cow_str();
        if !with.starts_with('/') {
            return window().location().replace(&with).unwrap_throw();
        }
        history()
            .replace_state_with_url(&JsValue::NULL, "", Some(&with))
            .unwrap_throw();
        self.url_change_sender
            .send(Self::current_url_segments())
            .unwrap_throw();
    }

    // -- private --

    fn inner_go<'a>(url_change_sender: &Sender<Option<Vec<String>>>, to: impl IntoCowStr<'a>) {
        let to = to.into_cow_str();
        if !to.starts_with('/') {
            return window().location().assign(&to).unwrap_throw();
        }
        history()
            .push_state_with_url(&JsValue::NULL, "", Some(&to))
            .unwrap_throw();
        url_change_sender
            .send(Self::current_url_segments())
            .unwrap_throw();
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
    ) -> SendWrapper<Closure<dyn Fn(MouseEvent)>> {
        let closure = Closure::wrap(Box::new(move |event| {
            Self::link_click_handler(event, &url_change_sender);
        }) as Box<dyn Fn(MouseEvent)>);

        document()
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap_throw();

        SendWrapper::new(closure)
    }

    fn link_click_handler(event: MouseEvent, url_change_sender: &Sender<Option<Vec<String>>>) -> Option<()> {
        if event.ctrl_key() || event.meta_key() || event.shift_key() || event.button() != 0 {
            None?
        }
        let ws_element: web_sys::Element = event.target()?.dyn_into().ok()?;
        let a: web_sys::Element = ws_element.closest(r#"a[href^="/"]:not([download], [target="_blank"])"#).ok()??;
        let href = a.get_attribute("href")?;
        event.prevent_default();
        Self::inner_go(url_change_sender, href);    
        Some(())
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
