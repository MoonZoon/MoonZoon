use crate::*;
use once_cell::race::OnceBox;
use std::mem::ManuallyDrop;
use std::{cell::Cell, rc::Rc};
use web_sys::{EventTarget, Node};

mod raw_html_el;
mod raw_svg_el;

pub use raw_html_el::RawHtmlEl;
pub use raw_svg_el::RawSvgEl;

type U32Width = u32;
type U32Height = u32;

// ------ class_ids ------

fn class_id_generator() -> &'static ClassIdGenerator {
    static GLOBAL_STYLES: OnceBox<ClassIdGenerator> = OnceBox::new();
    GLOBAL_STYLES.get_or_init(|| Box::new(ClassIdGenerator::default()))
}

#[derive(Default)]
struct ClassIdGenerator {
    index_generator: IndexGenerator,
}

impl ClassIdGenerator {
    fn next_class_id(&self) -> ClassId {
        ClassId::new(["_", &self.index_generator.next_index().to_string()].concat())
    }

    fn remove_class_id(&self, class_id: ClassId) {
        let class_id = class_id.take().unwrap_throw();
        self.index_generator
            .remove_index(class_id[1..].parse().unwrap_throw());
    }
}

// ------ UpdateRawEl ------

pub trait UpdateRawEl {
    type RawEl: RawEl;
    fn update_raw_el(self, updater: impl FnOnce(Self::RawEl) -> Self::RawEl) -> Self;
}

// ------ RawEl ------

pub trait RawEl: Sized {
    #[doc(hidden)]
    type DomElement: AsRef<Node>
        + AsRef<EventTarget>
        + AsRef<JsValue>
        + AsRef<web_sys::Element>
        + Into<web_sys::Element>
        + Clone
        + JsCast
        + 'static;

    fn new(tag: &str) -> Self;

    #[doc(hidden)]
    fn update_dom_builder(
        self,
        updater: impl FnOnce(DomBuilder<Self::DomElement>) -> DomBuilder<Self::DomElement>,
    ) -> Self;

    fn dom_element(&self) -> Self::DomElement;

    fn use_dom_element(self, f: impl FnOnce(Self, Self::DomElement) -> Self) -> Self {
        let dom_element = self.dom_element();
        f(self, dom_element)
    }

    fn attr(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| dom_builder.attribute(name, value))
    }

    fn attr_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.attribute_signal(
                name.into_cow_str_wrapper(),
                value.map(|value| value.into_option_cow_str_wrapper()),
            )
        })
    }

    fn prop(self, name: &str, value: &str) -> Self {
        self.update_dom_builder(|dom_builder| dom_builder.property(name, JsValue::from_str(value)))
    }

    fn prop_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.property_signal(
                name.into_cow_str_wrapper(),
                value.map(|value| value.into_option_cow_str_wrapper()),
            )
        })
    }

    fn event_handler<E: StaticEvent>(self, handler: impl FnOnce(E) + Clone + 'static) -> Self {
        self.event_handler_with_options(EventOptions::default(), handler)
    }

    fn event_handler_with_options<E: StaticEvent>(
        self,
        options: EventOptions,
        handler: impl FnOnce(E) + Clone + 'static,
    ) -> Self {
        let handler = move |event: E| handler.clone()(event);
        self.update_dom_builder(|dom_builder| {
            dom_builder.event_with_options(&options.into(), handler)
        })
    }

    fn global_event_handler<E: StaticEvent>(
        self,
        handler: impl FnOnce(E) + Clone + 'static,
    ) -> Self {
        self.global_event_handler_with_options(EventOptions::default(), handler)
    }

    fn global_event_handler_with_options<E: StaticEvent>(
        self,
        options: EventOptions,
        handler: impl FnOnce(E) + Clone + 'static,
    ) -> Self {
        let handler = move |event: E| handler.clone()(event);
        self.update_dom_builder(|dom_builder| {
            dom_builder.global_event_with_options(&options.into(), handler)
        })
    }

    fn child<'a>(self, child: impl IntoOptionElement<'a> + 'a) -> Self {
        if let Some(child) = child.into_option_element() {
            return self.update_dom_builder(|dom_builder| {
                dom_builder.child(child.into_raw_element().into_dom())
            });
        }
        self
    }

    fn child_signal<'a>(
        self,
        child: impl Signal<Item = impl IntoOptionElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.child_signal(child.map(|child| {
                child
                    .into_option_element()
                    .map(|element| element.into_raw_element().into_dom())
            }))
        })
    }

    fn children<'a>(self, children: impl IntoIterator<Item = impl IntoElement<'a> + 'a>) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.children(
                children
                    .into_iter()
                    .map(|child| child.into_element().into_raw_element().into_dom()),
            )
        })
    }

    fn children_signal_vec<'a>(
        self,
        children: impl SignalVec<Item = impl IntoElement<'a>> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.children_signal_vec(
                children.map(|child| child.into_element().into_raw_element().into_dom()),
            )
        })
    }

    fn style(self, name: &str, value: &str) -> Self;

    fn style_important(self, name: &str, value: &str) -> Self;

    fn style_signal<'a>(
        self,
        name: impl IntoCowStr<'static>,
        value: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
    ) -> Self;

    fn style_group(mut self, mut group: StyleGroup) -> Self {
        if group.selector.is_empty() {
            let StyleGroup { selector: _, static_css_props, dynamic_css_props } = group;
            for (name, CssPropValue { value, important }) in static_css_props {
                if important {
                    self = self.style_important(name, &value);
                } else {
                    self = self.style(name, &value);
                }
            }
            for (name, value) in dynamic_css_props {
                self = self.style_signal(name, value);
            }
            return self;
        }

        group.selector = self.class_id().map(|class_id| {
            [".", class_id.unwrap_throw(), &group.selector]
                .concat()
                .into()
        });
        let group_handle = global_styles().style_group_droppable(group);
        self.after_remove(|_| drop(group_handle))
    }

    fn after_insert(self, handler: impl FnOnce(Self::DomElement) + 'static) -> Self {
        let handler = ManuallyDrop::new(handler);
        let handler = |ws_element| ManuallyDrop::into_inner(handler)(ws_element);
        self.update_dom_builder(|dom_builder| dom_builder.after_inserted(handler))
    }

    fn after_remove(self, handler: impl FnOnce(Self::DomElement) + 'static) -> Self {
        let handler = ManuallyDrop::new(handler);
        let handler = |ws_element| ManuallyDrop::into_inner(handler)(ws_element);
        self.update_dom_builder(|dom_builder| dom_builder.after_removed(handler))
    }

    fn class(self, class: &str) -> Self {
        self.update_dom_builder(|dom_builder| dom_builder.class(class))
    }

    fn class_signal<'a>(
        self,
        class: impl IntoCowStr<'static>,
        enabled: impl Signal<Item = bool> + Unpin + 'static,
    ) -> Self {
        self.update_dom_builder(|dom_builder| {
            dom_builder.class_signal(class.into_cow_str_wrapper(), enabled)
        })
    }

    fn on_resize(mut self, handler: impl FnOnce(U32Width, U32Height) + Clone + 'static) -> Self {
        // @TODO should we create one global ResizeObserver to improve performance?
        // Inspiration: https://gist.github.com/Pauan/d9dcf0b47fc03c7a49b95f29ff8ef3c3

        let resize_observer = Rc::new(Cell::new(None));
        let resize_observer_for_insert = Rc::clone(&resize_observer);

        self = self.after_insert(move |ws_element| {
            let observer = ResizeObserver::new(ws_element.as_ref(), handler);
            resize_observer_for_insert.set(Some(observer));
        });

        self.after_remove(move |_| {
            drop(resize_observer);
        })
    }

    fn class_id(&self) -> ClassId;

    fn inner_markup(self, markup: impl AsRef<str>) -> Self {
        let dom_element = self.dom_element();
        let parent: &web_sys::Element = dom_element.as_ref();
        parent.set_inner_html(markup.as_ref());
        self
    }

    fn inner_markup_signal<'a>(
        self,
        markup: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
    ) -> Self {
        let parent: web_sys::Element = self.dom_element().into();
        let inner_html_updater = markup.for_each_sync(move |markup| {
            parent.set_inner_html(&markup.into_cow_str());
        });
        let inner_html_updater = Task::start_droppable(inner_html_updater);
        self.after_remove(move |_| drop(inner_html_updater))
    }

    fn from_markup(markup: impl AsRef<str>) -> Option<Self> {
        // https://grrr.tech/posts/create-dom-node-from-html-string/

        let template: web_sys::HtmlTemplateElement = document()
            .create_element("template")
            .unwrap_throw()
            .unchecked_into();

        template.set_inner_html(markup.as_ref().trim());
        let element = template.content().first_element_child()?;
        Some(Self::from_dom_element(element.dyn_into().ok()?))
    }

    fn find_html_child(&self, selectors: impl AsRef<str>) -> Option<RawHtmlEl<web_sys::HtmlElement>> {
        let parent_dom_element = self.dom_element();
        let parent: &web_sys::Element = parent_dom_element.as_ref();
        let child = parent
            .query_selector(selectors.as_ref())
            .expect_throw("query_selector failed")?
            .dyn_into()
            .ok()?;
        Some(RawHtmlEl::from_dom_element(child))
    }

    fn find_svg_child(&self, selectors: impl AsRef<str>) -> Option<RawSvgEl<web_sys::SvgElement>> {
        let parent_dom_element = self.dom_element();
        let parent: &web_sys::Element = parent_dom_element.as_ref();
        let child = parent
            .query_selector(selectors.as_ref())
            .expect_throw("query_selector failed")?
            .dyn_into()
            .ok()?;
        Some(RawSvgEl::from_dom_element(child))
    }

    fn update_html_child(
        self,
        selectors: impl AsRef<str>,
        updater: impl FnOnce(RawHtmlEl<web_sys::HtmlElement>) -> RawHtmlEl<web_sys::HtmlElement>,
    ) -> Self {
        if let Some(child) = self.find_html_child(selectors) {
            let child = updater(child);
            return self.after_remove(move |_| drop(child));
        }
        self
    }

    fn update_svg_child(
        self,
        selectors: impl AsRef<str>,
        updater: impl FnOnce(RawSvgEl<web_sys::SvgElement>) -> RawSvgEl<web_sys::SvgElement>,
    ) -> Self {
        if let Some(child) = self.find_svg_child(selectors) {
            let child = updater(child);
            return self.after_remove(move |_| drop(child));
        }
        self
    }

    fn from_dom_element(dom_element: Self::DomElement) -> Self;

    fn focus(self) -> Self where Self::DomElement: AsRef<web_sys::HtmlElement>;

    fn focus_signal(self, focus: impl Signal<Item = bool> + Unpin + 'static) -> Self
        where Self::DomElement: AsRef<web_sys::HtmlElement>;
}
