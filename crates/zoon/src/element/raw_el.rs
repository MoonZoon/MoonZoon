use crate::{Element, IntoElement};
use futures_signals::{signal::{Signal, SignalExt}, signal_vec::{SignalVec, SignalVecExt}};
use dominator::{Dom, html, DomBuilder, traits::StaticEvent};
use std::borrow::Cow;

// ------ ------
//   Element 
// ------ ------

// #[derive(Default)]
// pub struct RawEl<'a> {
//     tag: Option<Tag<'a>>,
//     attributes: Vec<Attribute<'a>>,
//     event_handlers: Vec<EventHandler>,
//     children: Vec<Child>,
//     children_signal_vec: Option<Box<dyn SignalVec<Item = Dom> + Unpin>>
// }

// enum Child {
//     Static(Dom),
//     Dynamic(Box<dyn Signal<Item = Option<Dom>> + Unpin>),
// }

// enum Attribute<'a> {
//     Static(&'a str, &'a str),
//     Dynamic(Cow<'static, str>, Box<dyn Signal<Item = Option<String>> + Unpin>),
// }

// #[derive(Debug)]
// pub struct CustomEvent {
//     event: web_sys::Event,
// }

// impl StaticEvent for CustomEvent {
//     // @TODO how?
//     const EVENT_TYPE: &'static str = "[custom]";

//     #[inline]
//     fn unchecked_from_event(event: web_sys::Event) -> Self {
//         Self { event }
//     }
// }

// impl<'a> Element for RawEl<'a> {
//     fn render(self) -> Dom {
//         let tag = self.tag.map_or("div", |Tag(tag)| tag);
//         let mut builder = DomBuilder::<web_sys::HtmlElement>::new_html(tag);

//         for attribute in self.attributes {
//             builder = match attribute {
//                 Attribute::Static(name, value) => builder.attribute(name, value),
//                 // @TODO without Cow / refactor?
//                 Attribute::Dynamic(name, value) => builder.attribute_signal(name.to_string(), value),
//             }
//         }

//         for item in self.children {
//             builder = match item {
//                 Child::Static(child) => builder.child(child),
//                 Child::Dynamic(child) => builder.child_signal(child),
//             }
//         }

//         if let Some(items_signal_vec) = self.children_signal_vec {
//             builder = builder
//                 .children_signal_vec(items_signal_vec);
//         }

//         for EventHandler(event_handler) in self.event_handlers {
//             crate::println!("eh");
//             builder = builder.event(event_handler);
//         }

//         builder.into_dom()




        // html!(self.tag.unwrap_or("div"), {

        // })

        // let node = dom_element(rcx, self.tag, |mut rcx| {
        //     for child in &mut self.children {
        //         child.render(rcx.inc_index().clone());
        //     }
        // });

        // let attrs = el_var(|| HashMap::<String, String>::default());
        // attrs.update_mut(|attrs| {
        //     node.update_mut(|node| {
        //         let element = node.node_ws.unchecked_ref::<web_sys::Element>();

        //         attrs.retain(|name, value| {
        //             if let Some(new_value) = self.attrs.remove(name.as_str()) {
        //                 if new_value != value {
        //                     element.set_attribute(name, value).unwrap();
        //                     *value = new_value.to_owned();
        //                 }
        //                 return true
        //             } 
        //             element.remove_attribute(name).unwrap();
        //             false
        //         });

        //         for (new_name, new_value) in mem::take(&mut self.attrs) {
        //             attrs.insert(new_name.to_owned(), new_value.to_owned());
        //             element.set_attribute(new_name, new_value).unwrap();
        //         }
        //     });
        // });

        // let listeners = el_var(|| HashMap::new());
        // listeners.update_mut(|listeners| {
        //     for (event, handlers) in mem::take(&mut self.event_handlers) {
        //         if handlers.is_empty() {
        //             listeners.remove(event);
        //             continue;
        //         }
        //         listeners
        //             .entry(event)
        //             .or_insert(Listener::new(event, node))
        //             .set_handlers(handlers);
        //     }
        // });
//     }
// }

// ------ ------
//  Attributes 
// ------ ------

// impl<'a> RawEl<'a> {
//     pub fn child(mut self, child: impl IntoElement<'a> + 'a) -> Self {
//         child.into_element().apply_to_element(&mut self);
//         self
//     }
// } 

// // ------ IntoElement ------

// impl<'a, T: IntoElement<'a> + 'a> ApplyToElement<RawEl<'a>> for T {
//     fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
//         raw_el.children.push(Child::Static(self.into_element().render()))
//     }
// }

// // ------ raw_el::tag(...)

// pub struct Tag<'a>(&'a str);
// pub fn tag<'a>(tag: &'a str) -> Tag<'a> {
//     Tag(tag)
// }
// impl<'a> ApplyToElement<RawEl<'a>> for Tag<'a> {
//     fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
//         raw_el.tag = Some(self);
//     }
// }

// // ------ raw_el::attr(...)

// pub struct Attr<'a>(&'a str, &'a str);
// pub fn attr<'a>(name: &'a str, value: &'a str) -> Attr<'a> {
//     Attr(name, value)
// }
// impl<'a> ApplyToElement<RawEl<'a>> for Attr<'a> {
//     fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
//         let Attr(name, value) = self;
//         raw_el.attributes.push(Attribute::Static(name, value));
//     }
// }

// // ------ raw_el::attr_signal(...) ------

// pub struct AttrSignal(Cow<'static, str>, Box<dyn Signal<Item = Option<String>> + Unpin>);
// pub fn attr_signal(name: impl Into<Cow<'static, str>>, attr: impl Signal<Item = Option<String>> + Unpin + 'static) -> AttrSignal {
//     AttrSignal(name.into(), Box::new(attr))
// }
// impl<'a> ApplyToElement<RawEl<'a>> for AttrSignal {
//     fn apply_to_element(self, row: &mut RawEl<'a>) {
//         let AttrSignal(name, value) = self;
//         row.attributes.push(Attribute::Dynamic(name, value));
//     }
// }

// // ------ raw_el::event_handler(...)

// pub struct EventHandler(Box<dyn FnMut(CustomEvent)>);
// pub fn event_handler(event: &str, handler: impl FnOnce(web_sys::Event) + Clone + 'static) -> EventHandler {
//     EventHandler(Box::new(move |event: CustomEvent| {
//         handler.clone()(event.event)
//     }))
// }
// impl<'a> ApplyToElement<RawEl<'a>> for EventHandler {
//     fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
//         raw_el.event_handlers.push(self)
//     }
// }

// // ------ raw_el::child_signal(...) ------

// pub struct ChildSignal(Box<dyn Signal<Item = Option<Dom>> + Unpin>);
// pub fn child_signal<'a, IE: IntoElement<'a> + 'a>(child: impl Signal<Item = IE> + Unpin + 'static) -> ChildSignal {
//     ChildSignal(Box::new(child.map(|child| Some(child.into_element().render()))))
// }
// impl<'a> ApplyToElement<RawEl<'a>> for ChildSignal {
//     fn apply_to_element(self, row: &mut RawEl<'a>) {
//         row.children.push(Child::Dynamic(self.0));
//     }
// }

// // ------ raw_el::children_signal_vec(...) ------

// pub struct ChildrenSignalVec(Box<dyn SignalVec<Item = Dom> + Unpin>);
// pub fn children_signal_vec<'a, IE: IntoElement<'a> + 'a>(children: impl SignalVec<Item = IE> + Unpin + 'static) -> ChildrenSignalVec {
//     ChildrenSignalVec(Box::new(children.map(|child| child.into_element().render())))
// }
// impl<'a> ApplyToElement<RawEl<'a>> for ChildrenSignalVec {
//     fn apply_to_element(self, raw_el: &mut RawEl<'a>) {
//         raw_el.children_signal_vec = Some(self.0);
//     }
// }
