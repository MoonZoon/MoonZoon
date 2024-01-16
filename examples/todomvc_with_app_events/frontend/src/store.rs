use crate::app_event::*;
use uuid::Uuid;
use zoon::{eprintln, once_cell::sync::Lazy, strum::EnumIter, *};

#[route]
#[derive(Clone, Copy)]
pub enum Route {
    #[route("active")]
    Active,
    #[route("completed")]
    Completed,
    #[route()]
    Root,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(crate = "serde")]
pub struct Todo {
    pub id: Uuid,
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

static STORAGE_KEY: &str = "todomvc-zoon";

pub static ROUTER: Lazy<Router<Route>> = Lazy::new(|| {
    on(|FilterPressed { route }| ROUTER.go(route));
    Router::new(|route| async move { emit(RouteChanged { route }) })
});

pub static SELECTED_FILTER: Lazy<Mutable<Filter>> = Lazy::new(|| {
    on(|RouteChanged { route }| {
        SELECTED_FILTER.set_neq(match route {
            Some(Route::Active) => Filter::Active,
            Some(Route::Completed) => Filter::Completed,
            Some(Route::Root) | None => Filter::All,
        })
    });
    Mutable::new(Filter::All)
});

pub static SELECTED_TODO: Lazy<Mutable<Option<Todo>>> = Lazy::new(|| {
    on(|TodoTitleDoubleClicked { todo }| SELECTED_TODO.set(Some(todo.clone())));
    on(|EditingTodoTitleEscapePressed| SELECTED_TODO.set(None));
    on(|EditingTodoTitleBlurredOrEnterPressed| {
        if let Some(todo) = SELECTED_TODO.take() {
            emit(SelectedTodoToSaveTaken { todo })
        }
    });
    Mutable::new(None)
});

pub static NEW_TODO_TITLE: Lazy<Mutable<String>> = Lazy::new(|| {
    on(|NewTodoTitleChanged { title }| NEW_TODO_TITLE.set(title));
    on(|NewTodoTitleReadyToSave { .. }| NEW_TODO_TITLE.lock_mut().clear());
    Mutable::new(String::new())
});

pub static TODOS: Lazy<MutableVec<Todo>> = Lazy::new(|| {
    on(
        |RemoveTodoButtonPressed {
             todo: todo_to_remove,
         }| { TODOS.lock_mut().retain(|todo| todo.id != todo_to_remove.id) },
    );
    on(|ClearCompletedButtonPressed| TODOS.lock_mut().retain(|todo| not(todo.completed.get())));
    on(|NewTodoTitleReadyToSave { title }| {
        TODOS.lock_mut().push_cloned({
            Todo {
                id: Uuid::new_v4(),
                title: Mutable::new(title),
                completed: Mutable::new(false),
                edited_title: Mutable::new(None),
            }
        });
    });
    todo_ons();
    store_todos_on_change();
    if let Some(Ok(todos)) = local_storage().get(STORAGE_KEY) {
        MutableVec::new_with_values(todos)
    } else {
        MutableVec::new()
    }
});

fn todo_ons() {
    // todo.title
    on(|EditedTitleToSaveTaken { todo, title }| todo.title.set(title));

    // todo.completed
    on(|ToggleAllCheckboxClicked| {
        let todos = TODOS.lock_ref();
        let are_all_todos_completed = todos.iter().all(|todo| todo.completed.get());
        for todo in todos.iter() {
            todo.completed.set_neq(not(are_all_todos_completed));
        }
    });
    on(|TodoCheckboxChanged { todo, checked }| todo.completed.set(checked));

    // todo.edited_title
    on(|TodoTitleDoubleClicked { todo }| {
        if todo.edited_title.lock_ref().is_none() {
            todo.edited_title.set(Some(todo.title.get_cloned()));
        }
    });
    on(|EditingTodoTitleChanged { todo, text }| todo.edited_title.set_neq(Some(text)));
    on(|SelectedTodoToSaveTaken { todo }| {
        let title = todo.edited_title.take().unwrap_throw();
        emit(EditedTitleToSaveTaken { todo, title });
    });
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
