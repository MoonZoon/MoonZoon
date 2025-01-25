use std::sync::Arc;

use zoon::*;

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
        .map(value_to_element);

    signal::from_stream(element_stream)
}

fn value_to_element(value: Value) -> RawElOrText {
    match value {
        Value::Text(text) => {
            zoon::Text::new(text.text()).unify()
        }
        Value::Number(number) => {
            zoon::Text::new(number.number()).unify()
        }
        _ => panic!("Element cannot be created from the given Value type")
    }
}
