use crate::*;

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap_throw()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap_throw()
}

pub fn history() -> web_sys::History {
    window().history().unwrap_throw()
}

pub fn head() -> web_sys::HtmlHeadElement {
    document().head().unwrap_throw()
}

pub fn body() -> web_sys::HtmlBodyElement {
    document().body().unwrap_throw().unchecked_into()
}

pub async fn load_stylesheet(url: impl AsRef<str>) {
    let link: web_sys::HtmlLinkElement = document()
        .create_element("link")
        .unwrap_throw()
        .unchecked_into();

    link.set_attribute("href", url.as_ref()).unwrap_throw();
    link.set_attribute("rel", "stylesheet").unwrap_throw();
    link.set_attribute("type", "text/css").unwrap_throw();

    head().append_child(&link).unwrap_throw();

    let (sender, receiver) = oneshot::channel();
    let load_event_handler = Closure::once_into_js(move || sender.send(()).unwrap_throw());
    link.add_event_listener_with_callback("load", load_event_handler.unchecked_ref())
        .unwrap_throw();
    receiver.await.unwrap_throw();
}

pub async fn load_script(url: impl AsRef<str>) {
    // Note: `script` cannot be added dynamically through `set_inner_html` (silent fail).
    // Note: async by default (https://javascript.info/script-async-defer#dynamic-scripts).

    let script: web_sys::HtmlScriptElement = document()
        .create_element("script")
        .unwrap_throw()
        .unchecked_into();

    script.set_attribute("src", url.as_ref()).unwrap_throw();
    script
        .set_attribute("type", "text/javascript")
        .unwrap_throw();

    head().append_child(&script).unwrap_throw();

    let (sender, receiver) = oneshot::channel();
    let load_event_handler = Closure::once_into_js(move || sender.send(()).unwrap_throw());
    script
        .add_event_listener_with_callback("load", load_event_handler.unchecked_ref())
        .unwrap_throw();
    receiver.await.unwrap_throw();
}
