use crate::*;

pub struct Task;

impl Task {
    pub fn start(future: impl Future<Output = ()> + 'static) {
        spawn_local(future)
    }
}
