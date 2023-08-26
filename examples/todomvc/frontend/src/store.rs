use educe::Educe;
use uuid::Uuid;
use zoon::{eprintln, strum::EnumIter, *};

static STORAGE_KEY: &str = "todomvc-zoon";

#[static_ref]
pub fn store() -> &'static Store {
    let store = Store::default();
    if let Some(Ok(todos)) = local_storage().get(STORAGE_KEY) {
        store.todos.lock_mut().replace_cloned(todos);
    }
    create_triggers();
    store
}

#[derive(Default)]
pub struct Store {
    pub todos: MutableVec<Todo>,
    pub selected_filter: Mutable<Filter>,
    pub selected_todo: Mutable<Option<Todo>>,
    pub new_todo_title: Mutable<String>,
    // -- caches --
    pub todos_count: Mutable<usize>,
    pub active_todos_count: Mutable<usize>,
    pub completed_todos_count: Mutable<usize>,
    pub are_todos_empty: Mutable<bool>,
    pub are_completed_todos_empty: Mutable<bool>,
    pub are_all_todos_completed: Mutable<bool>,
}

#[derive(Educe)]
#[educe(Deref, Default(new))]
#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
#[serde(crate = "serde")]
pub struct TodoId(#[educe(Default(expression = "Uuid::new_v4()"))] Uuid);

#[derive(Educe)]
#[educe(Default(new))]
#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "serde")]
pub struct Todo {
    pub id: TodoId,
    pub title: Mutable<String>,
    pub completed: Mutable<bool>,
    #[serde(skip)]
    pub edited_title: Mutable<Option<String>>,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumIter, Default)]
#[strum(crate = "strum")]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

fn create_triggers() {
    Task::start(async {
        store()
            .todos
            .signal_vec_cloned()
            .map_signal(|todo| {
                map_ref! {
                    let _ = todo.title.signal_ref(|_|()),
                    let _ = todo.completed.signal_ref(|_|()) =>
                    todo.clone()
                }
            })
            .to_signal_cloned()
            .for_each_sync(|todos| {
                if let Err(error) = local_storage().insert(STORAGE_KEY, &todos) {
                    eprintln!("failed to store todos: {error:#?}");
                }
                let completed_count = todos.iter().filter(|todo| todo.completed.get()).count();
                store().todos_count.set_neq(todos.len());
                store()
                    .active_todos_count
                    .set_neq(todos.len() - completed_count);
                store().completed_todos_count.set_neq(completed_count);
                store().are_todos_empty.set_neq(todos.len() == 0);
                store()
                    .are_completed_todos_empty
                    .set_neq(completed_count == 0);
                store()
                    .are_all_todos_completed
                    .set_neq(todos.len() == completed_count);
            })
            .await
    });
}
