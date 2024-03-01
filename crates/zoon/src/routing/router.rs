use crate::{routing::decode_uri_component, *};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use web_sys::MouseEvent;

type UrlChangeSender = Sender<Option<Vec<String>>>;

pub struct Router<R: Clone> {
    popstate_listener: SendWrapper<Closure<dyn Fn()>>,
    link_interceptor: SendWrapper<Closure<dyn Fn(MouseEvent)>>,
    url_change_sender: UrlChangeSender,
    route_history: Arc<Mutex<VecDeque<R>>>,
    _url_change_handle: TaskHandle,
}

impl<R: FromRouteSegments + Clone + 'static> Router<R> {
    pub fn new<O: Future<Output = ()> + 'static>(
        mut on_route_change: impl FnMut(Option<R>) -> O + 'static,
    ) -> Self {
        let route_history = Arc::new(Mutex::new(VecDeque::new()));
        let on_route_change = {
            let route_history = Arc::clone(&route_history);
            move |route| {
                if let Some(route) = &route {
                    let mut history = route_history.lock().unwrap_throw();
                    if history.len() == 2 {
                        history.pop_back();
                    }
                    history.push_front(Clone::clone(route));
                }
                on_route_change(route)
            }
        };
        let (url_change_sender, _url_change_handle) = setup_url_change_handler(on_route_change);
        Router {
            popstate_listener: setup_popstate_listener(url_change_sender.clone()),
            link_interceptor: setup_link_interceptor(url_change_sender.clone()),
            url_change_sender,
            route_history: Arc::new(Mutex::new(VecDeque::new())),
            _url_change_handle,
        }
    }

    pub fn go<'a>(&self, to: impl IntoCowStr<'a>) {
        go(&self.url_change_sender, to, false);
    }

    pub fn replace<'a>(&self, with: impl IntoCowStr<'a>) {
        replace(&self.url_change_sender, with, false);
    }

    pub fn silent_go<'a>(&self, to: impl IntoCowStr<'a>) {
        go(&self.url_change_sender, to, true);
    }

    pub fn silent_replace<'a>(&self, with: impl IntoCowStr<'a>) {
        replace(&self.url_change_sender, with, true);
    }

    pub fn current_route(&self) -> Option<R> {
        self.route_history.lock().unwrap_throw().get(0).cloned()
    }

    pub fn previous_route(&self) -> Option<R> {
        self.route_history.lock().unwrap_throw().get(1).cloned()
    }
}

impl<R: Clone> Drop for Router<R> {
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
    }
}

// ------ helpers -------

fn setup_url_change_handler<R: FromRouteSegments, O: Future<Output = ()> + 'static>(
    mut on_route_change: impl FnMut(Option<R>) -> O + 'static,
) -> (UrlChangeSender, TaskHandle) {
    let (url_change_sender, url_change_receiver) = channel(current_url_segments());
    let url_change_handler = url_change_receiver.for_each(move |segments| {
        let route = segments.and_then(R::from_route_segments);
        on_route_change(route)
    });
    let url_change_handle = Task::start_droppable(url_change_handler);
    (url_change_sender, url_change_handle)
}

fn go<'a>(url_change_sender: &UrlChangeSender, to: impl IntoCowStr<'a>, silent: bool) {
    let to = to.into_cow_str();
    if !to.starts_with('/') {
        return window().location().assign(&to).unwrap_throw();
    }
    history()
        .push_state_with_url(&JsValue::NULL, "", Some(&to))
        .unwrap_throw();
    if !silent {
        url_change_sender
            .send(current_url_segments())
            .unwrap_throw();
    }
}

fn replace<'a>(url_change_sender: &UrlChangeSender, with: impl IntoCowStr<'a>, silent: bool) {
    let with = with.into_cow_str();
    if !with.starts_with('/') {
        return window().location().replace(&with).unwrap_throw();
    }
    history()
        .replace_state_with_url(&JsValue::NULL, "", Some(&with))
        .unwrap_throw();
    if !silent {
        url_change_sender
            .send(current_url_segments())
            .unwrap_throw();
    }
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

fn setup_popstate_listener(url_change_sender: UrlChangeSender) -> SendWrapper<Closure<dyn Fn()>> {
    let closure = Closure::new(move || {
        url_change_sender
            .send(current_url_segments())
            .unwrap_throw();
    });

    window()
        .add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref())
        .unwrap_throw();

    SendWrapper::new(closure)
}

fn setup_link_interceptor(
    url_change_sender: UrlChangeSender,
) -> SendWrapper<Closure<dyn Fn(MouseEvent)>> {
    let closure = Closure::new(move |event| {
        link_click_handler(event, &url_change_sender);
    });

    document()
        .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
        .unwrap_throw();

    SendWrapper::new(closure)
}

fn link_click_handler(event: MouseEvent, url_change_sender: &UrlChangeSender) -> Option<()> {
    if event.ctrl_key() || event.meta_key() || event.shift_key() || event.button() != 0 {
        None?
    }
    let ws_element: web_sys::Element = event.target()?.dyn_into().ok()?;
    let a: web_sys::Element = ws_element
        .closest(r#"a[href^="/"]:not([download], [target="_blank"])"#)
        .ok()??;
    let href = a.get_attribute("href")?;
    event.prevent_default();
    go(url_change_sender, href, false);
    Some(())
}
