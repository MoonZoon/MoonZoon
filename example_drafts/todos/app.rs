use zoon::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use ulid::Ulid;


type TodoId = Ulid;
type Todos = BTreeMap<TodoId, Model<Todo>>;

// ------ Routes ------

#[Route]
enum Route {
    #[path("/active")]
    Active,
    #[path("/completed")]
    Completed,
    #[path("/")]
    Root,
    Unknown,
}

#[Cache]
fn route() -> Cache<Route> {
    let url = watch(zoon::model::url());

    use_cache(url.changed(), || {
        url.map(Route::from)
    })
}

#[Update]
fn set_route(route: Route) {
    zoon::model::url().update(Url::from)
}

// ------ Filters ------

// derive Iter / to_vec
enum Filter {
    All,
    Completed,
    Active,
}

#[Model]
fn filters() -> Model<Vec<Filter>> {
    use_model(|| Filter::iter().collect())
}

#[Cache]
fn selected_filter() -> Cache<Filter> {
    let route = watch(selected_route());

    use_cache(route.changed(), || {
        match route.inner() {
            Route::Active => Filter::Active,
            Route::Completed => Filter::Completed,
            _ => Filter::All,
        }
    })
}

// ------ SelectedTodo ------

struct SelectedTodo {
    id: TodoId,
    title: String,
}

#[Model]
fn selected_todo() -> Model<SelectedTodo> {
    use_model(|| None)
}

// ------ Todos ------

struct Todo {
    id: TodoId,
    title: String,
    completed: bool,
}

#[Model]
fn new_todo_title() -> Model<String> {
    use_model(String::new)
}

#[Update]
fn set_new_todo_title(title: String) {
    new_todo_title().set(title);
}

#[Update]
fn create_todo(title: &str) {
    let title = title.trim();
    if title.is_empty() {
        return;
    }

    let todo_id = TodoId::new();
    let mut todo = new_model(|| Todo {
        id: todo_id,
        title: title.trim(),
        completed: false,
    });

    todos().update(|todos| todos.insert(todo_id, todo));
    new_todo_title().update(|title| title.clear());
}

#[Update]
fn remove_todo(todo: Model<Todo>) {
    todos().update(|todos| todos.remove(todo.inner().id));
    todo.remove();
}

#[Update]
fn toggle_todo(todo: Model<Todo>) {
    todo.update(|todo| *todo.checked = !todo.checked);
}

// -- all --

#[Model]
fn todos() -> Model<Todos> {
    use_model(BTreeMap::new)
}

#[Update]
fn check_or_uncheck_all(checked: bool) {
    if are_all_completed().inner() {
        todos().update(|todos| {
            for todo in todos.values() {
                toggle_todo(todo)
            }
        })
    } else {
        active_todos().update(|todos| {
            for todo in todos.values() {
                toggle_todo(todo)
            }
        })
    }
}

#[Cache]
fn todos_count() -> Cache<usize> {
    let todos = watch(todos());

    use_cache(todos.changed(), || {
        todos.map(BTreeMap::len)
    })
}

#[Cache]
fn todos_exist() -> Cache<bool> {
    let todos_count = watch(todos_count());

    use_cache(todos_count.changed(), || {
        todos_count().inner() != 0
    })
}

// -- completed --

#[Cache]
fn completed_todos() -> Cache<Todos> {
    let todos = watch(todos);

    use_cache(todos.changed(), || {
        todos.map(|todos| {
            todos
                .iter()
                .filter(|_, todo| todo.completed)
                .collect()
        })
    })
}

#[Cache]
fn completed_count() -> Cache<usize> {
    let todos = watch(completed_todos());

    use_cache(completed_count.changed(), || {
        completed_count.map(BTreeMap::len)
    })
}

#[Cache]
fn completed_exist() -> Cache<bool> {
    let todos_count = watch(completed_count());

    use_cache(todos_count.changed(), || {
        todos_count().inner() != 0
    })
}

#[Cache]
fn are_all_completed() -> Cache<bool> {
    let total_count = watch(todos_count());
    let completed_count = watch(completed_count());

    use_cache(total_count.changed() || completed_count.changed(), || {
        total_count.inner() == completed_count.inner()
    })
}

// -- active --

#[Cache]
fn active_todos() -> Cache<Todos> {
    let todos = watch(todos());

    use_cache(todos.changed(), || {
        todos.map(|todos| {
            todos
                .iter()
                .filter(|_, todo| !todo.completed)
                .collect()
        })
    })
}

#[Cache]
fn active_count() -> Cache<usize> {
    let todos = watch(active_todos());

    use_cache(todos.changed(), || {
        todos.map(BTreeMap::len)
    })
}

// -- filtered --

#[Cache]
fn filtered_todos() -> Cache<Todos> {
    let selected_filter = watch(selected_filter());

    use_cache(selected_filter.changed(), || {
        match selected_filter.inner() {
            Filter::All => todos(),
            Filter::Active => active_todos(),
            Filter::Completed => completed_todos(),
        }
    })
}
