mod bindings;

use crate::bindings::exports::golem::component::api::*;
use futures_signals::signal::{Mutable, SignalExt};
use std::sync::LazyLock;

#[derive(Default)]
struct State {
    total: Mutable<u64>,
}

static STATE: LazyLock<State> = LazyLock::new(|| {
    let state = State::default();
    task::spawn(state.total.signal().for_each(|total| {
        println!("total changed: {total}!");
        async {}
    }));
    state
});

struct Component;

impl Guest for Component {
    fn add(value: u64) {
        *STATE.total.lock_mut() += value;
        task::run_all();
    }

    fn get() -> u64 {
        let total = STATE.total.get();
        task::run_all();
        total
    }
}

bindings::export!(Component with_types_in bindings);

mod task {
    use futures_executor::{LocalPool, LocalSpawner};
    use futures_task::LocalSpawn;
    use std::cell::RefCell;
    use std::future::Future;

    thread_local! {
        static TASK_POOL: RefCell<LocalPool> = Default::default();
        static TASK_SPAWNER: LocalSpawner = TASK_POOL.with_borrow(|pool| pool.spawner());
    }

    pub fn spawn(future: impl Future<Output = ()> + 'static) {
        TASK_SPAWNER.with(move |spawner| spawner.spawn_local_obj(Box::pin(future).into()).unwrap())
    }

    pub fn run_all() {
        TASK_POOL.with_borrow_mut(|pool| pool.run_until_stalled())
    }
}
