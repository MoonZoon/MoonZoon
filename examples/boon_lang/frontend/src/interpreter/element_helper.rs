use zoon::{*, println, eprintln};
use zoon::futures_util::select;

use std::sync::Arc;

use super::engine::*;

pub fn root_object_to_element_signal(root_object: Arc<Object>) -> impl Signal<Item = Option<RawElOrText>> {
    let element_stream = root_object
        .expect_variable("document")
        .subscribe()
        .flat_map(|value| {
            value
                .expect_object()
                .expect_variable("root_element")
                .subscribe()
        })
        .flat_map(value_to_element_stream);

    signal::from_stream(element_stream)
}

pub fn value_to_element_stream(value: Value) -> impl Stream<Item = RawElOrText> {
    match value {
        Value::Text(text) => {
            constant(zoon::Text::new(text.text()).unify())
        }
        Value::Number(number) => {
            constant(zoon::Text::new(number.number()).unify())
        }
        _ => panic!("Element cannot be created from the given Value type")
    }
}
