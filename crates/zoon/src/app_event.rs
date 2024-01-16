use crate::*;
use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;

// @TODO we can make it faster - don't hash TypeId, unsafe downcast,
// move into the last handler, `on_ref`, etc.
// - see crates "anymap{x}", "dfb" and "handler_map"

thread_local! {
    static HANDLER_MAP: RefCell<HashMap<TypeId, Vec<Box<dyn FnMut(Box<dyn Any>)>>>> = RefCell::new(HashMap::new());
}

pub fn on<T: Any>(mut handler: impl FnMut(T) + 'static) {
    Task::start(async move {
        let handler = Box::new(move |value: Box<dyn Any>| {
            let value = value.downcast::<T>().unwrap_throw();
            handler(*value)
        });
        let type_id = TypeId::of::<T>();
        HANDLER_MAP.with_borrow_mut(|handler_map| {
            if let Some(handlers) = handler_map.get_mut(&type_id) {
                handlers.push(handler);
            } else {
                handler_map.insert(type_id, vec![handler]);
            }
        });
    });
}

pub fn emit(app_event: impl Any + Clone) {
    Task::start(async move {
        let type_id = app_event.type_id();
        HANDLER_MAP.with_borrow_mut(|handler_map| {
            if let Some(handlers) = handler_map.get_mut(&type_id) {
                if handlers.len() == 1 {
                    handlers[0](Box::new(app_event));
                } else {
                    for handler in handlers {
                        handler(Box::new(app_event.clone()))
                    }
                }
            }
        });
    });
}
