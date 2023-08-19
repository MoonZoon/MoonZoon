use crate::*;

pub trait CloneDeep: Serialize + DeserializeOwned {
    fn clone_deep(&self) -> Self {
        // @TODO a better way?
        let value = serde_json::to_value(self).unwrap_throw();
        serde_json::from_value(value).unwrap_throw()
    }
}

impl<T: Serialize + DeserializeOwned> CloneDeep for T {}
