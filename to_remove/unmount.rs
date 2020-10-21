use crate::state_access::StateAccess;

pub struct Unmount {
    pub activated: bool,
    pub on_unmount: Box<dyn Fn() -> ()>,
}

impl Unmount {
    pub fn new(on_unmount: impl Fn() -> () + 'static) -> Self {
        Self {
            activated: true,
            on_unmount: Box::new(on_unmount),
        }
    }

    pub fn execute_if_activated(&self) {
        if self.activated {
            (self.on_unmount)();
        }
    }

    pub fn activate(&mut self) {
        self.activated = true;
    }
    pub fn deactivate(&mut self) {
        self.activated = false;
    }
}

pub trait StateAccessUnmount {
    fn activate(&self);
    fn deactivate(&self);
    fn execute_and_remove(self);
}

impl StateAccessUnmount for StateAccess<Unmount> {
    fn execute_and_remove(self) {
        self.update(|dt| {
            dt.execute_if_activated();
        });
        self.remove();
    }

    fn activate(&self) {
        self.update(|dt| dt.activate());
    }

    fn deactivate(&self) {
        self.update(|dt| dt.deactivate());
    }
}
