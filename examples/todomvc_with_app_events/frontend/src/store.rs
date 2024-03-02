use crate::app_event::*;
use std::sync::Arc;
use uuid::Uuid;
use zoon::{eprintln, strum::EnumIter, *};

#[route]
#[derive(Clone, Copy, Debug)]
pub enum Route {
    #[route("active")]
    Active,
    #[route("completed")]
    Completed,
    #[route()]
    Root,
}

#[derive(Copy, Clone, Eq, PartialEq, EnumIter, Default)]
#[strum(crate = "strum")]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "serde")]
pub struct Todo {
    pub id: Uuid,
    pub title: Mutable<Arc<String>>,
    pub completed: Mutable<bool>,
    #[serde(skip)]
    pub edited_title: Mutable<Option<Arc<String>>>,
}

static STORAGE_KEY: &str = "todomvc-zoon";

pub static ROUTER: Lazy<Router<Route>> = Lazy::new(|| {
    on(|FilterPressed { route }| ROUTER.go(route));
    Router::default()
});

pub static SELECTED_TODO: Lazy<Mutable<Option<Todo>>> = Lazy::new(|| {
    on(|TodoTitleDoubleClicked { todo, .. }| SELECTED_TODO.set(Some(todo.clone())));
    on(|EditingTodoTitleEscapePressed| SELECTED_TODO.set(None));
    on(|EditingTodoTitleBlurredOrEnterPressed { .. }| SELECTED_TODO.set(None));
    Mutable::new(None)
});

pub static NEW_TODO_TITLE: Lazy<Mutable<String>> = Lazy::new(|| {
    on(|NewTodoTitleChanged { title }| NEW_TODO_TITLE.set(title));
    on(|NewTodoTitlePreparedForSaving { .. }| NEW_TODO_TITLE.lock_mut().clear());
    Mutable::new(String::new())
});

pub static TODOS: Lazy<MutableVec<Todo>> = Lazy::new(|| {
    on(|RemoveTodoButtonPressed { todo }| {
        TODOS.lock_mut().retain(|Todo { id, .. }| id != &todo.id)
    });
    on(|ClearCompletedButtonPressed| TODOS.lock_mut().retain(|todo| not(todo.completed.get())));
    on(|NewTodoTitlePreparedForSaving { title }| {
        TODOS.lock_mut().push_cloned({
            Todo {
                id: Uuid::new_v4(),
                title: Mutable::new(title),
                completed: Mutable::new(false),
                edited_title: Mutable::new(None),
            }
        });
    });
    todo_setters();
    store_todos_on_change();
    local_storage()
        .get(STORAGE_KEY)
        .and_then(Result::ok)
        .map(MutableVec::new_with_values)
        .unwrap_or_default()
});

fn todo_setters() {
    '_title: {
        on(
            |EditingTodoTitleBlurredOrEnterPressed { todo, edited_title }| {
                todo.title.set(edited_title)
            },
        );
    }
    '_completed: {
        on(|ToggleAllCheckboxClicked| {
            let todos = TODOS.lock_ref();
            let are_all_todos_completed = todos.iter().all(|todo| todo.completed.get());
            for todo in todos.iter() {
                todo.completed.set_neq(not(are_all_todos_completed));
            }
        });
        on(|TodoCheckboxChanged { todo, checked }| todo.completed.set(checked));
    }
    '_edited_title: {
        on(|TodoTitleDoubleClicked { todo, title }| {
            todo.edited_title.lock_mut().get_or_insert(title);
        });
        on(|EditingTodoTitleChanged { todo, text }| {
            todo.edited_title.set_neq(Some(Arc::new(text)))
        });
        on(|EditingTodoTitleBlurredOrEnterPressed { todo, .. }| todo.edited_title.set(None));
    }
}

fn store_todos_on_change() {
    Task::start(async {
        TODOS
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
            })
            .await
    });
}
