use std::{sync::Arc, ops::Deref};

#[derive(Clone)]
pub struct ClassId(Arc<String>);

impl ClassId {
    pub(crate) fn new(class_id: String) -> Self {
        Self(Arc::new(class_id))
    }
}

impl Deref for ClassId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
