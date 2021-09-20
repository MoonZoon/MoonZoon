use crate::*;
use std::sync::{Arc, RwLock};

#[derive(Clone, Default)]
pub struct ClassId(Arc<RwLock<Option<String>>>);

impl ClassId {
    pub(crate) fn new(class_id: String) -> Self {
        Self(Arc::new(RwLock::new(Some(class_id))))
    }

    pub(crate) fn take(&self) -> Option<String> {
        self.0
            .write()
            .expect_throw("cannot write to ClassId")
            .take()
    }

    pub fn map<T>(&self, f: impl FnOnce(Option<&String>) -> T) -> T {
        f(self.0.read().expect_throw("cannot read ClassId").as_ref())
    }
}
